# Changelog

## v0.50

Clarify versioning support for upstream Kafka, tracking the latest Kafka
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