//! PCI device definitions and utilities.

use crate::types::{DeviceId, SubvendorId, SubdeviceId};

/// Represents a PCI subsystem device.
#[derive(Debug, Clone)]
pub struct Subsystem {
    /// The subvendor ID
    pub subvendor_id: SubvendorId,
    /// The subdevice ID
    pub subdevice_id: SubdeviceId,
    /// The subsystem name
    pub name: &'static str,
}

impl Subsystem {
    /// Create a new subsystem.
    #[inline]
    pub const fn new(subvendor_id: SubvendorId, subdevice_id: SubdeviceId, name: &'static str) -> Self {
        Self {
            subvendor_id,
            subdevice_id,
            name,
        }
    }

    /// Get the subvendor ID.
    #[inline]
    pub const fn subvendor_id(&self) -> SubvendorId {
        self.subvendor_id
    }

    /// Get the subdevice ID.
    #[inline]
    pub const fn subdevice_id(&self) -> SubdeviceId {
        self.subdevice_id
    }

    /// Get the subsystem name.
    #[inline]
    pub const fn name(&self) -> &'static str {
        self.name
    }
}

impl PartialEq for Subsystem {
    fn eq(&self, other: &Self) -> bool {
        self.subvendor_id == other.subvendor_id && self.subdevice_id == other.subdevice_id
    }
}

impl Eq for Subsystem {}

/// Represents a PCI device.
#[derive(Debug, Clone)]
pub struct Device {
    /// The device ID
    pub id: DeviceId,
    /// The device name
    pub name: &'static str,
    /// The subsystems for this device
    pub subsystems: &'static [Subsystem],
}

impl Device {
    /// Create a new device.
    #[inline]
    pub const fn new(id: DeviceId, name: &'static str, subsystems: &'static [Subsystem]) -> Self {
        Self { id, name, subsystems }
    }

    /// Get the device ID.
    #[inline]
    pub const fn id(&self) -> DeviceId {
        self.id
    }

    /// Get the device name.
    #[inline]
    pub const fn name(&self) -> &'static str {
        self.name
    }

    /// Get all subsystems for this device.
    #[inline]
    pub const fn subsystems(&self) -> &'static [Subsystem] {
        self.subsystems
    }

    /// Find a specific subsystem by subvendor and subdevice IDs.
    pub fn find_subsystem(&self, subvendor_id: SubvendorId, subdevice_id: SubdeviceId) -> Option<&Subsystem> {
        self.subsystems.iter().find(|subsystem| {
            subsystem.subvendor_id == subvendor_id && subsystem.subdevice_id == subdevice_id
        })
    }

    /// Get the number of subsystems for this device.
    #[inline]
    pub const fn subsystem_count(&self) -> usize {
        self.subsystems.len()
    }

    /// Check if this device has a specific subsystem.
    pub fn has_subsystem(&self, subvendor_id: SubvendorId, subdevice_id: SubdeviceId) -> bool {
        self.find_subsystem(subvendor_id, subdevice_id).is_some()
    }

    /// Iterate over all subsystems for this device.
    pub fn iter_subsystems(&self) -> core::slice::Iter<'_, Subsystem> {
        self.subsystems.iter()
    }
}

impl PartialEq for Device {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Device {}

impl PartialOrd for Device {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Device {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.id.cmp(&other.id)
    }
}