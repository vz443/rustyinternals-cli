use crate::headers::optionalheader::OptionalHeader;
use super::write_utils;

const FIELD_WIDTH: usize = 35;

pub fn write_optional_header(_header: &OptionalHeader) {
    write_utils::print_header_title("OPTIONAL HEADER");
    write_utils::print_info("Note: OptionalHeader fields are private - writer needs getter methods");
    write_utils::print_section_separator();
}

#[allow(clippy::too_many_arguments)]
pub fn write_optional_header_parsed(
    magic: u16,
    major_linker_version: u8,
    minor_linker_version: u8,
    size_of_code: u32,
    size_of_initialized_data: u32,
    size_of_uninitialized_data: u32,
    address_of_entry_point: u32,
    base_of_code: u32,
    base_of_data: u32,
    image_base: u32,
    section_alignment: u32,
    file_alignment: u32,
    major_operating_system_version: u16,
    minor_operating_system_version: u16,
    major_image_version: u16,
    minor_image_version: u16,
    major_subsystem_version: u16,
    minor_subsystem_version: u16,
    win32_version_value: u32,
    size_of_image: u32,
    size_of_headers: u32,
    checksum: u32,
    subsystem: u16,
    dll_characteristics: u16,
    size_of_stack_reserve: u32,
    size_of_stack_commit: u32,
    size_of_heap_reserve: u32,
    size_of_heap_commit: u32,
    loader_flags: u32,
    number_of_rva_and_sizes: u32,
) {
    write_utils::print_header_title("OPTIONAL HEADER");
    
    write_utils::print_field_u16("Magic:", magic, FIELD_WIDTH);
    print_magic_type(magic);
    write_utils::print_field_u8("MajorLinkerVersion:", major_linker_version, FIELD_WIDTH);
    write_utils::print_field_u8("MinorLinkerVersion:", minor_linker_version, FIELD_WIDTH);
    write_utils::print_field_u32("SizeOfCode:", size_of_code, FIELD_WIDTH);
    write_utils::print_field_u32("SizeOfInitializedData:", size_of_initialized_data, FIELD_WIDTH);
    write_utils::print_field_u32("SizeOfUninitializedData:", size_of_uninitialized_data, FIELD_WIDTH);
    write_utils::print_field_u32("AddressOfEntryPoint:", address_of_entry_point, FIELD_WIDTH);
    write_utils::print_field_u32("BaseOfCode:", base_of_code, FIELD_WIDTH);
    write_utils::print_field_u32("BaseOfData:", base_of_data, FIELD_WIDTH);
    write_utils::print_field_u32("ImageBase:", image_base, FIELD_WIDTH);
    write_utils::print_field_u32("SectionAlignment:", section_alignment, FIELD_WIDTH);
    write_utils::print_field_u32("FileAlignment:", file_alignment, FIELD_WIDTH);
    write_utils::print_subsection("Operating System Version:");
    write_utils::print_field_u16("  Major:", major_operating_system_version, FIELD_WIDTH - 2);
    write_utils::print_field_u16("  Minor:", minor_operating_system_version, FIELD_WIDTH - 2);
    write_utils::print_subsection("Image Version:");
    write_utils::print_field_u16("  Major:", major_image_version, FIELD_WIDTH - 2);
    write_utils::print_field_u16("  Minor:", minor_image_version, FIELD_WIDTH - 2);
    
    // Subsystem version
    write_utils::print_subsection("Subsystem Version:");
    write_utils::print_field_u16("  Major:", major_subsystem_version, FIELD_WIDTH - 2);
    write_utils::print_field_u16("  Minor:", minor_subsystem_version, FIELD_WIDTH - 2);
    
    // Win32 version value
    write_utils::print_field_u32("Win32VersionValue:", win32_version_value, FIELD_WIDTH);
    
    // Image and header sizes
    write_utils::print_field_u32("SizeOfImage:", size_of_image, FIELD_WIDTH);
    write_utils::print_field_u32("SizeOfHeaders:", size_of_headers, FIELD_WIDTH);
    
    // Checksum
    write_utils::print_field_u32("CheckSum:", checksum, FIELD_WIDTH);
    write_utils::print_field_u16("Subsystem:", subsystem, FIELD_WIDTH);
    print_subsystem_type(subsystem);
    write_utils::print_field_u16("DLLCharacteristics:", dll_characteristics, FIELD_WIDTH);
    print_dll_characteristics(dll_characteristics);
    write_utils::print_subsection("Stack and Heap:");
    write_utils::print_field_u32("  SizeOfStackReserve:", size_of_stack_reserve, FIELD_WIDTH - 2);
    write_utils::print_field_u32("  SizeOfStackCommit:", size_of_stack_commit, FIELD_WIDTH - 2);
    write_utils::print_field_u32("  SizeOfHeapReserve:", size_of_heap_reserve, FIELD_WIDTH - 2);
    write_utils::print_field_u32("  SizeOfHeapCommit:", size_of_heap_commit, FIELD_WIDTH - 2);
    write_utils::print_field_u32("LoaderFlags:", loader_flags, FIELD_WIDTH);
    write_utils::print_field_u32("NumberOfRvaAndSizes:", number_of_rva_and_sizes, FIELD_WIDTH);
    
    write_utils::print_section_separator();
}

