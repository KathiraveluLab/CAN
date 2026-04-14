# CAN Framework

Content-Aware Networking (CAN) framework implementation in Rust.

## Overview
This framework simulates a Content-aware Router (C-Router) architecture implementing the Scalable Content Routing (SCAN) protocol. It prioritizes healthcare data (DICOM) while navigating net neutrality constraints through intelligent, content-sensitive scheduling and routing.

## Key Features
- **Scalable Content Routing (SCAN):** Probabilistic routing using Bloom Filters for efficient neighbor discovery.
- **Persistent Content Caching:** ACID-compliant disk-based storage for the Local Content Table (LCT) using `redb`.
- **Advanced Deep Packet Inspection (DPI):** Granular inspection of DICOM headers (Study UID, Series Description) for prioritized medical data handling.
- **Time Slot Multicast:** Synchronized content distribution for concurrent requests.
- **Recursive Network Search:** Automated discovery across multi-hop network topologies triggered by Bloom Filter matches.

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
The built-in simulation verifies the research parity by executing several scenarios:
```bash
cargo run
```

### Verified Scenarios:
- **Scenario A: Basic Content Routing**: Verifies LCT lookup and SCAN discovery.
- **Scenario B: DICOM Priority**: Demonstrates content-aware packet scheduling for healthcare data.
- **Scenario C: Time Slot Multicast**: Validates synchronized distribution to multiple requesters.
- **Scenario D: Persistent Cache Recovery**: Confirms that indexed content survives router reboots.
- **Scenario E: Recursive Tree Search**: Verifies multi-hop discovery across a complex network topology.

## Testing
To run the framework's test suite:
```bash
cargo test
```

## Technical Architecture
- **Core (`src/core`)**: Content metadata models, Bloom Filter indexing, and persistent table management.
- **Routing (`src/routing`)**: SCAN protocol logic, multicast scheduling, and recursive searching.
- **Apps (`src/apps`)**: Specialized application routers (e.g., healthcare-specific DICOM router).
- **Persistence**: `redb` (Embedded NoSQL) with `rmp-serde` (MessagePack) serialization.
