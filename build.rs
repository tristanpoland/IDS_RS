use std::env;
use std::fs;
use std::path::Path;

fn main() {
    println!("cargo:rerun-if-changed=pci.ids");
    println!("cargo:rerun-if-changed=build.rs");

    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("pci_database.rs");

    // Check if pci.ids file exists
    let pci_ids_path = "pci.ids";
    if !Path::new(pci_ids_path).exists() {
        eprintln!("Warning: pci.ids file not found. Please run the update script first:");
        eprintln!("  PowerShell: .\\update_pci_ids.ps1");
        eprintln!("  Bash: ./update_pci_ids.sh");
        eprintln!("Creating empty database...");

        let empty_database = generate_empty_database();
        fs::write(&dest_path, empty_database).unwrap();
        return;
    }

    // Read the PCI IDs file
    let content = match fs::read_to_string(pci_ids_path) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Error reading pci.ids: {}", e);
            eprintln!("Creating empty database...");
            let empty_database = generate_empty_database();
            fs::write(&dest_path, empty_database).unwrap();
            return;
        }
    };

    // Parse the content and generate database
    match parse_and_generate(&content) {
        Ok(database_code) => {
            fs::write(&dest_path, database_code).unwrap();
            println!("Generated PCI database successfully");
        }
        Err(e) => {
            eprintln!("Error parsing pci.ids: {}", e);
            eprintln!("Creating empty database...");
            let empty_database = generate_empty_database();
            fs::write(&dest_path, empty_database).unwrap();
        }
    }
}

fn generate_empty_database() -> String {
    r#"
// Empty PCI database (pci.ids file not found or failed to parse)

static VENDORS: &[crate::vendors::Vendor] = &[];
static CLASSES: &[crate::classes::DeviceClass] = &[];

/// The global PCI database instance.
pub static GLOBAL_DATABASE: crate::database::PciDatabase = crate::database::PciDatabase::new(VENDORS, CLASSES);
"#.to_string()
}

// Simple parser structures for build script
#[derive(Debug)]
struct Vendor {
    id: u16,
    name: String,
    devices: Vec<Device>,
}

#[derive(Debug)]
struct Device {
    id: u16,
    name: String,
    subsystems: Vec<Subsystem>,
}

#[derive(Debug)]
struct Subsystem {
    subvendor_id: u16,
    subdevice_id: u16,
    name: String,
}

#[derive(Debug)]
struct Class {
    id: u8,
    name: String,
    subclasses: Vec<SubClass>,
}

#[derive(Debug)]
struct SubClass {
    id: u8,
    name: String,
    prog_interfaces: Vec<ProgInterface>,
}

#[derive(Debug)]
struct ProgInterface {
    id: u8,
    name: String,
}

