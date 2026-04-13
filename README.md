# CAN Framework

Content-Aware Networking (CAN) framework implementation in Rust.

## Overview
This framework simulates a Content-aware Router (C-Router) architecture implementing the Scalable Content Routing (SCAN) protocol. It prioritizes healthcare data (DICOM) while navigating net neutrality constraints.

## Prerequisites
- Rust (Edition 2021)
- Cargo

### Installing Rust
To install Rust and Cargo on Unix-like systems (Linux/macOS), run:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
For other platforms, please visit [rust-lang.org](https://www.rust-lang.org/tools/install) for official installation instructions.

## Build
To compile the framework and its dependencies:
```bash
cargo build
```

## Running the Simulation
To run the built-in simulation showing SCAN protocol discovery and DICOM priority:
```bash
cargo run
```

## Testing
To run the unit tests for the C-Router logic:
```bash
cargo test
```

## Architecture
- **Core**: Content metadata, Bloom Filter indexing, and concurrent lookup tables (LCT/CRT).
- **Routing**: SCAN protocol implementation and the C-Router engine.
- **Apps**: Specialized DICOM router for healthcare data.
