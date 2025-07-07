# decentralized-storage

A decentralized storage system built with Rust, where files are encrypted, split into chunks, and distributed across multiple nodes. File metadata, such as chunk locations and hashes, is stored on a blockchain for integrity verification. The system exposes an API built using Actix-web, with async operations handled by Tokio.

## Features

- End-to-end encryption of files
- Chunk-based file architecture
- Distribution across multiple storage nodes
- Metadata stored on-chain for integrity and traceability
- Actix-web based API with Tokio async runtime
- Optional incentive mechanism for storage nodes

## Solana Integration

The system integrates with the Solana blockchain to:

- Store file metadata such as chunk hashes and ownership
- Leverage Solana’s low transaction fees and fast finality
- Interact with smart contracts written in Rust