fn print_magic_type(magic: u16) {
    let magic_str = match magic {
        0x010b => "PE32 (32-bit executable)",
        0x020b => "PE32+ (64-bit executable)",
        0x0107 => "ROM image",
        _ => "Unknown format",
    };
    write_utils::print_info(magic_str);
}

fn print_subsystem_type(subsystem: u16) {
    let subsystem_str = match subsystem {
        0 => "IMAGE_SUBSYSTEM_UNKNOWN",
        1 => "IMAGE_SUBSYSTEM_NATIVE",
        2 => "IMAGE_SUBSYSTEM_WINDOWS_GUI",
        3 => "IMAGE_SUBSYSTEM_WINDOWS_CUI (Console)",
        5 => "IMAGE_SUBSYSTEM_OS2_CUI",
        7 => "IMAGE_SUBSYSTEM_POSIX_CUI",
        9 => "IMAGE_SUBSYSTEM_WINDOWS_CE_GUI",
        10 => "IMAGE_SUBSYSTEM_EFI_APPLICATION",
        11 => "IMAGE_SUBSYSTEM_EFI_BOOT_SERVICE_DRIVER",
        12 => "IMAGE_SUBSYSTEM_EFI_RUNTIME_DRIVER",
        13 => "IMAGE_SUBSYSTEM_EFI_ROM",
        14 => "IMAGE_SUBSYSTEM_XBOX",
        16 => "IMAGE_SUBSYSTEM_WINDOWS_BOOT_APPLICATION",
        _ => "Unknown subsystem",
    };
    write_utils::print_info(subsystem_str);
}

fn print_dll_characteristics(characteristics: u16) {
    if characteristics == 0 {
        write_utils::print_info("(None)");
        return;
    }
    
    write_utils::print_subsection("DLL Characteristics Flags:");
    
    if characteristics & 0x0020 != 0 {
        write_utils::print_flag("IMAGE_DLLCHARACTERISTICS_HIGH_ENTROPY_VA");
    }
    if characteristics & 0x0040 != 0 {
        write_utils::print_flag("IMAGE_DLLCHARACTERISTICS_DYNAMIC_BASE");
    }
    if characteristics & 0x0080 != 0 {
        write_utils::print_flag("IMAGE_DLLCHARACTERISTICS_FORCE_INTEGRITY");
    }
    if characteristics & 0x0100 != 0 {
        write_utils::print_flag("IMAGE_DLLCHARACTERISTICS_NX_COMPAT");
    }
    if characteristics & 0x0200 != 0 {
        write_utils::print_flag("IMAGE_DLLCHARACTERISTICS_NO_ISOLATION");
    }
    if characteristics & 0x0400 != 0 {
        write_utils::print_flag("IMAGE_DLLCHARACTERISTICS_NO_SEH");
    }
    if characteristics & 0x0800 != 0 {
        write_utils::print_flag("IMAGE_DLLCHARACTERISTICS_NO_BIND");
    }
    if characteristics & 0x1000 != 0 {
        write_utils::print_flag("IMAGE_DLLCHARACTERISTICS_APPCONTAINER");
    }
    if characteristics & 0x2000 != 0 {
        write_utils::print_flag("IMAGE_DLLCHARACTERISTICS_WDM_DRIVER");
    }
    if characteristics & 0x4000 != 0 {
        write_utils::print_flag("IMAGE_DLLCHARACTERISTICS_GUARD_CF");
    }
    if characteristics & 0x8000 != 0 {
        write_utils::print_flag("IMAGE_DLLCHARACTERISTICS_TERMINAL_SERVER_AWARE");
    }
}
