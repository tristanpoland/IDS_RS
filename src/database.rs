//! The main PCI database interface.

use crate::vendors::Vendor;
use crate::devices::{Device, Subsystem};
use crate::classes::{DeviceClass, SubClass, ProgInterface};
use crate::types::*;
use alloc::string::ToString;

/// The main PCI database containing all vendor, device, and class information.
///
/// This struct provides the primary interface for querying PCI device information.
/// The database is populated at compile time from the PCI IDs file, ensuring
/// zero runtime overhead for database loading.
#[derive(Debug)]
pub struct PciDatabase {
    /// All known PCI vendors
    vendors: &'static [Vendor],
    /// All known PCI device classes
    classes: &'static [DeviceClass],
}

impl PciDatabase {
    /// Create a new database with the given vendors and classes.
    ///
    /// This is primarily used by the build script to create the static database.
    #[doc(hidden)]
    pub const fn new(vendors: &'static [Vendor], classes: &'static [DeviceClass]) -> Self {
        Self { vendors, classes }
    }

    /// Get the global PCI database instance.
    ///
    /// This function returns a reference to the statically compiled PCI database.
    /// The database is populated at compile time, so this function has zero cost.
    pub fn get() -> &'static Self {
        &GLOBAL_DATABASE
    }

    /// Get all vendors in the database.
    #[inline]
    pub const fn vendors(&self) -> &'static [Vendor] {
        self.vendors
    }

    /// Get all device classes in the database.
    #[inline]
    pub const fn classes(&self) -> &'static [DeviceClass] {
        self.classes
    }

    /// Find a vendor by ID.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ids_rs::{PciDatabase, VendorId};
    ///
    /// let db = PciDatabase::get();
    /// let intel_id = VendorId::new(0x8086);
    ///
    /// if let Some(vendor) = db.find_vendor(intel_id) {
    ///     println!("Found vendor: {}", vendor.name());
    /// }
    /// ```
    pub fn find_vendor(&self, vendor_id: VendorId) -> Option<&Vendor> {
        // Use binary search since vendors are sorted by ID
        self.vendors.binary_search_by_key(&vendor_id, |v| v.id()).ok()
            .map(|index| &self.vendors[index])
    }

    /// Find a device by vendor and device IDs.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ids_rs::{PciDatabase, VendorId, DeviceId};
    ///
    /// let db = PciDatabase::get();
    /// let vendor_id = VendorId::new(0x8086);
    /// let device_id = DeviceId::new(0x1234);
    ///
    /// if let Some(device) = db.find_device(vendor_id, device_id) {
    ///     println!("Found device: {}", device.name());
    /// }
    /// ```
    pub fn find_device(&self, vendor_id: VendorId, device_id: DeviceId) -> Option<&Device> {
        self.find_vendor(vendor_id)?.find_device(device_id)
    }

    /// Find a subsystem by vendor, device, subvendor, and subdevice IDs.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ids_rs::{PciDatabase, VendorId, DeviceId, SubvendorId, SubdeviceId};
    ///
    /// let db = PciDatabase::get();
    /// let vendor_id = VendorId::new(0x8086);
    /// let device_id = DeviceId::new(0x1234);
    /// let subvendor_id = SubvendorId::new(0x1234);
    /// let subdevice_id = SubdeviceId::new(0x5678);
    ///
    /// if let Some(subsystem) = db.find_subsystem(vendor_id, device_id, subvendor_id, subdevice_id) {
    ///     println!("Found subsystem: {}", subsystem.name());
    /// }
    /// ```
    pub fn find_subsystem(
        &self,
        vendor_id: VendorId,
        device_id: DeviceId,
        subvendor_id: SubvendorId,
        subdevice_id: SubdeviceId,
    ) -> Option<&Subsystem> {
        self.find_device(vendor_id, device_id)?
            .find_subsystem(subvendor_id, subdevice_id)
    }

    /// Find a device class by ID.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ids_rs::{PciDatabase, DeviceClassId};
    ///
    /// let db = PciDatabase::get();
    /// let class_id = DeviceClassId::new(0x02); // Network controller
    ///
    /// if let Some(class) = db.find_class(class_id) {
    ///     println!("Found class: {}", class.name());
    /// }
    /// ```
    pub fn find_class(&self, class_id: DeviceClassId) -> Option<&DeviceClass> {
        // Use binary search since classes are sorted by ID
        self.classes.binary_search_by_key(&class_id, |c| c.id()).ok()
            .map(|index| &self.classes[index])
    }

    /// Find a subclass by class and subclass IDs.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ids_rs::{PciDatabase, DeviceClassId, SubClassId};
    ///
    /// let db = PciDatabase::get();
    /// let class_id = DeviceClassId::new(0x02);
    /// let subclass_id = SubClassId::new(0x00);
    ///
    /// if let Some(subclass) = db.find_subclass(class_id, subclass_id) {
    ///     println!("Found subclass: {}", subclass.name());
    /// }
    /// ```
    pub fn find_subclass(&self, class_id: DeviceClassId, subclass_id: SubClassId) -> Option<&SubClass> {
        self.find_class(class_id)?.find_subclass(subclass_id)
    }

    /// Find a programming interface by class, subclass, and programming interface IDs.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ids_rs::{PciDatabase, DeviceClassId, SubClassId, ProgInterfaceId};
    ///
    /// let db = PciDatabase::get();
    /// let class_id = DeviceClassId::new(0x02);
    /// let subclass_id = SubClassId::new(0x01);
    /// let prog_if_id = ProgInterfaceId::new(0x00);
    ///
    /// if let Some(prog_if) = db.find_prog_interface(class_id, subclass_id, prog_if_id) {
    ///     println!("Found programming interface: {}", prog_if.name());
    /// }
    /// ```
    pub fn find_prog_interface(
        &self,
        class_id: DeviceClassId,
        subclass_id: SubClassId,
        prog_interface_id: ProgInterfaceId,
    ) -> Option<&ProgInterface> {
        self.find_class(class_id)?
            .find_prog_interface(subclass_id, prog_interface_id)
    }

    /// Get a human-readable name for a vendor.
    ///
    /// Returns "Unknown Vendor (XXXX)" if the vendor ID is not found.
    pub fn vendor_name(&self, vendor_id: VendorId) -> alloc::string::String {
        match self.find_vendor(vendor_id) {
            Some(vendor) => vendor.name().to_string(),
            None => alloc::format!("Unknown Vendor ({:04x})", vendor_id.value()),
        }
    }

    /// Get a human-readable name for a device.
    ///
    /// Returns "Unknown Device (XXXX)" if the device ID is not found.
    pub fn device_name(&self, vendor_id: VendorId, device_id: DeviceId) -> alloc::string::String {
        match self.find_device(vendor_id, device_id) {
            Some(device) => device.name().to_string(),
            None => alloc::format!("Unknown Device ({:04x})", device_id.value()),
        }
    }

    /// Get a human-readable name for a subsystem.
    ///
    /// Returns "Unknown Subsystem (XXXX:XXXX)" if the subsystem is not found.
    pub fn subsystem_name(
        &self,
        vendor_id: VendorId,
        device_id: DeviceId,
        subvendor_id: SubvendorId,
        subdevice_id: SubdeviceId,
    ) -> alloc::string::String {
        match self.find_subsystem(vendor_id, device_id, subvendor_id, subdevice_id) {
            Some(subsystem) => subsystem.name().to_string(),
            None => alloc::format!(
                "Unknown Subsystem ({:04x}:{:04x})",
                subvendor_id.value(),
                subdevice_id.value()
            ),
        }
    }

    /// Get a human-readable description of a device class.
    ///
    /// Returns "Unknown Class (XX)" if the class ID is not found.
    pub fn class_name(&self, class_id: DeviceClassId) -> alloc::string::String {
        match self.find_class(class_id) {
            Some(class) => class.name().to_string(),
            None => alloc::format!("Unknown Class ({:02x})", class_id.value()),
        }
    }

    /// Get a complete description of a device including vendor, device, and class information.
    ///
    /// This is the most comprehensive lookup function, providing a full description
    /// of a PCI device based on all available identifiers.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ids_rs::{PciDatabase, VendorId, DeviceId, DeviceClassId, SubClassId};
    ///
    /// let db = PciDatabase::get();
    /// let description = db.describe_device(
    ///     VendorId::new(0x8086),
    ///     DeviceId::new(0x1234),
    ///     Some(DeviceClassId::new(0x02)),
    ///     Some(SubClassId::new(0x00)),
    ///     None,
    ///     None,
    ///     None,
    /// );
    ///
    /// println!("Device: {}", description);
    /// ```
    pub fn describe_device(
        &self,
        vendor_id: VendorId,
        device_id: DeviceId,
        class_id: Option<DeviceClassId>,
        subclass_id: Option<SubClassId>,
        prog_interface_id: Option<ProgInterfaceId>,
        subvendor_id: Option<SubvendorId>,
        subdevice_id: Option<SubdeviceId>,
    ) -> alloc::string::String {
        use alloc::format;

        let vendor_name = self.vendor_name(vendor_id);
        let device_name = self.device_name(vendor_id, device_id);

        let mut description = format!("{} {}", vendor_name, device_name);

        // Add class information if available
        if let Some(class_id) = class_id {
            if let Some(class) = self.find_class(class_id) {
                let class_desc = class.describe_device(subclass_id, prog_interface_id);
                description = format!("{} ({})", description, class_desc);
            }
        }

        // Add subsystem information if available
        if let (Some(subvendor_id), Some(subdevice_id)) = (subvendor_id, subdevice_id) {
            let subsystem_name = self.subsystem_name(vendor_id, device_id, subvendor_id, subdevice_id);
            description = format!("{} [{}]", description, subsystem_name);
        }

        description
    }

    /// Get statistics about the database.
    ///
    /// Returns information about the number of vendors, devices, classes, etc.
    pub fn stats(&self) -> DatabaseStats {
        let mut total_devices = 0;
        let mut total_subsystems = 0;
        let mut total_subclasses = 0;
        let mut total_prog_interfaces = 0;

        for vendor in self.vendors {
            total_devices += vendor.device_count();
            for device in vendor.devices() {
                total_subsystems += device.subsystem_count();
            }
        }

        for class in self.classes {
            total_subclasses += class.subclass_count();
            for subclass in class.subclasses() {
                total_prog_interfaces += subclass.prog_interface_count();
            }
        }

        DatabaseStats {
            vendor_count: self.vendors.len(),
            device_count: total_devices,
            subsystem_count: total_subsystems,
            class_count: self.classes.len(),
            subclass_count: total_subclasses,
            prog_interface_count: total_prog_interfaces,
        }
    }

    /// Iterate over all vendors in the database.
    pub fn iter_vendors(&self) -> core::slice::Iter<'_, Vendor> {
        self.vendors.iter()
    }

    /// Iterate over all device classes in the database.
    pub fn iter_classes(&self) -> core::slice::Iter<'_, DeviceClass> {
        self.classes.iter()
    }
}

