//! Parser for the PCI IDs database format.

use alloc::{string::String, vec::Vec, string::ToString};
use crate::error::{PciError, PciResult};
use crate::types::*;

/// Parser state for tracking which section we're currently parsing.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ParsingMode {
    /// Parsing vendor and device information
    Vendors,
    /// Parsing device class information
    Classes,
}

/// Internal parser state for vendors and devices.
#[derive(Debug)]
#[allow(dead_code)]
pub struct VendorBuilder {
    /// The vendor ID
    pub id: VendorId,
    /// The vendor name
    pub name: String,
    /// The devices for this vendor
    pub devices: Vec<DeviceBuilder>,
}

/// Internal parser state for devices.
#[derive(Debug)]
#[allow(dead_code)]
pub struct DeviceBuilder {
    /// The device ID
    pub id: DeviceId,
    /// The device name
    pub name: String,
    /// The subsystems for this device
    pub subsystems: Vec<SubsystemBuilder>,
}

/// Internal parser state for subsystems.
#[derive(Debug)]
#[allow(dead_code)]
pub struct SubsystemBuilder {
    /// The subvendor ID
    pub subvendor_id: SubvendorId,
    /// The subdevice ID
    pub subdevice_id: SubdeviceId,
    /// The subsystem name
    pub name: String,
}

/// Internal parser state for device classes.
#[derive(Debug)]
#[allow(dead_code)]
pub struct ClassBuilder {
    /// The device class ID
    pub id: DeviceClassId,
    /// The device class name
    pub name: String,
    /// The subclasses for this device class
    pub subclasses: Vec<SubClassBuilder>,
}

/// Internal parser state for subclasses.
#[derive(Debug)]
#[allow(dead_code)]
pub struct SubClassBuilder {
    /// The subclass ID
    pub id: SubClassId,
    /// The subclass name
    pub name: String,
    /// The programming interfaces for this subclass
    pub prog_interfaces: Vec<ProgInterfaceBuilder>,
}

/// Internal parser state for programming interfaces.
#[derive(Debug)]
#[allow(dead_code)]
pub struct ProgInterfaceBuilder {
    /// The programming interface ID
    pub id: ProgInterfaceId,
    /// The programming interface name
    pub name: String,
}

/// Parser for the PCI IDs database format.
pub struct PciIdsParser {
    vendors: Vec<VendorBuilder>,
    classes: Vec<ClassBuilder>,
}

impl PciIdsParser {
    /// Create a new parser.
    pub fn new() -> Self {
        Self {
            vendors: Vec::new(),
            classes: Vec::new(),
        }
    }

    /// Parse the PCI IDs database content.
    ///
    /// The PCI IDs format is structured as follows:
    /// - Vendor lines start with 4 hex digits followed by two spaces and the vendor name
    /// - Device lines are indented with one tab, followed by 4 hex digits, two spaces, and device name
    /// - Subsystem lines are indented with two tabs, followed by two 4-digit hex values, two spaces, and subsystem name
    /// - Class lines start with "C " followed by 2 hex digits, two spaces, and class name
    /// - Subclass lines are indented with one tab, followed by 2 hex digits, two spaces, and subclass name
    /// - Programming interface lines are indented with two tabs, followed by 2 hex digits, two spaces, and interface name
    /// - Comments start with "#" and are ignored
    /// - Empty lines are ignored
    pub fn parse(&mut self, content: &str) -> PciResult<()> {
        self.vendors.clear();
        self.classes.clear();

        let mut current_vendor: Option<VendorBuilder> = None;
        let mut current_device: Option<DeviceBuilder> = None;
        let mut current_class: Option<ClassBuilder> = None;
        let mut current_subclass: Option<SubClassBuilder> = None;
        let mut parsing_mode = ParsingMode::Vendors;

        for (_line_num, line) in content.lines().enumerate() {
            // Skip empty lines and comments
            if line.trim().is_empty() || line.trim().starts_with('#') {
                continue;
            }

            // Check for section transitions
            if line.trim().starts_with("C ") && count_leading_tabs(line) == 0 {
                // Switch to classes mode
                parsing_mode = ParsingMode::Classes;

                // Finalize any remaining vendor/device
                self.finalize_vendor_device(&mut current_vendor, &mut current_device)?;
            } else if count_leading_tabs(line) == 0 && !line.trim().starts_with("C ") && parsing_mode == ParsingMode::Classes {
                // Check if this looks like a vendor line (4 hex digits followed by two spaces)
                if line.trim().len() >= 6 && line.trim().chars().nth(4) == Some(' ') && line.trim().chars().nth(5) == Some(' ') {
                    let hex_part = &line.trim()[..4];
                    if hex_part.chars().all(|c| c.is_ascii_hexdigit()) {
                        // Switch back to vendors mode
                        parsing_mode = ParsingMode::Vendors;

                        // Finalize any remaining class/subclass
                        self.finalize_class_subclass(&mut current_class, &mut current_subclass)?;
                    }
                }
            }

            let indentation = count_leading_tabs(line);
            let trimmed = line.trim();

            let result = match parsing_mode {
                ParsingMode::Vendors => self.parse_vendor_section(
                    trimmed,
                    indentation,
                    &mut current_vendor,
                    &mut current_device,
                ),
                ParsingMode::Classes => self.parse_class_section(
                    trimmed,
                    indentation,
                    &mut current_class,
                    &mut current_subclass,
                ),
            };

            if let Err(e) = result {
                // Add line number context to error (note: no_std doesn't have eprintln!)
                return Err(e);
            }
        }

        // Finalize any remaining items
        self.finalize_vendor_device(&mut current_vendor, &mut current_device)?;
        self.finalize_class_subclass(&mut current_class, &mut current_subclass)?;

        Ok(())
    }

