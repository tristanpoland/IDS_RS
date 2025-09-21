//! Error types for PCI database operations.

use core::fmt;

/// Errors that can occur during PCI database parsing or querying.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PciError {
    /// Invalid format in the PCI IDs file
    InvalidFormat,
    /// Invalid hexadecimal value
    InvalidHexValue,
    /// Invalid indentation level
    InvalidIndentation,
    /// Unexpected end of input
    UnexpectedEndOfInput,
    /// Vendor ID not found
    VendorNotFound,
    /// Device ID not found
    DeviceNotFound,
    /// Device class not found
    ClassNotFound,
    /// Subclass not found
    SubclassNotFound,
    /// Programming interface not found
    ProgInterfaceNotFound,
}

impl fmt::Display for PciError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PciError::InvalidFormat => write!(f, "Invalid format in PCI IDs file"),
            PciError::InvalidHexValue => write!(f, "Invalid hexadecimal value"),
            PciError::InvalidIndentation => write!(f, "Invalid indentation level"),
            PciError::UnexpectedEndOfInput => write!(f, "Unexpected end of input"),
            PciError::VendorNotFound => write!(f, "Vendor ID not found"),
            PciError::DeviceNotFound => write!(f, "Device ID not found"),
            PciError::ClassNotFound => write!(f, "Device class not found"),
            PciError::SubclassNotFound => write!(f, "Subclass not found"),
            PciError::ProgInterfaceNotFound => write!(f, "Programming interface not found"),
        }
    }
}

/// Result type for PCI database operations.
pub type PciResult<T> = Result<T, PciError>;