/// Statistics about the PCI database.
#[derive(Debug, Clone, Copy)]
pub struct DatabaseStats {
    /// Number of vendors
    pub vendor_count: usize,
    /// Total number of devices across all vendors
    pub device_count: usize,
    /// Total number of subsystems across all devices
    pub subsystem_count: usize,
    /// Number of device classes
    pub class_count: usize,
    /// Total number of subclasses across all classes
    pub subclass_count: usize,
    /// Total number of programming interfaces across all subclasses
    pub prog_interface_count: usize,
}

impl DatabaseStats {
    /// Get the total number of entries in the database.
    pub fn total_entries(&self) -> usize {
        self.vendor_count
            + self.device_count
            + self.subsystem_count
            + self.class_count
            + self.subclass_count
            + self.prog_interface_count
    }
}

impl core::fmt::Display for DatabaseStats {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "PCI Database Statistics:\n\
             Vendors: {}\n\
             Devices: {}\n\
             Subsystems: {}\n\
             Classes: {}\n\
             Subclasses: {}\n\
             Programming Interfaces: {}\n\
             Total Entries: {}",
            self.vendor_count,
            self.device_count,
            self.subsystem_count,
            self.class_count,
            self.subclass_count,
            self.prog_interface_count,
            self.total_entries()
        )
    }
}

// This will be generated by the build script
include!(concat!(env!("OUT_DIR"), "/pci_database.rs"));

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_database_creation() {
        let vendors: &[Vendor] = &[];
        let classes: &[DeviceClass] = &[];
        let db = PciDatabase::new(vendors, classes);

        assert_eq!(db.vendors().len(), 0);
        assert_eq!(db.classes().len(), 0);
    }

    #[test]
    fn test_database_stats() {
        let vendors: &[Vendor] = &[];
        let classes: &[DeviceClass] = &[];
        let db = PciDatabase::new(vendors, classes);

        let stats = db.stats();
        assert_eq!(stats.vendor_count, 0);
        assert_eq!(stats.device_count, 0);
        assert_eq!(stats.total_entries(), 0);
    }
}