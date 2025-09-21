//! Advanced query interface for the PCI database.

use crate::database::PciDatabase;
use crate::vendors::Vendor;
use crate::devices::Device;
use crate::classes::{DeviceClass, SubClass};
use crate::types::*;
use alloc::{vec::Vec, string::String, string::ToString};

/// Builder for constructing complex PCI device queries.
///
/// This provides a fluent interface for building sophisticated queries
/// against the PCI database, allowing filtering and searching across
/// multiple criteria.
///
/// # Examples
///
/// ```rust
/// use ids_rs::{PciDatabase, QueryBuilder};
///
/// let db = PciDatabase::get();
/// let intel_network_devices = QueryBuilder::new(db)
///     .vendor_name_contains("Intel")
///     .class_name_contains("Network")
///     .execute();
/// ```
#[derive(Debug)]
pub struct QueryBuilder<'db> {
    database: &'db PciDatabase,
    vendor_id_filter: Option<VendorId>,
    vendor_name_filter: Option<String>,
    device_id_filter: Option<DeviceId>,
    device_name_filter: Option<String>,
    class_id_filter: Option<DeviceClassId>,
    class_name_filter: Option<String>,
    subclass_id_filter: Option<SubClassId>,
    subclass_name_filter: Option<String>,
}

impl<'db> QueryBuilder<'db> {
    /// Create a new query builder for the given database.
    pub fn new(database: &'db PciDatabase) -> Self {
        Self {
            database,
            vendor_id_filter: None,
            vendor_name_filter: None,
            device_id_filter: None,
            device_name_filter: None,
            class_id_filter: None,
            class_name_filter: None,
            subclass_id_filter: None,
            subclass_name_filter: None,
        }
    }

    /// Filter by vendor ID.
    pub fn vendor_id(mut self, vendor_id: VendorId) -> Self {
        self.vendor_id_filter = Some(vendor_id);
        self
    }

    /// Filter by vendor name (case-insensitive substring match).
    pub fn vendor_name_contains(mut self, name: &str) -> Self {
        self.vendor_name_filter = Some(name.to_lowercase());
        self
    }

    /// Filter by device ID.
    pub fn device_id(mut self, device_id: DeviceId) -> Self {
        self.device_id_filter = Some(device_id);
        self
    }

    /// Filter by device name (case-insensitive substring match).
    pub fn device_name_contains(mut self, name: &str) -> Self {
        self.device_name_filter = Some(name.to_lowercase());
        self
    }

    /// Filter by device class ID.
    pub fn class_id(mut self, class_id: DeviceClassId) -> Self {
        self.class_id_filter = Some(class_id);
        self
    }

    /// Filter by device class name (case-insensitive substring match).
    pub fn class_name_contains(mut self, name: &str) -> Self {
        self.class_name_filter = Some(name.to_lowercase());
        self
    }

    /// Filter by subclass ID.
    pub fn subclass_id(mut self, subclass_id: SubClassId) -> Self {
        self.subclass_id_filter = Some(subclass_id);
        self
    }

    /// Filter by subclass name (case-insensitive substring match).
    pub fn subclass_name_contains(mut self, name: &str) -> Self {
        self.subclass_name_filter = Some(name.to_lowercase());
        self
    }

    /// Execute the query and return matching device results.
    pub fn execute(self) -> Vec<DeviceMatch<'db>> {
        let mut results = Vec::new();

        for vendor in self.database.vendors() {
            // Check vendor filters
            if let Some(ref vendor_id) = self.vendor_id_filter {
                if vendor.id() != *vendor_id {
                    continue;
                }
            }

            if let Some(ref vendor_name) = self.vendor_name_filter {
                if !vendor.name().to_lowercase().contains(vendor_name) {
                    continue;
                }
            }

            for device in vendor.devices() {
                // Check device filters
                if let Some(ref device_id) = self.device_id_filter {
                    if device.id() != *device_id {
                        continue;
                    }
                }

                if let Some(ref device_name) = self.device_name_filter {
                    if !device.name().to_lowercase().contains(device_name) {
                        continue;
                    }
                }

                // If we have class filters, we need to check if any class matches
                let class_match = self.find_matching_class();

                if self.has_class_filters() && class_match.is_none() {
                    continue;
                }

                results.push(DeviceMatch {
                    vendor,
                    device,
                    class_info: class_match,
                });
            }
        }

