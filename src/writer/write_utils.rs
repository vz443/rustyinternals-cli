const SEPARATOR: &str = "========================================";
const MINOR_SEPARATOR: &str = "----------------------------------------";

pub fn print_header_title(title: &str) {
    println!("\n{}", SEPARATOR);
    println!("  {}", title);
    println!("{}", SEPARATOR);
}

pub fn print_section_separator() {
    println!("{}", MINOR_SEPARATOR);
}

pub fn print_field_u8(name: &str, value: u8, width: usize) {
    println!("{:<width$} 0x{:02X}", name, value, width = width);
}

pub fn print_field_u16(name: &str, value: u16, width: usize) {
    println!("{:<width$} 0x{:04X}", name, value, width = width);
}

pub fn print_field_u32(name: &str, value: u32, width: usize) {
    println!("{:<width$} 0x{:08X}", name, value, width = width);
}

pub fn print_field_u64(name: &str, value: u64, width: usize) {
    println!("{:<width$} 0x{:016X}", name, value, width = width);
}

pub fn print_field_decimal(name: &str, value: impl std::fmt::Display, width: usize) {
    println!("{:<width$} {}", name, value, width = width);
}

pub fn print_field_bool(name: &str, value: bool, width: usize) {
    println!("{:<width$} {}", name, if value { "Yes" } else { "No" }, width = width);
}

pub fn print_array_u16(name: &str, values: &[u16]) {
    print!("{}  [", name);
    for (i, val) in values.iter().enumerate() {
        if i < values.len() - 1 {
            print!("0x{:04X}, ", val);
        } else {
            print!("0x{:04X}", val);
        }
    }
    println!("]");
}

pub fn print_array_u32(name: &str, values: &[u32]) {
    print!("{}  [", name);
    for (i, val) in values.iter().enumerate() {
        if i < values.len() - 1 {
            print!("0x{:08X}, ", val);
        } else {
            print!("0x{:08X}", val);
        }
    }
    println!("]");
}

pub fn print_validation(message: &str, is_valid: bool) {
    let status = if is_valid { "✓" } else { "✗" };
    println!("{} {}", status, message);
}

pub fn print_info(message: &str) {
    println!("  ℹ {}", message);
}

pub fn print_warning(message: &str) {
    println!("  ⚠ WARNING: {}", message);
}

pub fn print_error(message: &str) {
    println!("  ✗ ERROR: {}", message);
}

pub fn print_flag(flag_name: &str) {
    println!("    • {}", flag_name);
}

pub fn print_subsection(title: &str) {
    println!("\n  {}", title);
}
