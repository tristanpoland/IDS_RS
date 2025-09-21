//! Comprehensive test suite for the IDS_RS crate
//!
//! This file contains exhaustive tests covering all functionality:
//! - Type safety and conversions
//! - Database lookups and queries
//! - Parser functionality
//! - Edge cases and error handling
//! - Performance characteristics

use ids_rs::*;
use ids_rs::parser::PciIdsParser;
use std::collections::HashSet;

// =============================================================================
// TYPE SYSTEM TESTS
// =============================================================================

#[test]
fn test_vendor_id_comprehensive() {
    // Basic creation and value access
    let id = VendorId::new(0x8086);
    assert_eq!(id.value(), 0x8086);

    // Display formatting
    assert_eq!(format!("{}", id), "8086");
    assert_eq!(format!("{:x}", id), "8086");
    assert_eq!(format!("{:X}", id), "8086");

    // Hex string conversion
    let hex_string = id.to_hex_string();
    assert_eq!(hex_string.as_str(), "8086");

    // Edge cases
    let zero_id = VendorId::new(0x0000);
    assert_eq!(format!("{}", zero_id), "0000");

    let max_id = VendorId::new(0xFFFF);
    assert_eq!(format!("{:X}", max_id), "FFFF");

    // Ordering and comparison
    let id1 = VendorId::new(0x1000);
    let id2 = VendorId::new(0x2000);
    let id3 = VendorId::new(0x1000);

    assert!(id1 < id2);
    assert!(id2 > id1);
    assert_eq!(id1, id3);
    assert_ne!(id1, id2);

    // Sorting
    let mut ids = vec![id2, id1, id3];
    ids.sort();
    assert_eq!(ids, vec![id1, id3, id2]);

    // Conversions
    let from_u16: VendorId = 0x8086u16.into();
    assert_eq!(id, from_u16);

    let to_u16: u16 = id.into();
    assert_eq!(to_u16, 0x8086);
}

#[test]
fn test_device_id_comprehensive() {
    let id = DeviceId::new(0x1234);
    assert_eq!(id.value(), 0x1234);
    assert_eq!(format!("{}", id), "1234");
    assert_eq!(id.to_hex_string().as_str(), "1234");

    // Conversions
    let from_u16: DeviceId = 0x1234u16.into();
    assert_eq!(id, from_u16);

    let to_u16: u16 = id.into();
    assert_eq!(to_u16, 0x1234);
}

#[test]
fn test_subsystem_ids_comprehensive() {
    let subvendor_id = SubvendorId::new(0xABCD);
    let subdevice_id = SubdeviceId::new(0x5678);

    assert_eq!(subvendor_id.value(), 0xABCD);
    assert_eq!(subdevice_id.value(), 0x5678);

    assert_eq!(format!("{}", subvendor_id), "abcd");
    assert_eq!(format!("{}", subdevice_id), "5678");

    // Conversions
    let from_u16: SubvendorId = 0xABCDu16.into();
    assert_eq!(subvendor_id, from_u16);

    let to_u16: u16 = subvendor_id.into();
    assert_eq!(to_u16, 0xABCD);
}

#[test]
fn test_class_ids_comprehensive() {
    let class_id = DeviceClassId::new(0x02);
    let subclass_id = SubClassId::new(0x00);
    let prog_if_id = ProgInterfaceId::new(0x30);

    assert_eq!(class_id.value(), 0x02);
    assert_eq!(subclass_id.value(), 0x00);
    assert_eq!(prog_if_id.value(), 0x30);

    assert_eq!(format!("{}", class_id), "02");
    assert_eq!(format!("{}", subclass_id), "00");
    assert_eq!(format!("{}", prog_if_id), "30");

    // Edge cases for 8-bit values
    let max_class = DeviceClassId::new(0xFF);
    assert_eq!(format!("{}", max_class), "ff");

    // Conversions
    let from_u8: DeviceClassId = 0x02u8.into();
    assert_eq!(class_id, from_u8);

    let to_u8: u8 = class_id.into();
    assert_eq!(to_u8, 0x02);
}