    fn parse_vendor_section(
        &mut self,
        trimmed: &str,
        indentation: usize,
        current_vendor: &mut Option<VendorBuilder>,
        current_device: &mut Option<DeviceBuilder>,
    ) -> PciResult<()> {
        match indentation {
            0 => {
                // Vendor definition (XXXX  Name)
                self.finalize_vendor_device(current_vendor, current_device)?;

                let (id, name) = parse_vendor_line(trimmed)?;
                *current_vendor = Some(VendorBuilder {
                    id,
                    name,
                    devices: Vec::new(),
                });
            }
            1 => {
                // Device definition (\tXXXX  Name)
                if let Some(device) = current_device.take() {
                    if let Some(ref mut vendor) = current_vendor {
                        vendor.devices.push(device);
                    }
                }

                let (id, name) = parse_device_line(trimmed)?;
                *current_device = Some(DeviceBuilder {
                    id,
                    name,
                    subsystems: Vec::new(),
                });
            }
            2 => {
                // Subsystem definition (\t\tXXXX XXXX  Name)
                if let Some(ref mut device) = current_device {
                    let (subvendor_id, subdevice_id, name) = parse_subsystem_line(trimmed)?;
                    device.subsystems.push(SubsystemBuilder {
                        subvendor_id,
                        subdevice_id,
                        name,
                    });
                }
            }
            _ => {
                return Err(PciError::InvalidIndentation);
            }
        }
        Ok(())
    }

    fn parse_class_section(
        &mut self,
        trimmed: &str,
        indentation: usize,
        current_class: &mut Option<ClassBuilder>,
        current_subclass: &mut Option<SubClassBuilder>,
    ) -> PciResult<()> {
        match indentation {
            0 => {
                // Class definition (C XX  Name)
                self.finalize_class_subclass(current_class, current_subclass)?;

                if trimmed.starts_with("C ") {
                    let (id, name) = parse_class_line(trimmed)?;
                    *current_class = Some(ClassBuilder {
                        id,
                        name,
                        subclasses: Vec::new(),
                    });
                }
            }
            1 => {
                // Subclass definition (\tXX  Name)
                if let Some(subclass) = current_subclass.take() {
                    if let Some(ref mut class) = current_class {
                        class.subclasses.push(subclass);
                    }
                }

                let (id, name) = parse_subclass_line(trimmed)?;
                *current_subclass = Some(SubClassBuilder {
                    id,
                    name,
                    prog_interfaces: Vec::new(),
                });
            }
            2 => {
                // Programming interface definition (\t\tXX  Name)
                if let Some(ref mut subclass) = current_subclass {
                    let (id, name) = parse_prog_interface_line(trimmed)?;
                    subclass.prog_interfaces.push(ProgInterfaceBuilder { id, name });
                }
            }
            _ => {
                return Err(PciError::InvalidIndentation);
            }
        }
        Ok(())
    }

