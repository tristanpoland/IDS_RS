//! PCI device class definitions and utilities.

use crate::types::{DeviceClassId, SubClassId, ProgInterfaceId};
use alloc::string::ToString;

/// Represents a PCI programming interface within a subclass.
#[derive(Debug, Clone)]
pub struct ProgInterface {
    /// The programming interface ID
    pub id: ProgInterfaceId,
    /// The programming interface name
    pub name: &'static str,
}

impl ProgInterface {
    /// Create a new programming interface.
    #[inline]
    pub const fn new(id: ProgInterfaceId, name: &'static str) -> Self {
        Self { id, name }
    }

    /// Get the programming interface ID.
    #[inline]
    pub const fn id(&self) -> ProgInterfaceId {
        self.id
    }

    /// Get the programming interface name.
    #[inline]
    pub const fn name(&self) -> &'static str {
        self.name
    }
}

impl PartialEq for ProgInterface {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for ProgInterface {}

impl PartialOrd for ProgInterface {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ProgInterface {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.id.cmp(&other.id)
    }
}

/// Represents a PCI subclass within a device class.
#[derive(Debug, Clone)]
pub struct SubClass {
    /// The subclass ID
    pub id: SubClassId,
    /// The subclass name
    pub name: &'static str,
    /// The programming interfaces for this subclass
    pub prog_interfaces: &'static [ProgInterface],
}

impl SubClass {
    /// Create a new subclass.
    #[inline]
    pub const fn new(id: SubClassId, name: &'static str, prog_interfaces: &'static [ProgInterface]) -> Self {
        Self {
            id,
            name,
            prog_interfaces,
        }
    }

    /// Get the subclass ID.
    #[inline]
    pub const fn id(&self) -> SubClassId {
        self.id
    }

    /// Get the subclass name.
    #[inline]
    pub const fn name(&self) -> &'static str {
        self.name
    }

    /// Get all programming interfaces for this subclass.
    #[inline]
    pub const fn prog_interfaces(&self) -> &'static [ProgInterface] {
        self.prog_interfaces
    }

    /// Find a specific programming interface by ID.
    pub fn find_prog_interface(&self, prog_interface_id: ProgInterfaceId) -> Option<&ProgInterface> {
        self.prog_interfaces
            .iter()
            .find(|prog_if| prog_if.id == prog_interface_id)
    }

    /// Get the number of programming interfaces for this subclass.
    #[inline]
    pub const fn prog_interface_count(&self) -> usize {
        self.prog_interfaces.len()
    }

    /// Check if this subclass has a specific programming interface.
    pub fn has_prog_interface(&self, prog_interface_id: ProgInterfaceId) -> bool {
        self.find_prog_interface(prog_interface_id).is_some()
    }

    /// Iterate over all programming interfaces for this subclass.
    pub fn iter_prog_interfaces(&self) -> core::slice::Iter<'_, ProgInterface> {
        self.prog_interfaces.iter()
    }
}

impl PartialEq for SubClass {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for SubClass {}

impl PartialOrd for SubClass {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for SubClass {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.id.cmp(&other.id)
    }
}

/// Represents a PCI device class.
#[derive(Debug, Clone)]
pub struct DeviceClass {
    /// The device class ID
    pub id: DeviceClassId,
    /// The device class name
    pub name: &'static str,
    /// The subclasses for this device class
    pub subclasses: &'static [SubClass],
}

impl DeviceClass {
    /// Create a new device class.
    #[inline]
    pub const fn new(id: DeviceClassId, name: &'static str, subclasses: &'static [SubClass]) -> Self {
        Self {
            id,
            name,
            subclasses,
        }
    }

    /// Get the device class ID.
    #[inline]
    pub const fn id(&self) -> DeviceClassId {
        self.id
    }

    /// Get the device class name.
    #[inline]
    pub const fn name(&self) -> &'static str {
        self.name
    }

