# Kafka-Protocol [![Build](https://github.com/tychedelia/kafka-protocol-rs/actions/workflows/build-and-test.yml/badge.svg)](https://github.com/0x1991babe/kafka-protocol-rs/actions/workflows/build-and-test.yml) ![crates.io](https://img.shields.io/crates/v/kafka-protocol.svg) ![docs.rs](https://img.shields.io/docsrs/kafka-protocol)

Rust implementation of the [Kafka wire protocol](https://kafka.apache.org/protocol.html).

Unlike other Kafka protocol implementations, this project uses code generation to cover the entire Kafka API surface, 
including different protocol versions. See [Kafka's repo](https://github.com/apache/kafka/blob/trunk/clients/src/main/resources/common/message)
for an example of protocol schema.


Originally implemented by
[@Diggsey](https://github.com/Diggsey) in a minimal Kafka client implementation [Franz](https://github.com/Diggsey/franz)