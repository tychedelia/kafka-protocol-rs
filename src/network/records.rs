use bytes::Bytes;
use indexmap::IndexMap;
use log::{info, error};
use crc::crc32;
use string::TryFrom;

use protocol_base::{Encoder, Decoder, EncodeError, DecodeError, StrBytes, types, buf::{ByteBuf, ByteBufMut, gap}};

use super::compression::{self as cmpr, Compressor, Decompressor};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Compression {
    None = 0,
    Gzip = 1,
    Snappy = 2,
    Lz4 = 3,
    Zstd = 4,
}

#[derive(Debug, Copy, Clone)]
pub enum TimestampType {
    Creation = 0,
    LogAppend = 1,
}

pub struct RecordEncodeOptions {
    version: i8,
    compression: Compression,
}

pub const NO_PRODUCER_ID: i64 = -1;
pub const NO_PRODUCER_EPOCH: i16 = -1;
pub const NO_PARTITION_LEADER_EPOCH: i32 = -1;
pub const NO_SEQUENCE: i32 = -1;
pub const NO_TIMESTAMP: i64 = -1;

#[derive(Debug, Clone)]
pub struct RecordBatchEncoder;

#[derive(Debug, Clone)]
pub struct RecordBatchDecoder;

struct BatchDecodeInfo {
    record_count: usize,
    timestamp_type: TimestampType,
    min_offset: i64,
    min_timestamp: i64,
    base_sequence: i32,
    transactional: bool,
    control: bool,
    partition_leader_epoch: i32,
    producer_id: i64,
    producer_epoch: i16,
}

#[derive(Debug, Clone)]
pub struct Record {
    // Batch properties
    pub transactional: bool,
    pub control: bool,
    pub partition_leader_epoch: i32,
    pub producer_id: i64,
    pub producer_epoch: i16,

    // Record properties
    pub timestamp_type: TimestampType,
    pub offset: i64,
    pub sequence: i32,
    pub timestamp: i64,
    pub key: Option<Bytes>,
    pub value: Option<Bytes>,
    pub headers: IndexMap<StrBytes, Option<Bytes>>,
}

const MAGIC_BYTE_OFFSET: usize = 16;