    /// Get all subclasses for this device class.
    #[inline]
    pub const fn subclasses(&self) -> &'static [SubClass] {
        self.subclasses
    }

    /// Find a specific subclass by ID.
    pub fn find_subclass(&self, subclass_id: SubClassId) -> Option<&SubClass> {
        self.subclasses.iter().find(|subclass| subclass.id == subclass_id)
    }

    /// Find a specific programming interface by subclass and prog-if IDs.
    pub fn find_prog_interface(&self, subclass_id: SubClassId, prog_interface_id: ProgInterfaceId) -> Option<&ProgInterface> {
        self.find_subclass(subclass_id)?
            .find_prog_interface(prog_interface_id)
    }

    /// Get the number of subclasses for this device class.
    #[inline]
    pub const fn subclass_count(&self) -> usize {
        self.subclasses.len()
    }

    /// Check if this device class has a specific subclass.
    pub fn has_subclass(&self, subclass_id: SubClassId) -> bool {
        self.find_subclass(subclass_id).is_some()
    }

    /// Iterate over all subclasses for this device class.
    pub fn iter_subclasses(&self) -> core::slice::Iter<'_, SubClass> {
        self.subclasses.iter()
    }

    /// Get a human-readable description of a device with the given class, subclass, and prog-if.
    pub fn describe_device(&self, subclass_id: Option<SubClassId>, prog_interface_id: Option<ProgInterfaceId>) -> alloc::string::String {
        use alloc::format;

        match (subclass_id, prog_interface_id) {
            (Some(sc_id), Some(pi_id)) => {
                if let Some(subclass) = self.find_subclass(sc_id) {
                    if let Some(prog_if) = subclass.find_prog_interface(pi_id) {
                        format!("{} - {} - {}", self.name, subclass.name, prog_if.name)
                    } else {
                        format!("{} - {} - Unknown Programming Interface ({:02x})", self.name, subclass.name, pi_id.value())
                    }
                } else {
                    format!("{} - Unknown Subclass ({:02x})", self.name, sc_id.value())
                }
            }
            (Some(sc_id), None) => {
                if let Some(subclass) = self.find_subclass(sc_id) {
                    format!("{} - {}", self.name, subclass.name)
                } else {
                    format!("{} - Unknown Subclass ({:02x})", self.name, sc_id.value())
                }
            }
            (None, _) => self.name.to_string(),
        }
    }
}

impl PartialEq for DeviceClass {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for DeviceClass {}

impl PartialOrd for DeviceClass {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for DeviceClass {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.id.cmp(&other.id)
    }
}

/// Well-known device class IDs for convenience.
pub mod well_known {
    use super::DeviceClassId;

    /// Unclassified device
    pub const UNCLASSIFIED: DeviceClassId = DeviceClassId::new(0x00);

    /// Mass storage controller
    pub const MASS_STORAGE: DeviceClassId = DeviceClassId::new(0x01);

    /// Network controller
    pub const NETWORK: DeviceClassId = DeviceClassId::new(0x02);

    /// Display controller
    pub const DISPLAY: DeviceClassId = DeviceClassId::new(0x03);

    /// Multimedia controller
    pub const MULTIMEDIA: DeviceClassId = DeviceClassId::new(0x04);

    /// Memory controller
    pub const MEMORY: DeviceClassId = DeviceClassId::new(0x05);

    /// Bridge device
    pub const BRIDGE: DeviceClassId = DeviceClassId::new(0x06);

    /// Simple communication controller
    pub const COMMUNICATION: DeviceClassId = DeviceClassId::new(0x07);

    /// Base system peripheral
    pub const SYSTEM_PERIPHERAL: DeviceClassId = DeviceClassId::new(0x08);

    /// Input device controller
    pub const INPUT_DEVICE: DeviceClassId = DeviceClassId::new(0x09);

    /// Docking station
    pub const DOCKING_STATION: DeviceClassId = DeviceClassId::new(0x0a);

    /// Processor
    pub const PROCESSOR: DeviceClassId = DeviceClassId::new(0x0b);

    /// Serial bus controller
    pub const SERIAL_BUS: DeviceClassId = DeviceClassId::new(0x0c);

    /// Wireless controller
    pub const WIRELESS: DeviceClassId = DeviceClassId::new(0x0d);

    /// Intelligent controller
    pub const INTELLIGENT: DeviceClassId = DeviceClassId::new(0x0e);

    /// Satellite communication controller
    pub const SATELLITE: DeviceClassId = DeviceClassId::new(0x0f);

    /// Encryption controller
    pub const ENCRYPTION: DeviceClassId = DeviceClassId::new(0x10);

    /// Signal processing controller
    pub const SIGNAL_PROCESSING: DeviceClassId = DeviceClassId::new(0x11);

    /// Processing accelerator
    pub const PROCESSING_ACCELERATOR: DeviceClassId = DeviceClassId::new(0x12);

    /// Non-essential instrumentation
    pub const NON_ESSENTIAL_INSTRUMENTATION: DeviceClassId = DeviceClassId::new(0x13);

    /// Co-processor
    pub const COPROCESSOR: DeviceClassId = DeviceClassId::new(0x40);

    /// Unassigned class
    pub const UNASSIGNED: DeviceClassId = DeviceClassId::new(0xff);
}