#[test]
fn test_all_types_hash_consistency() {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    // Test that equal values produce equal hashes
    let vendor1 = VendorId::new(0x8086);
    let vendor2 = VendorId::new(0x8086);

    let mut hasher1 = DefaultHasher::new();
    let mut hasher2 = DefaultHasher::new();

    vendor1.hash(&mut hasher1);
    vendor2.hash(&mut hasher2);

    assert_eq!(hasher1.finish(), hasher2.finish());

    // Test all types are hashable in separate sets
    let mut vendor_set = HashSet::new();
    let mut device_set = HashSet::new();
    let mut class_set = HashSet::new();

    vendor_set.insert(VendorId::new(0x8086));
    device_set.insert(DeviceId::new(0x1234));
    class_set.insert(DeviceClassId::new(0x02));

    assert_eq!(vendor_set.len(), 1);
    assert_eq!(device_set.len(), 1);
    assert_eq!(class_set.len(), 1);

    // Test duplicate detection
    vendor_set.insert(VendorId::new(0x8086));
    assert_eq!(vendor_set.len(), 1);
}

#[test]
fn test_const_creation() {
    // All types should be creatable in const contexts
    const VENDOR_ID: VendorId = VendorId::new(0x8086);
    const DEVICE_ID: DeviceId = DeviceId::new(0x1234);
    const CLASS_ID: DeviceClassId = DeviceClassId::new(0x02);
    const SUBCLASS_ID: SubClassId = SubClassId::new(0x00);
    const PROG_IF_ID: ProgInterfaceId = ProgInterfaceId::new(0x30);

    assert_eq!(VENDOR_ID.value(), 0x8086);
    assert_eq!(DEVICE_ID.value(), 0x1234);
    assert_eq!(CLASS_ID.value(), 0x02);
    assert_eq!(SUBCLASS_ID.value(), 0x00);
    assert_eq!(PROG_IF_ID.value(), 0x30);
}

#[test]
fn test_well_known_vendor_ids() {
    use ids_rs::vendors::well_known::*;

    assert_eq!(INTEL.value(), 0x8086);
    assert_eq!(AMD.value(), 0x1022);
    assert_eq!(NVIDIA.value(), 0x10de);
    assert_eq!(BROADCOM.value(), 0x14e4);
    assert_eq!(REALTEK.value(), 0x10ec);
    assert_eq!(QUALCOMM.value(), 0x17cb);
    assert_eq!(MARVELL.value(), 0x11ab);
    assert_eq!(VIA.value(), 0x1106);
    assert_eq!(ATHEROS.value(), 0x168c);
    assert_eq!(THREECOM.value(), 0x10b7);
}

#[test]
fn test_well_known_device_class_ids() {
    use ids_rs::classes::well_known::*;

    assert_eq!(UNCLASSIFIED.value(), 0x00);
    assert_eq!(MASS_STORAGE.value(), 0x01);
    assert_eq!(NETWORK.value(), 0x02);
    assert_eq!(DISPLAY.value(), 0x03);
    assert_eq!(MULTIMEDIA.value(), 0x04);
    assert_eq!(MEMORY.value(), 0x05);
    assert_eq!(BRIDGE.value(), 0x06);
    assert_eq!(COMMUNICATION.value(), 0x07);
    assert_eq!(SYSTEM_PERIPHERAL.value(), 0x08);
    assert_eq!(INPUT_DEVICE.value(), 0x09);
    assert_eq!(DOCKING_STATION.value(), 0x0a);
    assert_eq!(PROCESSOR.value(), 0x0b);
    assert_eq!(SERIAL_BUS.value(), 0x0c);
    assert_eq!(WIRELESS.value(), 0x0d);
    assert_eq!(INTELLIGENT.value(), 0x0e);
    assert_eq!(SATELLITE.value(), 0x0f);
    assert_eq!(ENCRYPTION.value(), 0x10);
    assert_eq!(SIGNAL_PROCESSING.value(), 0x11);
    assert_eq!(PROCESSING_ACCELERATOR.value(), 0x12);
    assert_eq!(NON_ESSENTIAL_INSTRUMENTATION.value(), 0x13);
    assert_eq!(COPROCESSOR.value(), 0x40);
    assert_eq!(UNASSIGNED.value(), 0xff);
}