impl RecordBatchEncoder {
    pub fn encode<'a, B, I>(
        buf: &mut B,
        records: I,
        options: &RecordEncodeOptions
    ) -> Result<(), EncodeError>
    where
        B: ByteBufMut,
        I: Iterator<Item=&'a Record> + Clone,
    {
        match options.version {
            0..=1 => Self::encode_legacy(buf, records, options),
            2 => Self::encode_new(buf, records, options),
            _ => panic!("Unknown record batch version"),
        }
    }
    fn encode_legacy_records<'a, B, I>(
        buf: &mut B,
        records: I,
        options: &RecordEncodeOptions
    ) -> Result<(), EncodeError>
    where
        B: ByteBufMut,
        I: Iterator<Item=&'a Record> + Clone,
    {
        for record in records {
            record.encode_legacy(buf, options)?;
        }
        Ok(())
    }
    fn encode_legacy<'a, B, I>(
        buf: &mut B,
        records: I,
        options: &RecordEncodeOptions
    ) -> Result<(), EncodeError>
    where
        B: ByteBufMut,
        I: Iterator<Item=&'a Record> + Clone,
    {
        if options.compression == Compression::None {
            // No wrapper needed
            Self::encode_legacy_records(buf, records, options)?;
        } else {
            // Need a "wrapper" message
            let inner_opts = RecordEncodeOptions {
                compression: Compression::None,
                version: options.version,
            };

            Record::encode_legacy_static(buf, options, |buf| {
                // Timestamp
                if options.version > 0 {
                    let min_timestamp = records.clone().map(|r| r.timestamp).min().unwrap_or_default();
                    types::Int64.encode(buf, min_timestamp)?;
                };

                // Key
                buf.put_i32(-1);

                // Value (Compressed MessageSet)
                let size_gap = buf.put_typed_gap(gap::I32);
                let value_start = buf.offset();
                match options.compression {
                    Compression::Snappy => cmpr::Snappy::compress(buf, |buf| Self::encode_legacy_records(buf, records, &inner_opts))?,
                    Compression::Gzip => cmpr::Gzip::compress(buf, |buf| Self::encode_legacy_records(buf, records, &inner_opts))?,
                    _ => unimplemented!(),
                }

                let value_end = buf.offset();
                let value_size = value_end - value_start;
                if value_size > std::i32::MAX as usize {
                    error!("Record batch was too large to encode ({} bytes)", value_size);
                    return Err(EncodeError);
                }
                buf.fill_typed_gap(size_gap, value_size as i32);

                Ok(())
            })?;
        }
        Ok(())
    }
    
    fn encode_new_records<'a, B, I>(
        buf: &mut B,
        records: I,
        min_offset: i64,
        min_timestamp: i64,
        options: &RecordEncodeOptions
    ) -> Result<(), EncodeError>
    where
        B: ByteBufMut,
        I: Iterator<Item=&'a Record>,
    {
        for record in records {
            record.encode_new(buf, min_offset, min_timestamp, options)?;
        }
        Ok(())
    }

    fn encode_new_batch<'a, B, I>(
        buf: &mut B,
        records: &mut I,
        options: &RecordEncodeOptions
    ) -> Result<bool, EncodeError>
    where
        B: ByteBufMut,
        I: Iterator<Item=&'a Record> + Clone,
    {
        let mut record_peeker = records.clone();

        // Get first record
        let first_record = match record_peeker.next() {
            Some(record) => record,
            None => return Ok(false),
        };

        // Determine how many additional records can be included in the batch
        let num_records = record_peeker.take_while(|record| {
            record.transactional == first_record.transactional &&
            record.control == first_record.control &&
            record.partition_leader_epoch == first_record.partition_leader_epoch &&
            record.producer_id == first_record.producer_id &&
            record.producer_epoch == first_record.producer_epoch &&
            (record.offset as i32).wrapping_sub(record.sequence) == (first_record.offset as i32).wrapping_sub(first_record.sequence)
        }).count() + 1;

        // Aggregate various record properties
        let min_offset = records.clone().take(num_records).map(|r| r.offset).min().expect("Batch contains at least one element");
        let max_offset = records.clone().take(num_records).map(|r| r.offset).max().expect("Batch contains at least one element");
        let min_timestamp = records.clone().take(num_records).map(|r| r.timestamp).min().expect("Batch contains at least one element");
        let max_timestamp = records.clone().take(num_records).map(|r| r.timestamp).max().expect("Batch contains at least one element");
        let base_sequence = first_record.sequence.wrapping_sub((first_record.offset - min_offset) as i32);

        // Base offset
        types::Int64.encode(buf, min_offset)?;

        // Batch length
        let size_gap = buf.put_typed_gap(gap::I32);
        let batch_start = buf.offset();

        // Partition leader epoch
        types::Int32.encode(buf, first_record.partition_leader_epoch)?;

        // Magic byte
        types::Int8.encode(buf, options.version)?;

        // CRC
        let crc_gap = buf.put_typed_gap(gap::U32);
        let content_start = buf.offset();

        // Attributes
        let mut attributes = options.compression as i16;
        if first_record.transactional {
            attributes |= 1 << 4;
        }
        if first_record.control {
            attributes |= 1 << 5;
        }
        types::Int16.encode(buf, attributes)?;

        // Last offset delta
        types::Int32.encode(buf, (max_offset - min_offset) as i32)?;

        // First timestamp
        types::Int64.encode(buf, min_timestamp)?;

        // Last timestamp
        types::Int64.encode(buf, max_timestamp)?;

        // Producer ID
        types::Int64.encode(buf, first_record.producer_id)?;

        // Producer epoch
        types::Int16.encode(buf, first_record.producer_epoch)?;

        // Base sequence
        types::Int32.encode(buf, base_sequence)?;

        // Record count
        if num_records > std::i32::MAX as usize {
            error!("Too many records to encode in one batch ({} records)", num_records);
            return Err(EncodeError);
        }
        types::Int32.encode(buf, num_records as i32)?;

        // Records
        let records = records.take(num_records);
        match options.compression {
            Compression::None => cmpr::None::compress(buf, |buf| Self::encode_new_records(buf, records, min_offset, min_timestamp, options))?,
            Compression::Snappy => cmpr::Snappy::compress(buf, |buf| Self::encode_new_records(buf, records, min_offset, min_timestamp, options))?,
            Compression::Gzip => cmpr::Gzip::compress(buf, |buf| Self::encode_new_records(buf, records, min_offset, min_timestamp, options))?,
            _ => unimplemented!(),
        }

        let batch_end = buf.offset();

        // Fill size gap
        let batch_size = batch_end - batch_start;
        if batch_size > std::i32::MAX as usize {
            error!("Record batch was too large to encode ({} bytes)", batch_size);
            return Err(EncodeError);
        }

        buf.fill_typed_gap(size_gap, batch_size as i32);

        // Fill CRC gap
        let crc = crc32::checksum_castagnoli(buf.range(content_start..batch_end));
        buf.fill_typed_gap(crc_gap, crc);

        Ok(true)
    }

    fn encode_new<'a, B, I>(
        buf: &mut B,
        mut records: I,
        options: &RecordEncodeOptions
    ) -> Result<(), EncodeError>
    where
        B: ByteBufMut,
        I: Iterator<Item=&'a Record> + Clone,
    {
        while Self::encode_new_batch(buf, &mut records, options)? {}
        Ok(())
    }
}

