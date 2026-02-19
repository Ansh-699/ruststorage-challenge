# Generic Storage - Architecture

## Overview

Generic Storage is a Rust CLI application that stores and retrieves serialized data with efficient slot management and multiple encoding formats.

![Architecture Diagram](./assets/genricstorage.png)

## Key Capabilities

- **Multiple Codecs**: Support for JSON, binary, and protocol buffer serialization
- **Slot Allocation**: Efficient space management with reuse of freed slots
- **Persistence**: File-based storage with transaction-safe writes
- **Metadata Tracking**: Registry maintains location and state information

## Core Components

| Component | Purpose |
|-----------|----------|
| **Engine** | Coordinates data flow and slot allocation |
| **Codec** | Handles serialization/deserialization |
| **Slot Manager** | Tracks available storage locations |
| **Persistence** | File I/O operations |

## User Stories

### Store and Retrieve Data
As a user, I need to persist data and retrieve it later using unique identifiers.
- Store data with automatic encoding
- Retrieve data by ID
- Retrieved data matches original

### Manage Storage Slots
As an operator, I need efficient slot allocation and reuse when data is deleted.
- Deleted slots become available for reuse
- Allocation finds appropriate slots quickly
- Fragmentation is controlled

### Handle Failures Safely
As a user, I need failed operations to not corrupt existing data.
- Failed writes don't corrupt other data
- Registry remains consistent
- Clear error messages on failure