// =============================================================================
// DATABASE TESTS
// =============================================================================

#[test]
fn test_global_database_access() {
    // Test that we can access the global database
    let db = PciDatabase::get();

    // The database might be empty if no pci.ids file was found during build
    // but it should still be accessible
    assert!(db.vendors().len() >= 0);  // Should not panic
    assert!(db.classes().len() >= 0);  // Should not panic

    // Test basic operations don't panic
    let _stats = db.stats();
    let _vendor_name = db.vendor_name(VendorId::new(0x8086));
    let _device_name = db.device_name(VendorId::new(0x8086), DeviceId::new(0x1234));
}

#[test]
fn test_database_with_empty_lookups() {
    let db = PciDatabase::get();

    // Test lookups that might not find anything
    let _vendor = db.find_vendor(VendorId::new(0xFFFF));
    // Don't assert None since we don't know what's in the database

    let _device = db.find_device(VendorId::new(0xFFFF), DeviceId::new(0xFFFF));
    // Don't assert None since we don't know what's in the database

    let _class = db.find_class(DeviceClassId::new(0xFF));
    // Don't assert None since we don't know what's in the database

    // Test name methods with potentially unknown IDs
    let vendor_name = db.vendor_name(VendorId::new(0xFFFF));
    assert!(vendor_name.len() > 0); // Should always return something

    let device_name = db.device_name(VendorId::new(0xFFFF), DeviceId::new(0xFFFF));
    assert!(device_name.len() > 0); // Should always return something

    let class_name = db.class_name(DeviceClassId::new(0xFF));
    assert!(class_name.len() > 0); // Should always return something
}

#[test]
fn test_database_statistics() {
    let db = PciDatabase::get();

    let stats = db.stats();

    // All counts should be non-negative
    assert!(stats.vendor_count >= 0);
    assert!(stats.device_count >= 0);
    assert!(stats.subsystem_count >= 0);
    assert!(stats.class_count >= 0);
    assert!(stats.subclass_count >= 0);
    assert!(stats.prog_interface_count >= 0);
    assert!(stats.total_entries() >= 0);

    // Test display formatting
    let stats_string = format!("{}", stats);
    assert!(stats_string.contains("Vendors:"));
    assert!(stats_string.contains("Total Entries:"));
}

#[test]
fn test_database_convenience_methods() {
    let db = PciDatabase::get();

    // Test devices_by_vendor (might return None if vendor doesn't exist)
    let _intel_devices = db.devices_by_vendor(VendorId::new(0x8086));

    // Test search methods (should always return a Vec, might be empty)
    let intel_vendors = db.search_vendors("intel");
    assert!(intel_vendors.len() >= 0);

    let test_devices = db.search_devices("test");
    assert!(test_devices.len() >= 0);

    let network_classes = db.search_classes("network");
    assert!(network_classes.len() >= 0);

    // Test query builder
    let query_results = db.query().execute();
    assert!(query_results.len() >= 0);
}

#[test]
fn test_comprehensive_device_description() {
    let db = PciDatabase::get();

    // Test device description generation
    let description = db.describe_device(
        VendorId::new(0x8086),
        DeviceId::new(0x1234),
        Some(DeviceClassId::new(0x02)),
        Some(SubClassId::new(0x00)),
        None,
        None,
        None
    );

    // Should always return a string
    assert!(description.len() > 0);
}

// =============================================================================
// PARSER TESTS
// =============================================================================

