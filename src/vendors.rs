//! PCI vendor definitions and utilities.

use crate::types::{VendorId, DeviceId};
use crate::devices::Device;

/// Represents a PCI vendor.
#[derive(Debug, Clone)]
pub struct Vendor {
    /// The vendor ID
    pub id: VendorId,
    /// The vendor name
    pub name: &'static str,
    /// The devices manufactured by this vendor
    pub devices: &'static [Device],
}

impl Vendor {
    /// Create a new vendor.
    #[inline]
    pub const fn new(id: VendorId, name: &'static str, devices: &'static [Device]) -> Self {
        Self { id, name, devices }
    }

    /// Get the vendor ID.
    #[inline]
    pub const fn id(&self) -> VendorId {
        self.id
    }

    /// Get the vendor name.
    #[inline]
    pub const fn name(&self) -> &'static str {
        self.name
    }

    /// Get all devices from this vendor.
    #[inline]
    pub const fn devices(&self) -> &'static [Device] {
        self.devices
    }

    /// Find a specific device by ID.
    pub fn find_device(&self, device_id: DeviceId) -> Option<&Device> {
        self.devices.iter().find(|device| device.id() == device_id)
    }

    /// Get the number of devices from this vendor.
    #[inline]
    pub const fn device_count(&self) -> usize {
        self.devices.len()
    }

    /// Check if this vendor manufactures a specific device.
    pub fn has_device(&self, device_id: DeviceId) -> bool {
        self.find_device(device_id).is_some()
    }

    /// Iterate over all devices from this vendor.
    pub fn iter_devices(&self) -> core::slice::Iter<'_, Device> {
        self.devices.iter()
    }
}

impl PartialEq for Vendor {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Vendor {}

impl PartialOrd for Vendor {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Vendor {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.id.cmp(&other.id)
    }
}

/// Well-known vendor IDs for convenience.
pub mod well_known {
    use super::VendorId;

    /// Intel Corporation
    pub const INTEL: VendorId = VendorId::new(0x8086);

    /// Advanced Micro Devices (AMD)
    pub const AMD: VendorId = VendorId::new(0x1022);

    /// NVIDIA Corporation
    pub const NVIDIA: VendorId = VendorId::new(0x10de);

    /// Broadcom
    pub const BROADCOM: VendorId = VendorId::new(0x14e4);

    /// Realtek Semiconductor
    pub const REALTEK: VendorId = VendorId::new(0x10ec);

    /// Qualcomm
    pub const QUALCOMM: VendorId = VendorId::new(0x17cb);

    /// Marvell Technology Group
    pub const MARVELL: VendorId = VendorId::new(0x11ab);

    /// VIA Technologies
    pub const VIA: VendorId = VendorId::new(0x1106);

    /// Atheros Communications
    pub const ATHEROS: VendorId = VendorId::new(0x168c);

    /// 3Com Corporation
    pub const THREECOM: VendorId = VendorId::new(0x10b7);
}