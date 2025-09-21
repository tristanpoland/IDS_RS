#![no_std]
#![deny(missing_docs)]
#![doc = include_str!("../README.md")]

//! # IDS_RS: A no_std PCI Device Identification Library
//!
//! This crate provides comprehensive PCI device identification capabilities for operating systems
//! and low-level system software. It parses the PCI IDs database at compile time, enabling
//! efficient runtime lookups without heap allocation or file I/O.
//!
//! ## Features
//!
//! - **no_std compatible**: Perfect for kernel-space and embedded use
//! - **Compile-time parsing**: Database is parsed during build, not runtime
//! - **Comprehensive coverage**: Supports vendors, devices, subsystems, and device classes
//! - **Zero-cost abstractions**: Efficient static data structures
//! - **Type-safe queries**: Strongly typed IDs prevent common mistakes
//!
//! ## Quick Start
//!
//! ```rust
//! use ids_rs::{PciDatabase, VendorId, DeviceId};
//!
//! // Get the compiled database
//! let db = PciDatabase::get();
//!
//! // Look up a vendor
//! let vendor_id = VendorId::new(0x8086); // Intel
//! if let Some(vendor) = db.find_vendor(vendor_id) {
//!     println!("Vendor: {}", vendor.name());
//! }
//!
//! // Look up a specific device
//! let device_id = DeviceId::new(0x1234);
//! if let Some(device) = db.find_device(vendor_id, device_id) {
//!     println!("Device: {}", device.name());
//! }
//! ```

extern crate alloc;

pub mod error;
pub mod types;
pub mod vendors;
pub mod devices;
pub mod classes;
pub mod parser;
pub mod database;
pub mod query;

pub use error::*;
pub use types::*;
pub use database::PciDatabase;
pub use query::*;

// Re-export commonly used types
pub use vendors::Vendor;
pub use devices::{Device, Subsystem};
pub use classes::{DeviceClass, SubClass, ProgInterface};