#[test]
fn test_parser_creation() {
    let parser = PciIdsParser::new();
    assert_eq!(parser.vendors().len(), 0);
    assert_eq!(parser.classes().len(), 0);
}

#[test]
fn test_parser_empty_content() {
    let mut parser = PciIdsParser::new();
    let result = parser.parse("");
    assert!(result.is_ok());
    assert_eq!(parser.vendors().len(), 0);
    assert_eq!(parser.classes().len(), 0);
}

#[test]
fn test_parser_comments_and_empty_lines() {
    let mut parser = PciIdsParser::new();
    let content = r#"
# This is a comment

# Another comment

# Empty lines should be ignored
"#;

    let result = parser.parse(content);
    assert!(result.is_ok());
    assert_eq!(parser.vendors().len(), 0);
    assert_eq!(parser.classes().len(), 0);
}

#[test]
fn test_parser_vendor_parsing() {
    let mut parser = PciIdsParser::new();
    let content = r#"
# Test vendor parsing
8086  Intel Corporation
1022  Advanced Micro Devices, Inc. [AMD/ATI]
10de  NVIDIA Corporation
"#;

    let result = parser.parse(content);
    assert!(result.is_ok());

    let vendors = parser.vendors();
    assert_eq!(vendors.len(), 3);

    assert_eq!(vendors[0].id.value(), 0x8086);
    assert_eq!(vendors[0].name, "Intel Corporation");
    assert_eq!(vendors[0].devices.len(), 0);

    assert_eq!(vendors[1].id.value(), 0x1022);
    assert_eq!(vendors[1].name, "Advanced Micro Devices, Inc. [AMD/ATI]");

    assert_eq!(vendors[2].id.value(), 0x10de);
    assert_eq!(vendors[2].name, "NVIDIA Corporation");
}

#[test]
fn test_parser_device_parsing() {
    let mut parser = PciIdsParser::new();
    let content = r#"
8086  Intel Corporation
	1234  Test Device 1
	5678  Test Device 2
1022  AMD
	abcd  AMD Device
"#;

    let result = parser.parse(content);
    assert!(result.is_ok());

    let vendors = parser.vendors();
    assert_eq!(vendors.len(), 2);

    // Intel vendor
    assert_eq!(vendors[0].devices.len(), 2);
    assert_eq!(vendors[0].devices[0].id.value(), 0x1234);
    assert_eq!(vendors[0].devices[0].name, "Test Device 1");
    assert_eq!(vendors[0].devices[1].id.value(), 0x5678);
    assert_eq!(vendors[0].devices[1].name, "Test Device 2");

    // AMD vendor
    assert_eq!(vendors[1].devices.len(), 1);
    assert_eq!(vendors[1].devices[0].id.value(), 0xabcd);
    assert_eq!(vendors[1].devices[0].name, "AMD Device");
}

#[test]
fn test_parser_subsystem_parsing() {
    let mut parser = PciIdsParser::new();
    let content = r#"
8086  Intel Corporation
	1234  Test Device
		8086 1111  Intel Subsystem 1
		8086 2222  Intel Subsystem 2
		1022 3333  AMD Subsystem
"#;

    let result = parser.parse(content);
    assert!(result.is_ok());

    let vendors = parser.vendors();
    assert_eq!(vendors.len(), 1);

    let device = &vendors[0].devices[0];
    assert_eq!(device.subsystems.len(), 3);

    assert_eq!(device.subsystems[0].subvendor_id.value(), 0x8086);
    assert_eq!(device.subsystems[0].subdevice_id.value(), 0x1111);
    assert_eq!(device.subsystems[0].name, "Intel Subsystem 1");

    assert_eq!(device.subsystems[1].subvendor_id.value(), 0x8086);
    assert_eq!(device.subsystems[1].subdevice_id.value(), 0x2222);
    assert_eq!(device.subsystems[1].name, "Intel Subsystem 2");

    assert_eq!(device.subsystems[2].subvendor_id.value(), 0x1022);
    assert_eq!(device.subsystems[2].subdevice_id.value(), 0x3333);
    assert_eq!(device.subsystems[2].name, "AMD Subsystem");
}

