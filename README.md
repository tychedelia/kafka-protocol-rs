# Kafka-Protocol

Rust implementation of the [Kafka wire protocol](https://kafka.apache.org/protocol.html), originally implemented by [Diggsey](https://github.com/Diggsey) in a minimal Kafka client implementation [Franz](https://github.com/Diggsey/franz).

Unlike other Kafka protocol implementations, this project uses code generation to cover the entire Kafka API surface, including different protocol versions. See
[the messages directory](./messages) for an example of protocol schema.