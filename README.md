# IDS_RS: A no_std PCI Device Identification Library

A no_std-compatible PCI device identification library designed specifically for operating systems and low-level system software. This crate parses the PCI IDs database at compile time, enabling efficient runtime lookups without heap allocation or file I/O.

## Features

- **🚀 no_std compatible**: Perfect for kernel-space and embedded environments
- **⚡ Compile-time parsing**: Database is parsed during build, not runtime
- **🔍 Comprehensive coverage**: Supports vendors, devices, subsystems, and device classes
- **🎯 Zero-cost abstractions**: Efficient static data structures with no runtime overhead
- **🛡️ Type-safe queries**: Strongly typed IDs prevent common mistakes
- **🔧 Advanced querying**: Flexible query builder for complex lookups
- **📦 Auto-updating**: Includes scripts to update the PCI IDs database

## Quick Start

Add this to your `Cargo.toml`:

```toml
[dependencies]
ids_rs = "0.1"
```

### Basic Usage

```rust
use ids_rs::{PciDatabase, VendorId, DeviceId, DeviceClassId, SubClassId};

// Get the compiled database
let db = PciDatabase::get();

// Look up a vendor (Intel)
let vendor_id = VendorId::new(0x8086);
if let Some(vendor) = db.find_vendor(vendor_id) {
    println!("Vendor: {}", vendor.name());
}

// Look up a specific device
let device_id = DeviceId::new(0x1234);
if let Some(device) = db.find_device(vendor_id, device_id) {
    println!("Device: {}", device.name());
}

// Get a complete device description
let description = db.describe_device(
    vendor_id,
    device_id,
    Some(DeviceClassId::new(0x02)), // Network controller
    Some(SubClassId::new(0x00)),    // Ethernet controller
    None,
    None,
    None,
);
println!("Full description: {}", description);
```

### Advanced Querying

```rust
use ids_rs::{PciDatabase, QueryBuilder};

let db = PciDatabase::get();

// Find all Intel network devices
let intel_network_devices = db.query()
    .vendor_name_contains("Intel")
    .class_name_contains("Network")
    .execute();

for device_match in intel_network_devices {
    println!("{}: {}", device_match.vendor_name(), device_match.device_name());
}

// Search for specific device types
let ethernet_devices = db.search_devices("ethernet");
let wireless_classes = db.search_classes("wireless");
```

### Device Class Lookups

```rust
use ids_rs::{PciDatabase, DeviceClassId, SubClassId, ProgInterfaceId};

let db = PciDatabase::get();

// Look up device class information
let class_id = DeviceClassId::new(0x0c); // Serial bus controller
let subclass_id = SubClassId::new(0x03);  // USB controller
let prog_if_id = ProgInterfaceId::new(0x30); // XHCI

if let Some(prog_if) = db.find_prog_interface(class_id, subclass_id, prog_if_id) {
    println!("Programming interface: {}", prog_if.name());
}
```

## Database Updates

The crate includes scripts to download and update the PCI IDs database:

### PowerShell (Windows)
```powershell
.\update_pci_ids.ps1
```

### Bash (Linux/macOS)
```bash
./update_pci_ids.sh
```

These scripts will:
- Download the latest PCI IDs database from https://pci-ids.ucw.cz/
- Validate the download
- Show database statistics
- Only download if the local file is older than 7 days (use `-Force` to override)

After updating the database, rebuild your project to incorporate the new data:

```bash
cargo clean
cargo build
```

## Architecture

### Modular Design

The crate is organized into focused modules:

- **`types`**: Type-safe wrappers for PCI identifiers
- **`vendors`**: Vendor definitions and utilities
- **`devices`**: Device and subsystem definitions
- **`classes`**: Device class, subclass, and programming interface definitions
- **`database`**: Main database interface and lookups
- **`query`**: Advanced query builder and search functionality
- **`parser`**: PCI IDs format parser (build-time only)
- **`error`**: Error types and handling

### Compile-Time Database Generation

The PCI IDs database is parsed at compile time using a build script. This approach provides:

1. **Zero runtime cost**: No parsing overhead during program execution
2. **Static memory usage**: All data is embedded in the binary
3. **Type safety**: All IDs are validated at compile time
4. **Efficient lookups**: Binary search on sorted arrays

### Memory Layout

The generated database uses efficient memory layouts:

- Vendors are sorted by ID for binary search
- Device classes are sorted by ID for binary search
- Devices within vendors use linear search (typically small arrays)
- All strings are static `&'static str` references

## no_std Compatibility

This crate is fully compatible with `no_std` environments:

```rust,ignore
#![no_std]

use ids_rs::{PciDatabase, VendorId};

fn main() {
    // Works in no_std environments
    let db = PciDatabase::get();
    let vendor = db.find_vendor(VendorId::new(0x8086));
}
```

The only requirement is the `heapless` crate for some string operations in type conversion methods.

## API Reference

### Core Types

- `VendorId`, `DeviceId`: Type-safe PCI vendor and device identifiers
- `SubvendorId`, `SubdeviceId`: Type-safe subsystem identifiers
- `DeviceClassId`, `SubClassId`, `ProgInterfaceId`: Type-safe class identifiers

### Main Structures

- `PciDatabase`: Main database interface
- `Vendor`: PCI vendor information
- `Device`: PCI device information
- `Subsystem`: PCI subsystem information
- `DeviceClass`: PCI device class information

### Query Interface

- `QueryBuilder`: Flexible query builder for complex searches
- `DeviceMatch`: Device search result
- `ClassMatch`: Class search result

## Performance

The library is designed for maximum performance in system-level code:

- **Lookup time**: O(log n) for vendors and classes, O(n) for devices (n typically < 100)
- **Memory usage**: ~500KB-2MB depending on database size (static data)
- **Binary size impact**: Moderate increase due to embedded database
- **Runtime allocations**: None (all data is static)

## Use Cases

This library is ideal for:

- **Operating system kernels**: Device driver loading and hardware identification
- **System monitoring tools**: Hardware inventory and device enumeration
- **Embedded systems**: Hardware discovery in resource-constrained environments
- **Hypervisors**: Virtual device management and PCI passthrough
- **Boot loaders**: Early hardware detection and initialization

## Database Statistics

The current PCI IDs database contains approximately:
- 2,500+ vendors
- 25,000+ devices
- 5,000+ subsystems
- 20+ device classes
- 100+ subclasses
- 200+ programming interfaces

## Contributing

Contributions are welcome! Please:

1. Run the update scripts to get the latest database
2. Add tests for new functionality
3. Ensure `no_std` compatibility
4. Follow the existing code style
5. Update documentation as needed

## License

MIT