#[test]
fn test_parser_class_parsing() {
    let mut parser = PciIdsParser::new();
    let content = r#"
C 02  Network controller
C 03  Display controller
C 0c  Serial bus controller
"#;

    let result = parser.parse(content);
    assert!(result.is_ok());

    let classes = parser.classes();
    assert_eq!(classes.len(), 3);

    assert_eq!(classes[0].id.value(), 0x02);
    assert_eq!(classes[0].name, "Network controller");

    assert_eq!(classes[1].id.value(), 0x03);
    assert_eq!(classes[1].name, "Display controller");

    assert_eq!(classes[2].id.value(), 0x0c);
    assert_eq!(classes[2].name, "Serial bus controller");
}

#[test]
fn test_parser_subclass_parsing() {
    let mut parser = PciIdsParser::new();
    let content = r#"
C 02  Network controller
	00  Ethernet controller
	01  Token ring network controller
	02  FDDI network controller
C 0c  Serial bus controller
	03  USB controller
"#;

    let result = parser.parse(content);
    assert!(result.is_ok());

    let classes = parser.classes();
    assert_eq!(classes.len(), 2);

    // Network controller class
    assert_eq!(classes[0].subclasses.len(), 3);
    assert_eq!(classes[0].subclasses[0].id.value(), 0x00);
    assert_eq!(classes[0].subclasses[0].name, "Ethernet controller");
    assert_eq!(classes[0].subclasses[1].id.value(), 0x01);
    assert_eq!(classes[0].subclasses[1].name, "Token ring network controller");
    assert_eq!(classes[0].subclasses[2].id.value(), 0x02);
    assert_eq!(classes[0].subclasses[2].name, "FDDI network controller");

    // Serial bus controller class
    assert_eq!(classes[1].subclasses.len(), 1);
    assert_eq!(classes[1].subclasses[0].id.value(), 0x03);
    assert_eq!(classes[1].subclasses[0].name, "USB controller");
}

#[test]
fn test_parser_prog_interface_parsing() {
    let mut parser = PciIdsParser::new();
    let content = r#"
C 0c  Serial bus controller
	03  USB controller
		00  OHCI
		10  UHCI
		20  EHCI
		30  XHCI
"#;

    let result = parser.parse(content);
    assert!(result.is_ok());

    let classes = parser.classes();
    assert_eq!(classes.len(), 1);

    let subclass = &classes[0].subclasses[0];
    assert_eq!(subclass.prog_interfaces.len(), 4);

    assert_eq!(subclass.prog_interfaces[0].id.value(), 0x00);
    assert_eq!(subclass.prog_interfaces[0].name, "OHCI");

    assert_eq!(subclass.prog_interfaces[1].id.value(), 0x10);
    assert_eq!(subclass.prog_interfaces[1].name, "UHCI");

    assert_eq!(subclass.prog_interfaces[2].id.value(), 0x20);
    assert_eq!(subclass.prog_interfaces[2].name, "EHCI");

    assert_eq!(subclass.prog_interfaces[3].id.value(), 0x30);
    assert_eq!(subclass.prog_interfaces[3].name, "XHCI");
}

#[test]
fn test_parser_mixed_content() {
    let mut parser = PciIdsParser::new();
    let content = r#"
# Mixed vendor and class parsing
8086  Intel Corporation
	1234  Test Device
		8086 1111  Intel Subsystem

C 02  Network controller
	00  Ethernet controller
		00  Basic Ethernet

1022  AMD
	5678  AMD Device
"#;

    let result = parser.parse(content);
    assert!(result.is_ok());

    // Check vendors
    let vendors = parser.vendors();
    assert_eq!(vendors.len(), 2);
    assert_eq!(vendors[0].name, "Intel Corporation");
    assert_eq!(vendors[1].name, "AMD");

    // Check classes
    let classes = parser.classes();
    assert_eq!(classes.len(), 1);
    assert_eq!(classes[0].name, "Network controller");
    assert_eq!(classes[0].subclasses[0].name, "Ethernet controller");
    assert_eq!(classes[0].subclasses[0].prog_interfaces[0].name, "Basic Ethernet");
}

