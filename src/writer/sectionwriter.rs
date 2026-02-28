use crate::sections::sections::{Section, Sections};
use super::write_utils;

const FIELD_WIDTH: usize = 30;

pub fn write_sections(sections: &Sections) {
    write_utils::print_header_title("SECTION HEADERS");
    write_utils::print_field_decimal("Number of sections:", sections.sections.len(), FIELD_WIDTH);

    for (i, section) in sections.sections.iter().enumerate() {
        write_section(i, section);
    }
}

fn write_section(index: usize, section: &Section) {
    let name = section_name_to_string(&section.name);
    write_utils::print_subsection(&format!("Section [{}]: {}", index, name));

    write_utils::print_field_decimal("Name:", &name, FIELD_WIDTH);
    write_utils::print_field_u32("VirtualSize:", section.virtual_size, FIELD_WIDTH);
    write_utils::print_field_u32("VirtualAddress:", section.virtual_address, FIELD_WIDTH);
    write_utils::print_field_u32("SizeOfRawData:", section.size_of_raw_data, FIELD_WIDTH);
    write_utils::print_field_u32("PointerToRawData:", section.pointer_to_raw_data, FIELD_WIDTH);
    write_utils::print_field_u32("PointerToRelocations:", section.pointer_to_relocations, FIELD_WIDTH);
    write_utils::print_field_u32("PointerToLineNumbers:", section.pointer_to_line_numbers, FIELD_WIDTH);
    write_utils::print_field_decimal("NumberOfRelocations:", section.number_of_relocations, FIELD_WIDTH);
    write_utils::print_field_decimal("NumberOfLineNumbers:", section.number_of_line_numbers, FIELD_WIDTH);
    write_utils::print_field_u32("Characteristics:", section.characteristics, FIELD_WIDTH);

    write_utils::print_subsection("  Characteristics:");
    print_section_characteristics(section.characteristics);

    write_utils::print_section_separator();
}

fn section_name_to_string(name: &[u8; 8]) -> String {
    let trimmed = name.iter().take_while(|&&b| b != 0).cloned().collect::<Vec<u8>>();
    String::from_utf8_lossy(&trimmed).into_owned()
}

fn print_section_characteristics(characteristics: u32) {
    if characteristics == 0 {
        write_utils::print_flag("(None)");
        return;
    }

    let flags: &[(u32, &str)] = &[
        (0x00000020, "IMAGE_SCN_CNT_CODE"),
        (0x00000040, "IMAGE_SCN_CNT_INITIALIZED_DATA"),
        (0x00000080, "IMAGE_SCN_CNT_UNINITIALIZED_DATA"),
        (0x00000200, "IMAGE_SCN_LNK_INFO"),
        (0x00000800, "IMAGE_SCN_LNK_REMOVE"),
        (0x00001000, "IMAGE_SCN_LNK_COMDAT"),
        (0x00008000, "IMAGE_SCN_GPREL"),
        (0x01000000, "IMAGE_SCN_LNK_NRELOC_OVFL"),
        (0x02000000, "IMAGE_SCN_MEM_DISCARDABLE"),
        (0x04000000, "IMAGE_SCN_MEM_NOT_CACHED"),
        (0x08000000, "IMAGE_SCN_MEM_NOT_PAGED"),
        (0x10000000, "IMAGE_SCN_MEM_SHARED"),
        (0x20000000, "IMAGE_SCN_MEM_EXECUTE"),
        (0x40000000, "IMAGE_SCN_MEM_READ"),
        (0x80000000, "IMAGE_SCN_MEM_WRITE"),
    ];

    let mut any = false;
    for &(mask, name) in flags {
        if characteristics & mask != 0 {
            write_utils::print_flag(name);
            any = true;
        }
    }

    if !any {
        write_utils::print_flag("(Unknown flags)");
    }
}