impl RecordBatchDecoder {
    pub fn decode<B: ByteBuf>(buf: &mut B) -> Result<Vec<Record>, DecodeError> {
        let mut records = Vec::new();
        while buf.has_remaining() {
            Self::decode_batch(buf, &mut records)?;
        }
        Ok(records)
    }
    fn decode_batch<B: ByteBuf>(buf: &mut B, records: &mut Vec<Record>) -> Result<(), DecodeError> {
        let version = buf.try_peek_bytes(MAGIC_BYTE_OFFSET..(MAGIC_BYTE_OFFSET+1))?[0] as i8;
        info!("Decoding record batch (version: {})", version);
        match version {
            0..=1 => Record::decode_legacy(buf, version, records),
            2 => Self::decode_new_batch(buf, version, records),
            _ => {
                error!("Unknown record batch version ({})", version);
                Err(DecodeError)
            },
        }
    }
    fn decode_new_records<B: ByteBuf>(
        buf: &mut B,
        batch_decode_info: &BatchDecodeInfo,
        version: i8,
        records: &mut Vec<Record>,
    ) -> Result<(), DecodeError> {
        records.reserve(batch_decode_info.record_count);
        for _ in 0..batch_decode_info.record_count {
            records.push(Record::decode_new(buf, batch_decode_info, version)?);
        }
        Ok(())
    }
    fn decode_new_batch<B: ByteBuf>(buf: &mut B, version: i8, records: &mut Vec<Record>) -> Result<(), DecodeError> {
        // Base offset
        let min_offset = types::Int64.decode(buf)?;

        // Batch length
        let batch_length: i32 = types::Int32.decode(buf)?;
        if batch_length < 0 {
            error!("Unexpected negative batch size: {}", batch_length);
            return Err(DecodeError);
        }

        // Convert buf to bytes
        let buf = &mut buf.try_get_bytes(batch_length as usize)?;

        // Partition leader epoch
        let partition_leader_epoch = types::Int32.decode(buf)?;

        // Magic byte
        let magic: i8 = types::Int8.decode(buf)?;
        if magic != version {
            error!("Version mismtach ({} != {})", magic, version);
            return Err(DecodeError);
        }

        // CRC
        let supplied_crc: u32 = types::UInt32.decode(buf)?;
        let actual_crc = crc32::checksum_castagnoli(&buf);
        
        if supplied_crc != actual_crc {
            error!("Cyclic redundancy check failed ({} != {})", supplied_crc, actual_crc);
            return Err(DecodeError);
        }

        // Attributes
        let attributes: i16 = types::Int16.decode(buf)?;
        let transactional = (attributes & (1 << 4)) != 0;
        let control = (attributes & (1 << 5)) != 0;
        let compression = match attributes & 0x7 {
            0 => Compression::None,
            1 => Compression::Gzip,
            2 => Compression::Snappy,
            3 => Compression::Lz4,
            4 => Compression::Zstd,
            other => {
                error!("Unknown compression algorithm used: {}", other);
                return Err(DecodeError);
            }
        };
        let timestamp_type = if (attributes & (1 << 3)) != 0 {
            TimestampType::LogAppend
        } else {
            TimestampType::Creation
        };

        // Last offset delta
        let _max_offset_delta: i32 = types::Int32.decode(buf)?;

        // First timestamp
        let min_timestamp = types::Int64.decode(buf)?;

        // Last timestamp
        let _max_timestamp: i64 = types::Int64.decode(buf)?;

        // Producer ID
        let producer_id = types::Int64.decode(buf)?;

        // Producer epoch
        let producer_epoch = types::Int16.decode(buf)?;

        // Base sequence
        let base_sequence = types::Int32.decode(buf)?;

        // Record count
        let record_count: i32 = types::Int32.decode(buf)?;
        if record_count < 0 {
            error!("Unexpected negative record count ({})", record_count);
            return Err(DecodeError);
        }
        let record_count = record_count as usize;

        let batch_decode_info = BatchDecodeInfo {
            record_count,
            timestamp_type,
            min_offset,
            min_timestamp,
            base_sequence,
            transactional,
            control,
            partition_leader_epoch,
            producer_id,
            producer_epoch,
        };

        // Records
        match compression {
            Compression::None => cmpr::None::decompress(buf, |buf| Self::decode_new_records(buf, &batch_decode_info, version, records))?,
            Compression::Snappy => cmpr::Snappy::decompress(buf, |buf| Self::decode_new_records(buf, &batch_decode_info, version, records))?,
            Compression::Gzip => cmpr::Gzip::decompress(buf, |buf| Self::decode_new_records(buf, &batch_decode_info, version, records))?,
            _ => unimplemented!(),
        };

        Ok(())
    }
}

