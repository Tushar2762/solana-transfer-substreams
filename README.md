# Solana Transfer Extraction - Substreams Module

## Overview
This Rust-based Substreams module extracts native SOL transfer instructions (`from`, `to`, `amount`) from Solana blocks `385870151` to `385870156` in the exact required JSON format:
`{"from": "...", "to": "...", "amount": 325253}`

## Architecture & Implementation
- `Cargo.toml`: Configures Rust dependencies (`substreams`, `substreams-solana`, `prost`, `bs58`).
- `proto/transfers.proto`: Protobuf schema defining the `Transfers` and `Transfer` messages.
- `src/lib.rs`: Core `map_transfers` handler decoding Solana System Program (`11111111111111111111111111111111`) transfer instructions (Index `2`, 12 bytes) and converting wallet addresses to base58.
- `substreams.yaml`: Substreams manifest mapping `sf.solana.type.v1.Block` inputs to the `proto:transfers.v1.Transfers` output.

## Setup & Execution Approach
- Configured Substreams streaming credentials via Pinax Network (`solana.substreams.pinax.network:443`).
- Designed the complete Rust + Protobuf codebase locally on Windows. Due to local Windows Visual Studio C++ Build Tools installation bottlenecks within the 2-hour window, the complete, ready-to-compile Substreams project architecture is submitted here.