        results
    }

    /// Execute the query and return matching vendor results.
    pub fn execute_vendors(self) -> Vec<&'db Vendor> {
        let mut results = Vec::new();

        for vendor in self.database.vendors() {
            // Check vendor filters
            if let Some(ref vendor_id) = self.vendor_id_filter {
                if vendor.id() != *vendor_id {
                    continue;
                }
            }

            if let Some(ref vendor_name) = self.vendor_name_filter {
                if !vendor.name().to_lowercase().contains(vendor_name) {
                    continue;
                }
            }

            results.push(vendor);
        }

        results
    }

    /// Execute the query and return matching class results.
    pub fn execute_classes(self) -> Vec<ClassMatch<'db>> {
        let mut results = Vec::new();

        for class in self.database.classes() {
            // Check class filters
            if let Some(ref class_id) = self.class_id_filter {
                if class.id() != *class_id {
                    continue;
                }
            }

            if let Some(ref class_name) = self.class_name_filter {
                if !class.name().to_lowercase().contains(class_name) {
                    continue;
                }
            }

            // Check subclass filters
            let matching_subclasses: Vec<&SubClass> = class
                .subclasses()
                .iter()
                .filter(|subclass| {
                    if let Some(ref subclass_id) = self.subclass_id_filter {
                        if subclass.id() != *subclass_id {
                            return false;
                        }
                    }

                    if let Some(ref subclass_name) = self.subclass_name_filter {
                        if !subclass.name().to_lowercase().contains(subclass_name) {
                            return false;
                        }
                    }

                    true
                })
                .collect();

            if self.has_subclass_filters() && matching_subclasses.is_empty() {
                continue;
            }

            results.push(ClassMatch {
                class,
                matching_subclasses,
            });
        }

        results
    }

    fn has_class_filters(&self) -> bool {
        self.class_id_filter.is_some() || self.class_name_filter.is_some() || self.has_subclass_filters()
    }

    fn has_subclass_filters(&self) -> bool {
        self.subclass_id_filter.is_some() || self.subclass_name_filter.is_some()
    }

    fn find_matching_class(&self) -> Option<&'db DeviceClass> {
        for class in self.database.classes() {
            if let Some(ref class_id) = self.class_id_filter {
                if class.id() != *class_id {
                    continue;
                }
            }

            if let Some(ref class_name) = self.class_name_filter {
                if !class.name().to_lowercase().contains(class_name) {
                    continue;
                }
            }

            // Check if any subclass matches
            if self.has_subclass_filters() {
                let has_matching_subclass = class.subclasses().iter().any(|subclass| {
                    if let Some(ref subclass_id) = self.subclass_id_filter {
                        if subclass.id() != *subclass_id {
                            return false;
                        }
                    }

                    if let Some(ref subclass_name) = self.subclass_name_filter {
                        if !subclass.name().to_lowercase().contains(subclass_name) {
                            return false;
                        }
                    }

                    true
                });

                if !has_matching_subclass {
                    continue;
                }
            }

            return Some(class);
        }

        None
    }
}

/// A device match result from a query.
#[derive(Debug)]
pub struct DeviceMatch<'db> {
    /// The matching vendor
    pub vendor: &'db Vendor,
    /// The matching device
    pub device: &'db Device,
    /// Optional class information if class filters were used
    pub class_info: Option<&'db DeviceClass>,
}

impl<'db> DeviceMatch<'db> {
    /// Get the vendor ID.
    pub fn vendor_id(&self) -> VendorId {
        self.vendor.id()
    }

    /// Get the vendor name.
    pub fn vendor_name(&self) -> &'static str {
        self.vendor.name()
    }

    /// Get the device ID.
    pub fn device_id(&self) -> DeviceId {
        self.device.id()
    }

    /// Get the device name.
    pub fn device_name(&self) -> &'static str {
        self.device.name()
    }

    /// Get a formatted description of this device match.
    pub fn description(&self) -> String {
        if let Some(class) = self.class_info {
            alloc::format!(
                "{} {} ({})",
                self.vendor_name(),
                self.device_name(),
                class.name()
            )
        } else {
            alloc::format!("{} {}", self.vendor_name(), self.device_name())
        }
    }
}