    fn finalize_vendor_device(
        &mut self,
        current_vendor: &mut Option<VendorBuilder>,
        current_device: &mut Option<DeviceBuilder>,
    ) -> PciResult<()> {
        if let Some(device) = current_device.take() {
            if let Some(ref mut vendor) = current_vendor {
                vendor.devices.push(device);
            }
        }

        if let Some(vendor) = current_vendor.take() {
            self.vendors.push(vendor);
        }

        Ok(())
    }

    fn finalize_class_subclass(
        &mut self,
        current_class: &mut Option<ClassBuilder>,
        current_subclass: &mut Option<SubClassBuilder>,
    ) -> PciResult<()> {
        if let Some(subclass) = current_subclass.take() {
            if let Some(ref mut class) = current_class {
                class.subclasses.push(subclass);
            }
        }

        if let Some(class) = current_class.take() {
            self.classes.push(class);
        }

        Ok(())
    }

    /// Get the parsed vendors (for use by build scripts and tests).
    #[allow(dead_code)]
    pub fn vendors(&self) -> &[VendorBuilder] {
        &self.vendors
    }

    /// Get the parsed classes (for use by build scripts and tests).
    #[allow(dead_code)]
    pub fn classes(&self) -> &[ClassBuilder] {
        &self.classes
    }

    /// Generate Rust code for the parsed database.
    pub fn generate_code(&self) -> String {
        let mut code = String::new();

        // Generate vendor data
        code.push_str("// Generated PCI vendor and device data\n");
        code.push_str("use crate::vendors::Vendor;\n");
        code.push_str("use crate::devices::{Device, Subsystem};\n");
        code.push_str("use crate::classes::{DeviceClass, SubClass, ProgInterface};\n");
        code.push_str("use crate::types::*;\n\n");

        // Generate static arrays for all data structures
        // This will be used by the build script to generate the actual database

        code
    }
}

impl Default for PciIdsParser {
    fn default() -> Self {
        Self::new()
    }
}

/// Count the number of leading tabs in a line.
fn count_leading_tabs(line: &str) -> usize {
    line.chars().take_while(|&c| c == '\t').count()
}

/// Parse a vendor line: "XXXX  Name"
fn parse_vendor_line(line: &str) -> PciResult<(VendorId, String)> {
    let parts: Vec<&str> = line.splitn(2, "  ").collect();

    if parts.len() != 2 {
        return Err(PciError::InvalidFormat);
    }

    let id = parse_hex_u16(parts[0])?;
    let name = parts[1].trim().to_string();

    Ok((VendorId::new(id), name))
}

/// Parse a device line: "XXXX  Name"
fn parse_device_line(line: &str) -> PciResult<(DeviceId, String)> {
    let parts: Vec<&str> = line.splitn(2, "  ").collect();

    if parts.len() != 2 {
        return Err(PciError::InvalidFormat);
    }

    let id = parse_hex_u16(parts[0])?;
    let name = parts[1].trim().to_string();

    Ok((DeviceId::new(id), name))
}

/// Parse a subsystem line: "XXXX XXXX  Name"
fn parse_subsystem_line(line: &str) -> PciResult<(SubvendorId, SubdeviceId, String)> {
    let parts: Vec<&str> = line.splitn(2, "  ").collect();

    if parts.len() != 2 {
        return Err(PciError::InvalidFormat);
    }

    let ids: Vec<&str> = parts[0].split_whitespace().collect();
    if ids.len() != 2 {
        return Err(PciError::InvalidFormat);
    }

    let subvendor_id = parse_hex_u16(ids[0])?;
    let subdevice_id = parse_hex_u16(ids[1])?;
    let name = parts[1].trim().to_string();

    Ok((SubvendorId::new(subvendor_id), SubdeviceId::new(subdevice_id), name))
}

/// Parse a class line: "C XX  Name"
fn parse_class_line(line: &str) -> PciResult<(DeviceClassId, String)> {
    if !line.starts_with("C ") {
        return Err(PciError::InvalidFormat);
    }

    let rest = &line[2..]; // Skip "C "
    let parts: Vec<&str> = rest.splitn(2, "  ").collect();

    if parts.len() != 2 {
        return Err(PciError::InvalidFormat);
    }

    let id = parse_hex_u8(parts[0])?;
    let name = parts[1].trim().to_string();

    Ok((DeviceClassId::new(id), name))
}

