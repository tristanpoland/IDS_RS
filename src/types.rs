//! Type-safe wrappers for PCI identifiers.

use core::fmt;
use core::fmt::Write;

/// A type-safe wrapper for PCI vendor IDs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VendorId(u16);

impl VendorId {
    /// Create a new vendor ID.
    #[inline]
    pub const fn new(id: u16) -> Self {
        Self(id)
    }

    /// Get the raw vendor ID value.
    #[inline]
    pub const fn value(self) -> u16 {
        self.0
    }

    /// Convert to a 4-character hexadecimal string.
    pub fn to_hex_string(self) -> heapless::String<4> {
        let mut s = heapless::String::new();
        let _ = write!(&mut s, "{:04x}", self.0);
        s
    }
}

impl fmt::Display for VendorId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:04x}", self.0)
    }
}

impl fmt::LowerHex for VendorId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:04x}", self.0)
    }
}

impl fmt::UpperHex for VendorId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:04X}", self.0)
    }
}

impl From<u16> for VendorId {
    fn from(id: u16) -> Self {
        Self::new(id)
    }
}

impl From<VendorId> for u16 {
    fn from(id: VendorId) -> Self {
        id.value()
    }
}

/// A type-safe wrapper for PCI device IDs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DeviceId(u16);

impl DeviceId {
    /// Create a new device ID.
    #[inline]
    pub const fn new(id: u16) -> Self {
        Self(id)
    }

    /// Get the raw device ID value.
    #[inline]
    pub const fn value(self) -> u16 {
        self.0
    }

    /// Convert to a 4-character hexadecimal string.
    pub fn to_hex_string(self) -> heapless::String<4> {
        let mut s = heapless::String::new();
        let _ = write!(&mut s, "{:04x}", self.0);
        s
    }
}

impl fmt::Display for DeviceId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:04x}", self.0)
    }
}

impl fmt::LowerHex for DeviceId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:04x}", self.0)
    }
}

impl fmt::UpperHex for DeviceId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:04X}", self.0)
    }
}

impl From<u16> for DeviceId {
    fn from(id: u16) -> Self {
        Self::new(id)
    }
}

impl From<DeviceId> for u16 {
    fn from(id: DeviceId) -> Self {
        id.value()
    }
}

/// A type-safe wrapper for PCI subvendor IDs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SubvendorId(u16);

impl SubvendorId {
    /// Create a new subvendor ID.
    #[inline]
    pub const fn new(id: u16) -> Self {
        Self(id)
    }

    /// Get the raw subvendor ID value.
    #[inline]
    pub const fn value(self) -> u16 {
        self.0
    }
}

impl fmt::Display for SubvendorId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:04x}", self.0)
    }
}

impl From<u16> for SubvendorId {
    fn from(id: u16) -> Self {
        Self::new(id)
    }
}

impl From<SubvendorId> for u16 {
    fn from(id: SubvendorId) -> Self {
        id.value()
    }
}

/// A type-safe wrapper for PCI subdevice IDs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SubdeviceId(u16);

impl SubdeviceId {
    /// Create a new subdevice ID.
    #[inline]
    pub const fn new(id: u16) -> Self {
        Self(id)
    }

    /// Get the raw subdevice ID value.
    #[inline]
    pub const fn value(self) -> u16 {
        self.0
    }
}

impl fmt::Display for SubdeviceId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:04x}", self.0)
    }
}

impl From<u16> for SubdeviceId {
    fn from(id: u16) -> Self {
        Self::new(id)
    }
}

impl From<SubdeviceId> for u16 {
    fn from(id: SubdeviceId) -> Self {
        id.value()
    }
}

/// A type-safe wrapper for PCI device class IDs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DeviceClassId(u8);

impl DeviceClassId {
    /// Create a new device class ID.
    #[inline]
    pub const fn new(id: u8) -> Self {
        Self(id)
    }

    /// Get the raw device class ID value.
    #[inline]
    pub const fn value(self) -> u8 {
        self.0
    }
}

impl fmt::Display for DeviceClassId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02x}", self.0)
    }
}

impl From<u8> for DeviceClassId {
    fn from(id: u8) -> Self {
        Self::new(id)
    }
}

impl From<DeviceClassId> for u8 {
    fn from(id: DeviceClassId) -> Self {
        id.value()
    }
}

/// A type-safe wrapper for PCI subclass IDs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SubClassId(u8);

impl SubClassId {
    /// Create a new subclass ID.
    #[inline]
    pub const fn new(id: u8) -> Self {
        Self(id)
    }

    /// Get the raw subclass ID value.
    #[inline]
    pub const fn value(self) -> u8 {
        self.0
    }
}

impl fmt::Display for SubClassId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02x}", self.0)
    }
}

impl From<u8> for SubClassId {
    fn from(id: u8) -> Self {
        Self::new(id)
    }
}

impl From<SubClassId> for u8 {
    fn from(id: SubClassId) -> Self {
        id.value()
    }
}

/// A type-safe wrapper for PCI programming interface IDs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ProgInterfaceId(u8);

impl ProgInterfaceId {
    /// Create a new programming interface ID.
    #[inline]
    pub const fn new(id: u8) -> Self {
        Self(id)
    }

    /// Get the raw programming interface ID value.
    #[inline]
    pub const fn value(self) -> u8 {
        self.0
    }
}

impl fmt::Display for ProgInterfaceId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02x}", self.0)
    }
}

impl From<u8> for ProgInterfaceId {
    fn from(id: u8) -> Self {
        Self::new(id)
    }
}

impl From<ProgInterfaceId> for u8 {
    fn from(id: ProgInterfaceId) -> Self {
        id.value()
    }
}