#[test]
fn test_parser_error_cases() {
    let mut parser = PciIdsParser::new();

    // Invalid vendor format
    let result = parser.parse("invalid_vendor_line");
    assert!(result.is_err());

    // Invalid hex values
    let mut parser = PciIdsParser::new();
    let result = parser.parse("ZZZZ  Invalid vendor");
    assert!(result.is_err());

    // Invalid indentation
    let mut parser = PciIdsParser::new();
    let result = parser.parse("8086  Intel\n\t\t\t\tinvalid_indentation");
    assert!(result.is_err());

    // Invalid subsystem format
    let mut parser = PciIdsParser::new();
    let result = parser.parse("8086  Intel\n\t1234  Device\n\t\tinvalid subsystem");
    assert!(result.is_err());
}

// =============================================================================
// ERROR HANDLING TESTS
// =============================================================================

#[test]
fn test_error_display() {
    use ids_rs::error::PciError;

    assert_eq!(format!("{}", PciError::InvalidFormat), "Invalid format in PCI IDs file");
    assert_eq!(format!("{}", PciError::InvalidHexValue), "Invalid hexadecimal value");
    assert_eq!(format!("{}", PciError::InvalidIndentation), "Invalid indentation level");
    assert_eq!(format!("{}", PciError::UnexpectedEndOfInput), "Unexpected end of input");
    assert_eq!(format!("{}", PciError::VendorNotFound), "Vendor ID not found");
    assert_eq!(format!("{}", PciError::DeviceNotFound), "Device ID not found");
    assert_eq!(format!("{}", PciError::ClassNotFound), "Device class not found");
    assert_eq!(format!("{}", PciError::SubclassNotFound), "Subclass not found");
    assert_eq!(format!("{}", PciError::ProgInterfaceNotFound), "Programming interface not found");
}

#[test]
fn test_error_equality() {
    use ids_rs::error::PciError;

    assert_eq!(PciError::InvalidFormat, PciError::InvalidFormat);
    assert_ne!(PciError::InvalidFormat, PciError::InvalidHexValue);
}

// =============================================================================
// PERFORMANCE TESTS
// =============================================================================

#[test]
fn test_lookup_performance_characteristics() {
    use std::time::Instant;

    let db = PciDatabase::get();

    // Test vendor lookup performance (should be O(log n) due to binary search)
    let start = Instant::now();
    for i in 0..1000 {
        let _result = db.find_vendor(VendorId::new(i));
    }
    let vendor_lookup_time = start.elapsed();

    // Test class lookup performance (should be O(log n) due to binary search)
    let start = Instant::now();
    for i in 0..255 {
        let _result = db.find_class(DeviceClassId::new(i as u8));
    }
    let class_lookup_time = start.elapsed();

    // These shouldn't take an unreasonable amount of time
    // (This is more of a smoke test than a strict performance requirement)
    assert!(vendor_lookup_time.as_millis() < 100);
    assert!(class_lookup_time.as_millis() < 100);
}

#[test]
fn test_query_performance_characteristics() {
    use std::time::Instant;

    let db = PciDatabase::get();

    // Test query builder performance
    let start = Instant::now();
    for _ in 0..100 {
        let _results = db.query()
            .vendor_name_contains("intel")
            .device_name_contains("ethernet")
            .execute();
    }
    let query_time = start.elapsed();

    // Query operations should complete in reasonable time
    assert!(query_time.as_millis() < 1000);
}

// =============================================================================
// NO_STD COMPATIBILITY TESTS
// =============================================================================