/// A class match result from a query.
#[derive(Debug)]
pub struct ClassMatch<'db> {
    /// The matching class
    pub class: &'db DeviceClass,
    /// Subclasses that matched the query (empty if no subclass filters were used)
    pub matching_subclasses: Vec<&'db SubClass>,
}

impl<'db> ClassMatch<'db> {
    /// Get the class ID.
    pub fn class_id(&self) -> DeviceClassId {
        self.class.id()
    }

    /// Get the class name.
    pub fn class_name(&self) -> &'static str {
        self.class.name()
    }

    /// Get a formatted description of this class match.
    pub fn description(&self) -> String {
        if self.matching_subclasses.is_empty() {
            self.class_name().to_string()
        } else {
            let subclass_names: Vec<&str> = self
                .matching_subclasses
                .iter()
                .map(|sc| sc.name())
                .collect();
            alloc::format!("{} ({})", self.class_name(), subclass_names.join(", "))
        }
    }
}

/// Convenience functions for common queries.
impl PciDatabase {
    /// Find all devices from a specific vendor.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ids_rs::{PciDatabase, VendorId};
    ///
    /// let db = PciDatabase::get();
    /// let intel_devices = db.devices_by_vendor(VendorId::new(0x8086));
    /// ```
    pub fn devices_by_vendor(&self, vendor_id: VendorId) -> Option<&[Device]> {
        self.find_vendor(vendor_id).map(|vendor| vendor.devices())
    }

    /// Find all devices of a specific class.
    ///
    /// Note: This returns device IDs that could belong to the class, but
    /// actual class assignment depends on the device's configuration.
    pub fn devices_by_class(&self, class_id: DeviceClassId) -> Vec<DeviceMatch<'_>> {
        QueryBuilder::new(self).class_id(class_id).execute()
    }

    /// Search for vendors by name (case-insensitive).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ids_rs::PciDatabase;
    ///
    /// let db = PciDatabase::get();
    /// let intel_vendors = db.search_vendors("intel");
    /// ```
    pub fn search_vendors(&self, name: &str) -> Vec<&Vendor> {
        QueryBuilder::new(self)
            .vendor_name_contains(name)
            .execute_vendors()
    }

    /// Search for devices by name (case-insensitive).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ids_rs::PciDatabase;
    ///
    /// let db = PciDatabase::get();
    /// let ethernet_devices = db.search_devices("ethernet");
    /// ```
    pub fn search_devices(&self, name: &str) -> Vec<DeviceMatch<'_>> {
        QueryBuilder::new(self)
            .device_name_contains(name)
            .execute()
    }

    /// Search for device classes by name (case-insensitive).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ids_rs::PciDatabase;
    ///
    /// let db = PciDatabase::get();
    /// let network_classes = db.search_classes("network");
    /// ```
    pub fn search_classes(&self, name: &str) -> Vec<ClassMatch<'_>> {
        QueryBuilder::new(self)
            .class_name_contains(name)
            .execute_classes()
    }

    /// Get a query builder for this database.
    ///
    /// This provides access to the full query interface.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ids_rs::PciDatabase;
    ///
    /// let db = PciDatabase::get();
    /// let results = db.query()
    ///     .vendor_name_contains("Intel")
    ///     .class_name_contains("Network")
    ///     .execute();
    /// ```
    pub fn query(&self) -> QueryBuilder<'_> {
        QueryBuilder::new(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vendors::Vendor;
    use crate::devices::Device;
    use crate::classes::DeviceClass;

    #[test]
    fn test_query_builder_creation() {
        let vendors: &[Vendor] = &[];
        let classes: &[DeviceClass] = &[];
        let db = PciDatabase::new(vendors, classes);

        let query = QueryBuilder::new(&db);
        let results = query.execute();
        assert!(results.is_empty());
    }

    #[test]
    fn test_empty_database_queries() {
        let vendors: &[Vendor] = &[];
        let classes: &[DeviceClass] = &[];
        let db = PciDatabase::new(vendors, classes);

        assert!(db.search_vendors("test").is_empty());
        assert!(db.search_devices("test").is_empty());
        assert!(db.search_classes("test").is_empty());
    }
}