impl Record {
    fn encode_legacy_static<B, F>(buf: &mut B, options: &RecordEncodeOptions, content_writer: F) -> Result<(), EncodeError>
    where
        B: ByteBufMut,
        F: FnOnce(&mut B) -> Result<(), EncodeError>
    {
        types::Int64.encode(buf, 0)?;
        let size_gap = buf.put_typed_gap(gap::I32);
        let message_start = buf.offset();
        let crc_gap = buf.put_typed_gap(gap::U32);
        let content_start = buf.offset();

        types::Int8.encode(buf, options.version)?;

        let compression = options.compression as i8;
        if compression > 2 + options.version {
            error!("Compression algorithm '{:?}' is unsupported for record version '{}'", options.compression, options.version);
            return Err(EncodeError);
        }
        types::Int8.encode(buf, compression)?;

        // Write content
        content_writer(buf)?;

        let message_end = buf.offset();

        let message_size = message_end - message_start;
        if message_start > std::i32::MAX as usize {
            error!("Record was too large to encode ({} bytes)", message_size);
            return Err(EncodeError);
        }
        buf.fill_typed_gap(size_gap, message_size as i32);

        let crc = crc32::checksum_ieee(buf.range(content_start..message_end));
        buf.fill_typed_gap(crc_gap, crc);

        Ok(())
    }
    fn encode_legacy<B: ByteBufMut>(&self, buf: &mut B, options: &RecordEncodeOptions) -> Result<(), EncodeError> {
        if self.transactional || self.control {
            error!("Transactional and control records are not supported in this version of the protocol!");
            return Err(EncodeError);
        }

        if !self.headers.is_empty() {
            error!("Record headers are not supported in this version of the protocol!");
            return Err(EncodeError);
        }

        Self::encode_legacy_static(buf, options, |buf| {
            if options.version > 0 {
                types::Int64.encode(buf, self.timestamp)?;
            }
            types::Bytes.encode(buf, &self.key)?;
            types::Bytes.encode(buf, &self.value)?;

            Ok(())
        })
    }
    fn encode_new<B: ByteBufMut>(&self, buf: &mut B, min_offset: i64, min_timestamp: i64, options: &RecordEncodeOptions) -> Result<(), EncodeError> {
        // Size
        let size = self.compute_size_new(min_offset, min_timestamp, options)?;
        if size > std::i32::MAX as usize {
            error!("Record was too large to encode ({} bytes)", size);
            return Err(EncodeError);
        }
        types::VarInt.encode(buf, size as i32)?;

        // Attributes
        types::Int8.encode(buf, 0)?;

        // Timestamp delta
        let timestamp_delta = self.timestamp - min_timestamp;
        if timestamp_delta > std::i32::MAX as i64 || timestamp_delta < std::i32::MIN as i64 {
            error!("Timestamps within batch are too far apart ({}, {})", min_timestamp, self.timestamp);
            return Err(EncodeError);
        }
        types::VarInt.encode(buf, timestamp_delta as i32)?;

        // Offset delta
        let offset_delta = self.offset - min_offset;
        if offset_delta > std::i32::MAX as i64 || offset_delta < std::i32::MIN as i64 {
            error!("Timestamps within batch are too far apart ({}, {})", min_offset, self.offset);
            return Err(EncodeError);
        }
        types::VarInt.encode(buf, offset_delta as i32)?;

        // Key
        if let Some(k) = self.key.as_ref() {
            if k.len() > std::i32::MAX as usize {
                error!("Record key was too large to encode ({} bytes)", k.len());
                return Err(EncodeError);
            }
            types::VarInt.encode(buf, k.len() as i32)?;
            buf.put_slice(k);
        } else {
            types::VarInt.encode(buf, -1)?;
        }

        // Value
        if let Some(v) = self.value.as_ref() {
            if v.len() > std::i32::MAX as usize {
                error!("Record value was too large to encode ({} bytes)", v.len());
                return Err(EncodeError);
            }
            types::VarInt.encode(buf, v.len() as i32)?;
            buf.put_slice(v);
        } else {
            types::VarInt.encode(buf, -1)?;
        }

        // Headers
        if self.headers.len() > std::i32::MAX as usize {
            error!("Too many record headers encode ({})", self.headers.len());
            return Err(EncodeError);
        }
        types::VarInt.encode(buf, self.headers.len() as i32)?;
        for (k, v) in &self.headers {
            // Key len
            if k.len() > std::i32::MAX as usize {
                error!("Record header key was too large to encode ({} bytes)", k.len());
                return Err(EncodeError);
            }
            types::VarInt.encode(buf, k.len() as i32)?;

            // Key
            buf.put_slice(k.as_ref());

            // Value
            if let Some(v) = v.as_ref() {
                if v.len() > std::i32::MAX as usize {
                    error!("Record header value was too large to encode ({} bytes)", v.len());
                    return Err(EncodeError);
                }
                types::VarInt.encode(buf, v.len() as i32)?;
                buf.put_slice(v);
            } else {
                types::VarInt.encode(buf, -1)?;
            }
        }

        Ok(())
    }
    fn compute_size_new(&self, min_offset: i64, min_timestamp: i64, _options: &RecordEncodeOptions) -> Result<usize, EncodeError> {
        let mut total_size = 0;

        // Attributes
        total_size += types::Int8.compute_size(0)?;

        // Timestamp delta
        let timestamp_delta = self.timestamp - min_timestamp;
        if timestamp_delta > std::i32::MAX as i64 || timestamp_delta < std::i32::MIN as i64 {
            error!("Timestamps within batch are too far apart ({}, {})", min_timestamp, self.timestamp);
            return Err(EncodeError);
        }
        total_size += types::VarInt.compute_size(timestamp_delta as i32)?;

        // Offset delta
        let offset_delta = self.offset - min_offset;
        if offset_delta > std::i32::MAX as i64 || offset_delta < std::i32::MIN as i64 {
            error!("Timestamps within batch are too far apart ({}, {})", min_offset, self.offset);
            return Err(EncodeError);
        }
        total_size += types::VarInt.compute_size(offset_delta as i32)?;

        // Key
        if let Some(k) = self.key.as_ref() {
            if k.len() > std::i32::MAX as usize {
                error!("Record key was too large to encode ({} bytes)", k.len());
                return Err(EncodeError);
            }
            total_size += types::VarInt.compute_size(k.len() as i32)?;
            total_size += k.len();
        } else {
            total_size += types::VarInt.compute_size(-1)?;
        }

        // Value len
        if let Some(v) = self.value.as_ref() {
            if v.len() > std::i32::MAX as usize {
                error!("Record value was too large to encode ({} bytes)", v.len());
                return Err(EncodeError);
            }
            total_size += types::VarInt.compute_size(v.len() as i32)?;
            total_size += v.len();
        } else {
            total_size += types::VarInt.compute_size(-1)?;
        }

        // Headers
        if self.headers.len() > std::i32::MAX as usize {
            error!("Too many record headers encode ({})", self.headers.len());
            return Err(EncodeError);
        }
        total_size += types::VarInt.compute_size(self.headers.len() as i32)?;
        for (k, v) in &self.headers {
            // Key len
            if k.len() > std::i32::MAX as usize {
                error!("Record header key was too large to encode ({} bytes)", k.len());
                return Err(EncodeError);
            }
            total_size += types::VarInt.compute_size(k.len() as i32)?;

            // Key
            total_size += k.len();

            // Value
            if let Some(v) = v.as_ref() {
                if v.len() > std::i32::MAX as usize {
                    error!("Record header value was too large to encode ({} bytes)", v.len());
                    return Err(EncodeError);
                }
                total_size += types::VarInt.compute_size(v.len() as i32)?;
                total_size += v.len();
            } else {
                total_size += types::VarInt.compute_size(-1)?;
            }
        }

        Ok(total_size)
    }
    fn decode_legacy<B: ByteBuf>(buf: &mut B, version: i8, records: &mut Vec<Record>) -> Result<(), DecodeError> {
        let offset = types::Int64.decode(buf)?;
        let size: i32 = types::Int32.decode(buf)?;
        if size < 0 {
            error!("Unexpected negative record size: {}", size);
            return Err(DecodeError);
        }

        // Ensure we don't over-read
        let buf = &mut buf.try_get_bytes(size as usize)?;

        // CRC
        let supplied_crc: u32 = types::UInt32.decode(buf)?;
        let actual_crc = crc32::checksum_ieee(&buf);
        
        if supplied_crc != actual_crc {
            error!("Cyclic redundancy check failed ({} != {})", supplied_crc, actual_crc);
            return Err(DecodeError);
        }

        // Magic
        let magic: i8 = types::Int8.decode(buf)?;
        if magic != version {
            error!("Version mismtach ({} != {})", magic, version);
            return Err(DecodeError);
        }

        // Attributes
        let attributes: i8 = types::Int8.decode(buf)?;
        let compression = match attributes & 0x7 {
            0 => Compression::None,
            1 => Compression::Gzip,
            2 => Compression::Snappy,
            3 if version > 0 => Compression::Lz4,
            other => {
                error!("Unknown compression algorithm used: {}", other);
                return Err(DecodeError);
            }
        };
        let timestamp_type = if (attributes & (1 << 3)) != 0 {
            TimestampType::LogAppend
        } else {
            TimestampType::Creation
        };

        // Write content
        let timestamp = if version > 0 {
            types::Int64.decode(buf)?
        } else {
            NO_TIMESTAMP
        };
        let key = types::Bytes.decode(buf)?;
        let value = types::Bytes.decode(buf)?;

        if compression == Compression::None {
            // Uncompressed record
            records.push(Record {
                transactional: false,
                control: false,
                partition_leader_epoch: NO_PARTITION_LEADER_EPOCH,
                producer_id: NO_PRODUCER_ID,
                producer_epoch: NO_PRODUCER_EPOCH,
                sequence: NO_SEQUENCE,
                timestamp_type,
                offset,
                timestamp,
                key,
                value,
                headers: Default::default(),
            });
        } else {
            // Wrapper record around a compressed MessageSet
            let mut value = value.ok_or_else(|| {
                error!("Received compressed legacy record without a value");
                DecodeError
            })?;

            while !value.is_empty() {
                Record::decode_legacy(&mut value, version, records)?;
            }
        }

        Ok(())
    }
    fn decode_new<B: ByteBuf>(buf: &mut B, batch_decode_info: &BatchDecodeInfo, _version: i8) -> Result<Self, DecodeError> {
        // Size
        let size: i32 = types::VarInt.decode(buf)?;
        if size < 0 {
            error!("Unexpected negative record size: {}", size);
            return Err(DecodeError);
        }

        // Ensure we don't over-read
        let buf = &mut buf.try_get_bytes(size as usize)?;

        // Attributes
        let _attributes: i8 = types::Int8.decode(buf)?;

        // Timestamp delta
        let timestamp_delta: i32 = types::VarInt.decode(buf)?;
        let timestamp = batch_decode_info.min_timestamp + timestamp_delta as i64;

        // Offset delta
        let offset_delta: i32 = types::VarInt.decode(buf)?;
        let offset = batch_decode_info.min_offset + offset_delta as i64;
        let sequence = batch_decode_info.base_sequence.wrapping_add(offset_delta);

        // Key
        let key_len: i32 = types::VarInt.decode(buf)?;
        let key = if key_len < -1 {
            error!("Unexpected negative record key length ({} bytes)", key_len);
            return Err(DecodeError);
        } else if key_len == -1 {
            None
        } else {
            Some(buf.try_get_bytes(key_len as usize)?)
        };

        // Value
        let value_len: i32 = types::VarInt.decode(buf)?;
        let value = if value_len < -1 {
            error!("Unexpected negative record value length ({} bytes)", value_len);
            return Err(DecodeError);
        } else if value_len == -1 {
            None
        } else {
            Some(buf.try_get_bytes(value_len as usize)?)
        };

        // Headers
        let num_headers: i32 = types::VarInt.decode(buf)?;
        if num_headers < 0 {
            error!("Unexpected negative record header count: {}", num_headers);
            return Err(DecodeError);
        }
        let num_headers = num_headers as usize;

        let mut headers = IndexMap::with_capacity(num_headers);
        for _ in 0..num_headers {
            // Key len
            let key_len: i32 = types::VarInt.decode(buf)?;
            if key_len < 0 {
                error!("Unexpected negative record header key length ({} bytes)", key_len);
                return Err(DecodeError);
            }

            // Key
            let key = StrBytes::try_from(buf.try_get_bytes(key_len as usize)?)?;

            // Key len
            let value_len: i32 = types::VarInt.decode(buf)?;

            // Value
            let value = if value_len < -1 {
                error!("Unexpected negative record header value length ({} bytes)", value_len);
                return Err(DecodeError);
            } else if value_len == -1 {
                None
            } else {
                Some(buf.try_get_bytes(value_len as usize)?)
            };

            headers.insert(key, value);
        }

        Ok(Self {
            transactional: batch_decode_info.transactional,
            control: batch_decode_info.control,
            timestamp_type: batch_decode_info.timestamp_type,
            partition_leader_epoch: batch_decode_info.partition_leader_epoch,
            producer_id: batch_decode_info.producer_id,
            producer_epoch: batch_decode_info.producer_epoch,
            sequence,
            offset,
            timestamp,
            key,
            value,
            headers,
        })
    }
}