/// Parse a subclass line: "XX  Name"
fn parse_subclass_line(line: &str) -> PciResult<(SubClassId, String)> {
    let parts: Vec<&str> = line.splitn(2, "  ").collect();

    if parts.len() != 2 {
        return Err(PciError::InvalidFormat);
    }

    let id = parse_hex_u8(parts[0])?;
    let name = parts[1].trim().to_string();

    Ok((SubClassId::new(id), name))
}

/// Parse a programming interface line: "XX  Name"
fn parse_prog_interface_line(line: &str) -> PciResult<(ProgInterfaceId, String)> {
    let parts: Vec<&str> = line.splitn(2, "  ").collect();

    if parts.len() != 2 {
        return Err(PciError::InvalidFormat);
    }

    let id = parse_hex_u8(parts[0])?;
    let name = parts[1].trim().to_string();

    Ok((ProgInterfaceId::new(id), name))
}

/// Parse a hexadecimal string to u16.
fn parse_hex_u16(hex_str: &str) -> PciResult<u16> {
    u16::from_str_radix(hex_str.trim(), 16).map_err(|_| PciError::InvalidHexValue)
}

/// Parse a hexadecimal string to u8.
fn parse_hex_u8(hex_str: &str) -> PciResult<u8> {
    u8::from_str_radix(hex_str.trim(), 16).map_err(|_| PciError::InvalidHexValue)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_vendor_parsing() {
        let content = r#"
# Test PCI IDs
1234  Test Vendor
	5678  Test Device
		abcd 1234  Test Subsystem
"#;

        let mut parser = PciIdsParser::new();
        parser.parse(content).expect("Failed to parse");

        assert_eq!(parser.vendors.len(), 1);
        let vendor = &parser.vendors[0];
        assert_eq!(vendor.id.value(), 0x1234);
        assert_eq!(vendor.name, "Test Vendor");
        assert_eq!(vendor.devices.len(), 1);

        let device = &vendor.devices[0];
        assert_eq!(device.id.value(), 0x5678);
        assert_eq!(device.name, "Test Device");
        assert_eq!(device.subsystems.len(), 1);

        let subsystem = &device.subsystems[0];
        assert_eq!(subsystem.subvendor_id.value(), 0xabcd);
        assert_eq!(subsystem.subdevice_id.value(), 0x1234);
        assert_eq!(subsystem.name, "Test Subsystem");
    }

    #[test]
    fn test_basic_class_parsing() {
        let content = r#"
C 02  Network controller
	00  Ethernet controller
	01  Token ring network controller
		00  Basic token ring
		01  Advanced token ring
C 03  Display controller
	00  VGA compatible controller
		00  VGA controller
		01  8514 controller
"#;

        let mut parser = PciIdsParser::new();
        parser.parse(content).expect("Failed to parse");

        assert_eq!(parser.classes.len(), 2);

        // Test Network controller class
        let network_class = &parser.classes[0];
        assert_eq!(network_class.id.value(), 0x02);
        assert_eq!(network_class.name, "Network controller");
        assert_eq!(network_class.subclasses.len(), 2);

        let ethernet_subclass = &network_class.subclasses[0];
        assert_eq!(ethernet_subclass.id.value(), 0x00);
        assert_eq!(ethernet_subclass.name, "Ethernet controller");
        assert_eq!(ethernet_subclass.prog_interfaces.len(), 0);

        let token_ring_subclass = &network_class.subclasses[1];
        assert_eq!(token_ring_subclass.id.value(), 0x01);
        assert_eq!(token_ring_subclass.name, "Token ring network controller");
        assert_eq!(token_ring_subclass.prog_interfaces.len(), 2);

        // Test Display controller class
        let display_class = &parser.classes[1];
        assert_eq!(display_class.id.value(), 0x03);
        assert_eq!(display_class.name, "Display controller");
        assert_eq!(display_class.subclasses.len(), 1);
    }

    #[test]
    fn test_mixed_parsing() {
        let content = r#"
1234  Test Vendor
	5678  Test Device
C 02  Network controller
	00  Ethernet controller
"#;

        let mut parser = PciIdsParser::new();
        parser.parse(content).expect("Failed to parse");

        assert_eq!(parser.vendors.len(), 1);
        assert_eq!(parser.classes.len(), 1);
    }
}