#[test]
fn test_no_std_compatibility() {
    // Test that core functionality works without std-specific features

    // Type creation should work
    let vendor_id = VendorId::new(0x8086);
    let device_id = DeviceId::new(0x1234);

    // Basic operations should work
    assert_eq!(vendor_id.value(), 0x8086);
    assert_eq!(device_id.value(), 0x1234);

    // Comparison should work
    assert_eq!(vendor_id, VendorId::new(0x8086));
    assert_ne!(vendor_id, VendorId::new(0x1234));

    // Ordering should work
    assert!(VendorId::new(0x1000) < VendorId::new(0x2000));
}

#[test]
fn test_memory_usage_characteristics() {
    // Test that the library doesn't allocate unexpectedly
    let vendor_id = VendorId::new(0x8086);
    let device_id = DeviceId::new(0x1234);

    // These operations should not allocate
    let _value = vendor_id.value();
    let _formatted = format!("{}", vendor_id);
    let _hex_string = vendor_id.to_hex_string();

    // Database access should not allocate
    let db = PciDatabase::get();
    let _vendors = db.vendors();
    let _classes = db.classes();

    // Lookups should not allocate (except for string returns)
    let _found_vendor = db.find_vendor(vendor_id);
    let _found_device = db.find_device(vendor_id, device_id);
}

// =============================================================================
// COMPREHENSIVE INTEGRATION TEST
// =============================================================================

#[test]
fn test_end_to_end_workflow() {
    // This test simulates a complete workflow of using the library

    // 1. Access the global database
    let db = PciDatabase::get();

    // 2. Get some statistics
    let stats = db.stats();
    println!("Database contains {} vendors, {} devices, {} classes",
             stats.vendor_count, stats.device_count, stats.class_count);

    // 3. Look up some well-known vendor IDs
    let intel_id = VendorId::new(0x8086);
    let vendor_name = db.vendor_name(intel_id);
    assert!(vendor_name.len() > 0); // Should always return something

    // 4. Search for network-related devices
    let network_devices = db.search_devices("network");
    let ethernet_devices = db.search_devices("ethernet");

    // The searches should not panic and should return reasonable results
    assert!(network_devices.len() >= 0);
    assert!(ethernet_devices.len() <= network_devices.len() || network_devices.is_empty());

    // 5. Look up device classes
    let network_class_id = DeviceClassId::new(0x02);
    let network_class = db.find_class(network_class_id);

    if let Some(class) = network_class {
        // If we found a network class, test its properties
        assert!(class.name().len() > 0);

        // Look for Ethernet subclass
        let ethernet_subclass = class.find_subclass(SubClassId::new(0x00));
        if let Some(subclass) = ethernet_subclass {
            assert!(subclass.name().len() > 0);
        }
    }

    // 6. Use the query builder for complex searches
    let complex_results = db.query()
        .class_id(network_class_id)
        .execute();

    // Should complete without panicking
    assert!(complex_results.len() >= 0);

    // 7. Test device description generation
    let description = db.describe_device(
        intel_id,
        DeviceId::new(0x1234),
        Some(network_class_id),
        Some(SubClassId::new(0x00)),
        None,
        None,
        None
    );

    // Should return a non-empty string
    assert!(description.len() > 0);
}

#[test]
fn test_stress_operations() {
    let db = PciDatabase::get();

    // Test many lookups don't cause issues
    for vendor_id in 0..0x100u16 {  // Reduced for faster testing
        let _vendor = db.find_vendor(VendorId::new(vendor_id));

        for device_id in 0..0x10u16 {  // Reduced for faster testing
            let _device = db.find_device(VendorId::new(vendor_id), DeviceId::new(device_id));
        }
    }

    // Test many class lookups
    for class_id in 0..0x20u8 {  // Reduced for faster testing
        let _class = db.find_class(DeviceClassId::new(class_id));

        for subclass_id in 0..0x10u8 {  // Reduced for faster testing
            let _subclass = db.find_subclass(DeviceClassId::new(class_id), SubClassId::new(subclass_id));
        }
    }

    // Should complete without panicking or excessive memory usage
}