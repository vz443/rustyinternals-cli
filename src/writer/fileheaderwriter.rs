use crate::headers::fileheader::{FileHeader, Characteristics};
use super::write_utils;

const FIELD_WIDTH: usize = 30;

pub fn write_file_header(_header: &FileHeader) {
    write_utils::print_header_title("FILE HEADER (COFF)");
    write_utils::print_info("Note: FileHeader fields are private - writer needs getter methods");
    write_utils::print_section_separator();
}

pub fn write_file_header_parsed(
    machine: u16,
    number_of_sections: u16,
    time_date_stamp: u32,
    pointer_to_symbol_table: u32,
    number_of_symbols: u32,
    size_of_optional_header: u16,
    characteristics: Characteristics,
) {
    write_utils::print_header_title("FILE HEADER (COFF)");
    
    write_utils::print_field_u16("Machine:", machine, FIELD_WIDTH);
    print_machine_type(machine);
    write_utils::print_field_u16("NumberOfSections:", number_of_sections, FIELD_WIDTH);
    write_utils::print_field_u32("TimeDateStamp:", time_date_stamp, FIELD_WIDTH);
    if let Some(datetime) = timestamp_to_string(time_date_stamp) {
        write_utils::print_info(&format!("{}", datetime));
    }
    write_utils::print_field_u32("PointerToSymbolTable:", pointer_to_symbol_table, FIELD_WIDTH);
    write_utils::print_field_u32("NumberOfSymbols:", number_of_symbols, FIELD_WIDTH);
    write_utils::print_field_u16("SizeOfOptionalHeader:", size_of_optional_header, FIELD_WIDTH);
    write_utils::print_subsection("Characteristics:");
    print_characteristics(characteristics);
    
    write_utils::print_section_separator();
}

fn print_machine_type(machine: u16) {
    let machine_str = match machine {
        0x014c => "IMAGE_FILE_MACHINE_I386 (x86)",
        0x0200 => "IMAGE_FILE_MACHINE_IA64 (Intel Itanium)",
        0x8664 => "IMAGE_FILE_MACHINE_AMD64 (x64)",
        0x01c0 => "IMAGE_FILE_MACHINE_ARM",
        0xaa64 => "IMAGE_FILE_MACHINE_ARM64",
        _ => "Unknown machine type",
    };
    write_utils::print_info(machine_str);
}

fn timestamp_to_string(timestamp: u32) -> Option<String> {
    if timestamp == 0 {
        return None;
    }
    
    use std::time::{UNIX_EPOCH, Duration};
    
    let duration = Duration::from_secs(timestamp as u64);
    let _datetime = UNIX_EPOCH + duration;
    
    Some(format!("Unix timestamp: {} seconds since epoch", timestamp))
}

fn print_characteristics(characteristics: Characteristics) {
    if characteristics.is_empty() {
        write_utils::print_flag("(None)");
        return;
    }
    
    for flag in characteristics.iter() {
        match flag {
            Characteristics::IMAGE_FILE_RELOCS_STRIPPED => {
                write_utils::print_flag("IMAGE_FILE_RELOCS_STRIPPED");
            }
            Characteristics::IMAGE_FILE_EXECUTABLE_IMAGE => {
                write_utils::print_flag("IMAGE_FILE_EXECUTABLE_IMAGE");
            }
            Characteristics::IMAGE_FILE_LINE_NUMS_STRIPPED => {
                write_utils::print_flag("IMAGE_FILE_LINE_NUMS_STRIPPED");
            }
            Characteristics::IMAGE_FILE_LOCAL_SYMS_STRIPPED => {
                write_utils::print_flag("IMAGE_FILE_LOCAL_SYMS_STRIPPED");
            }
            Characteristics::IMAGE_FILE_AGGRESSIVE_WS_TRIM => {
                write_utils::print_flag("IMAGE_FILE_AGGRESSIVE_WS_TRIM");
            }
            Characteristics::IMAGE_FILE_LARGE_ADDRESS_AWARE => {
                write_utils::print_flag("IMAGE_FILE_LARGE_ADDRESS_AWARE");
            }
            Characteristics::IMAGE_FILE_RESERVED_0040 => {
                write_utils::print_flag("IMAGE_FILE_RESERVED_0040");
            }
            Characteristics::IMAGE_FILE_BYTES_REVERSED_LO => {
                write_utils::print_flag("IMAGE_FILE_BYTES_REVERSED_LO");
            }
            Characteristics::IMAGE_FILE_32BIT_MACHINE => {
                write_utils::print_flag("IMAGE_FILE_32BIT_MACHINE");
            }
            Characteristics::IMAGE_FILE_DEBUG_STRIPPED => {
                write_utils::print_flag("IMAGE_FILE_DEBUG_STRIPPED");
            }
            Characteristics::IMAGE_FILE_REMOVABLE_RUN_FROM_SWAP => {
                write_utils::print_flag("IMAGE_FILE_REMOVABLE_RUN_FROM_SWAP");
            }
            Characteristics::IMAGE_FILE_NET_RUN_FROM_SWAP => {
                write_utils::print_flag("IMAGE_FILE_NET_RUN_FROM_SWAP");
            }
            Characteristics::IMAGE_FILE_SYSTEM => {
                write_utils::print_flag("IMAGE_FILE_SYSTEM");
            }
            Characteristics::IMAGE_FILE_DLL => {
                write_utils::print_flag("IMAGE_FILE_DLL");
            }
            Characteristics::IMAGE_FILE_UP_SYSTEM_ONLY => {
                write_utils::print_flag("IMAGE_FILE_UP_SYSTEM_ONLY");
            }
            Characteristics::IMAGE_FILE_BYTES_REVERSED_HI => {
                write_utils::print_flag("IMAGE_FILE_BYTES_REVERSED_HI");
            }
            _ => {
                write_utils::print_flag(&format!("UNKNOWN_FLAG: 0x{:04X}", flag.bits()));
            }
        }
    }
}