fn parse_and_generate(content: &str) -> Result<String, String> {
    let mut vendors = Vec::new();
    let mut classes = Vec::new();

    let mut current_vendor: Option<Vendor> = None;
    let mut current_device: Option<Device> = None;
    let mut current_class: Option<Class> = None;
    let mut current_subclass: Option<SubClass> = None;
    let mut parsing_mode = ParsingMode::Vendors;

    for line in content.lines() {
        // Skip empty lines and comments
        if line.trim().is_empty() || line.trim().starts_with('#') {
            continue;
        }

        // Check for section transitions
        if line.trim().starts_with("C ") && count_leading_tabs(line) == 0 {
            // Switch to classes mode
            parsing_mode = ParsingMode::Classes;

            // Finalize any remaining vendor/device
            finalize_vendor_device(&mut vendors, &mut current_vendor, &mut current_device);
        }

        let indentation = count_leading_tabs(line);
        let trimmed = line.trim();

        match parsing_mode {
            ParsingMode::Vendors => {
                match indentation {
                    0 => {
                        // Vendor definition (XXXX  Name)
                        finalize_vendor_device(&mut vendors, &mut current_vendor, &mut current_device);

                        let (id, name) = parse_vendor_line(trimmed)?;
                        current_vendor = Some(Vendor {
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
                        current_device = Some(Device {
                            id,
                            name,
                            subsystems: Vec::new(),
                        });
                    }
                    2 => {
                        // Subsystem definition (\t\tXXXX XXXX  Name)
                        if let Some(ref mut device) = current_device {
                            let (subvendor_id, subdevice_id, name) = parse_subsystem_line(trimmed)?;
                            device.subsystems.push(Subsystem {
                                subvendor_id,
                                subdevice_id,
                                name,
                            });
                        }
                    }
                    _ => {
                        return Err("Invalid indentation in vendor section".to_string());
                    }
                }
            }
            ParsingMode::Classes => {
                match indentation {
                    0 => {
                        // Class definition (C XX  Name)
                        finalize_class_subclass(&mut classes, &mut current_class, &mut current_subclass);

                        if trimmed.starts_with("C ") {
                            let (id, name) = parse_class_line(trimmed)?;
                            current_class = Some(Class {
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
                        current_subclass = Some(SubClass {
                            id,
                            name,
                            prog_interfaces: Vec::new(),
                        });
                    }
                    2 => {
                        // Programming interface definition (\t\tXX  Name)
                        if let Some(ref mut subclass) = current_subclass {
                            let (id, name) = parse_prog_interface_line(trimmed)?;
                            subclass.prog_interfaces.push(ProgInterface { id, name });
                        }
                    }
                    _ => {
                        return Err("Invalid indentation in class section".to_string());
                    }
                }
            }
        }
    }

    // Finalize any remaining items
    finalize_vendor_device(&mut vendors, &mut current_vendor, &mut current_device);
    finalize_class_subclass(&mut classes, &mut current_class, &mut current_subclass);

    Ok(generate_database_code(&vendors, &classes))
}

#[derive(Debug, Clone, Copy)]
enum ParsingMode {
    Vendors,
    Classes,
}

fn count_leading_tabs(line: &str) -> usize {
    line.chars().take_while(|&c| c == '\t').count()
}

fn finalize_vendor_device(
    vendors: &mut Vec<Vendor>,
    current_vendor: &mut Option<Vendor>,
    current_device: &mut Option<Device>,
) {
    if let Some(device) = current_device.take() {
        if let Some(ref mut vendor) = current_vendor {
            vendor.devices.push(device);
        }
    }

    if let Some(vendor) = current_vendor.take() {
        vendors.push(vendor);
    }
}

fn finalize_class_subclass(
    classes: &mut Vec<Class>,
    current_class: &mut Option<Class>,
    current_subclass: &mut Option<SubClass>,
) {
    if let Some(subclass) = current_subclass.take() {
        if let Some(ref mut class) = current_class {
            class.subclasses.push(subclass);
        }
    }

    if let Some(class) = current_class.take() {
        classes.push(class);
    }
}

fn parse_vendor_line(line: &str) -> Result<(u16, String), String> {
    let parts: Vec<&str> = line.splitn(2, "  ").collect();
    if parts.len() != 2 {
        return Err("Invalid vendor line format".to_string());
    }

    let id = u16::from_str_radix(parts[0].trim(), 16)
        .map_err(|_| "Invalid vendor ID".to_string())?;
    let name = parts[1].trim().to_string();

    Ok((id, name))
}

fn parse_device_line(line: &str) -> Result<(u16, String), String> {
    let parts: Vec<&str> = line.splitn(2, "  ").collect();
    if parts.len() != 2 {
        return Err("Invalid device line format".to_string());
    }

    let id = u16::from_str_radix(parts[0].trim(), 16)
        .map_err(|_| "Invalid device ID".to_string())?;
    let name = parts[1].trim().to_string();

    Ok((id, name))
}

fn parse_subsystem_line(line: &str) -> Result<(u16, u16, String), String> {
    let parts: Vec<&str> = line.splitn(2, "  ").collect();
    if parts.len() != 2 {
        return Err("Invalid subsystem line format".to_string());
    }

    let ids: Vec<&str> = parts[0].split_whitespace().collect();
    if ids.len() != 2 {
        return Err("Invalid subsystem ID format".to_string());
    }

    let subvendor_id = u16::from_str_radix(ids[0].trim(), 16)
        .map_err(|_| "Invalid subvendor ID".to_string())?;
    let subdevice_id = u16::from_str_radix(ids[1].trim(), 16)
        .map_err(|_| "Invalid subdevice ID".to_string())?;
    let name = parts[1].trim().to_string();

    Ok((subvendor_id, subdevice_id, name))
}

fn parse_class_line(line: &str) -> Result<(u8, String), String> {
    if !line.starts_with("C ") {
        return Err("Invalid class line format".to_string());
    }

    let rest = &line[2..]; // Skip "C "
    let parts: Vec<&str> = rest.splitn(2, "  ").collect();
    if parts.len() != 2 {
        return Err("Invalid class line format".to_string());
    }

    let id = u8::from_str_radix(parts[0].trim(), 16)
        .map_err(|_| "Invalid class ID".to_string())?;
    let name = parts[1].trim().to_string();

    Ok((id, name))
}

fn parse_subclass_line(line: &str) -> Result<(u8, String), String> {
    let parts: Vec<&str> = line.splitn(2, "  ").collect();
    if parts.len() != 2 {
        return Err("Invalid subclass line format".to_string());
    }

    let id = u8::from_str_radix(parts[0].trim(), 16)
        .map_err(|_| "Invalid subclass ID".to_string())?;
    let name = parts[1].trim().to_string();

    Ok((id, name))
}

fn parse_prog_interface_line(line: &str) -> Result<(u8, String), String> {
    let parts: Vec<&str> = line.splitn(2, "  ").collect();
    if parts.len() != 2 {
        return Err("Invalid programming interface line format".to_string());
    }

    let id = u8::from_str_radix(parts[0].trim(), 16)
        .map_err(|_| "Invalid programming interface ID".to_string())?;
    let name = parts[1].trim().to_string();

    Ok((id, name))
}

fn generate_database_code(vendors: &[Vendor], classes: &[Class]) -> String {
    let mut code = String::new();

    code.push_str("// Generated PCI database from pci.ids\n");
    code.push_str("// This file is automatically generated by the build script\n\n");

    // Generate subsystem data
    for vendor in vendors {
        for device in &vendor.devices {
            if !device.subsystems.is_empty() {
                code.push_str(&format!(
                    "static SUBSYSTEMS_{}_{}: &[Subsystem] = &[\n",
                    vendor.id, device.id
                ));
                for subsystem in &device.subsystems {
                    code.push_str(&format!(
                        "    crate::devices::Subsystem::new(crate::types::SubvendorId::new(0x{:04x}), crate::types::SubdeviceId::new(0x{:04x}), {:?}),\n",
                        subsystem.subvendor_id, subsystem.subdevice_id, subsystem.name
                    ));
                }
                code.push_str("];\n\n");
            }
        }
    }

    // Generate device data
    for vendor in vendors {
        if !vendor.devices.is_empty() {
            code.push_str(&format!("static DEVICES_{}: &[crate::devices::Device] = &[\n", vendor.id));
            for device in &vendor.devices {
                let subsystems_ref = if device.subsystems.is_empty() {
                    "&[]".to_string()
                } else {
                    format!("SUBSYSTEMS_{}_{}", vendor.id, device.id)
                };

                code.push_str(&format!(
                    "    crate::devices::Device::new(crate::types::DeviceId::new(0x{:04x}), {:?}, {}),\n",
                    device.id, device.name, subsystems_ref
                ));
            }
            code.push_str("];\n\n");
        }
    }

    // Generate vendor data
    code.push_str("static VENDORS: &[crate::vendors::Vendor] = &[\n");
    for vendor in vendors {
        let devices_ref = if vendor.devices.is_empty() {
            "&[]".to_string()
        } else {
            format!("DEVICES_{}", vendor.id)
        };

        code.push_str(&format!(
            "    crate::vendors::Vendor::new(crate::types::VendorId::new(0x{:04x}), {:?}, {}),\n",
            vendor.id, vendor.name, devices_ref
        ));
    }
    code.push_str("];\n\n");

    // Generate programming interface data
    for class in classes {
        for subclass in &class.subclasses {
            if !subclass.prog_interfaces.is_empty() {
                code.push_str(&format!(
                    "static PROG_INTERFACES_{}_{}: &[crate::classes::ProgInterface] = &[\n",
                    class.id, subclass.id
                ));
                for prog_if in &subclass.prog_interfaces {
                    code.push_str(&format!(
                        "    crate::classes::ProgInterface::new(crate::types::ProgInterfaceId::new(0x{:02x}), {:?}),\n",
                        prog_if.id, prog_if.name
                    ));
                }
                code.push_str("];\n\n");
            }
        }
    }

    // Generate subclass data
    for class in classes {
        if !class.subclasses.is_empty() {
            code.push_str(&format!(
                "static SUBCLASSES_{}: &[crate::classes::SubClass] = &[\n",
                class.id
            ));
            for subclass in &class.subclasses {
                let prog_interfaces_ref = if subclass.prog_interfaces.is_empty() {
                    "&[]".to_string()
                } else {
                    format!("PROG_INTERFACES_{}_{}", class.id, subclass.id)
                };

                code.push_str(&format!(
                    "    crate::classes::SubClass::new(crate::types::SubClassId::new(0x{:02x}), {:?}, {}),\n",
                    subclass.id, subclass.name, prog_interfaces_ref
                ));
            }
            code.push_str("];\n\n");
        }
    }

    // Generate class data
    code.push_str("static CLASSES: &[crate::classes::DeviceClass] = &[\n");
    for class in classes {
        let subclasses_ref = if class.subclasses.is_empty() {
            "&[]".to_string()
        } else {
            format!("SUBCLASSES_{}", class.id)
        };

        code.push_str(&format!(
            "    crate::classes::DeviceClass::new(crate::types::DeviceClassId::new(0x{:02x}), {:?}, {}),\n",
            class.id, class.name, subclasses_ref
        ));
    }
    code.push_str("];\n\n");

    // Generate the global database
    code.push_str("/// The global PCI database instance.\n");
    code.push_str("pub static GLOBAL_DATABASE: crate::database::PciDatabase = crate::database::PciDatabase::new(VENDORS, CLASSES);\n");

    code
}