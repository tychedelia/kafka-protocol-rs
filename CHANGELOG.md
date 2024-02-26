# Changelog

## v0.9.0

- Remove the `string` crate.
- Update `derive-builder`.

## v0.8.2
- Expose protocol::buf::gap module.

## v0.8.1
- Re-export `indexmap` as part of the public api.

## v0.8.0
- Implement Zstd and Lz4 compression.

## v0.7.0

- Switch to [crc32c](https://crates.io/crates/crc32c) crate, providing hardware accelration for crc operations
on supported platforms.
- Formatting fixes.
- Miscellaneous dependency updates.
- Fix issue with `derive_builder` implementation in which default fields were not being used correctly.

## v0.6.1

- Miscellaneous dependency updates.

## v0.6.0

- Add `ApiKey::request_header_version` and `ApiKey::response_header_version` to assist deserializing
headers without reference to message type.

## v0.5.1

Bump `uuid` crate version.

## v0.5.0

- Clarify versioning support for upstream Kafka, tracking the latest Kafka
stable release. Improve docs.

#### Enhancements

- `Builder::builder` trait for retrieving builder instances from messages
without requiring extra builder imports.
- `#[non_exhaustive]` added to all items to ensure forward compatability
with protocol upgrades.

## v0.4.0

- Add utilities for dealing with response error codes.

## v0.3.0 

#### Enhancements:

- Build messages from crate root.
- Upgrade to latest version of Kafka protocol.


## v0.2.0 

#### Enhancements:

- Add derive builder for all messages.

## v0.1.0

Initial release.