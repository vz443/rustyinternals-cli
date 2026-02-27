#[derive(Default)]
struct Section {
    name: [u8; 8],
    virtual_size: u32,
    virtual_address: u32,
    size_of_raw_data: u32,
    pointer_to_raw_data: u32,
    pointer_to_relocations: u32,
    pointer_to_line_numbers: u32,
    number_of_relocations: u16,
    number_of_line_numbers: u16,
    characteristics: u32,
}

#[derive(Default)]
pub struct Sections {
    sections: Vec<Section>,
}

impl Sections {
    pub fn new() -> Self {
        Sections::default()
    }
    
    pub fn parse_all_sections(buf: &[u8], pointer: &mut usize, sectionscount: &u8) -> Sections {
    let mut sections = Sections::default();

    println!("=== Section Headers ===");
    println!("Number of sections: {}\n", sectionscount);

    for _i in 0..*sectionscount {
        let mut section = Section::default();

        section.name = buf[*pointer..*pointer + 8].try_into().unwrap();
        *pointer += 8;

        section.virtual_size =
            u32::from_le_bytes(buf[*pointer..*pointer + 4].try_into().unwrap());
        *pointer += 4;

        section.virtual_address =
            u32::from_le_bytes(buf[*pointer..*pointer + 4].try_into().unwrap());
        *pointer += 4;

        section.size_of_raw_data =
            u32::from_le_bytes(buf[*pointer..*pointer + 4].try_into().unwrap());
        *pointer += 4;

        section.pointer_to_raw_data =
            u32::from_le_bytes(buf[*pointer..*pointer + 4].try_into().unwrap());
        *pointer += 4;

        section.pointer_to_relocations =
            u32::from_le_bytes(buf[*pointer..*pointer + 4].try_into().unwrap());
        *pointer += 4;

        section.pointer_to_line_numbers =
            u32::from_le_bytes(buf[*pointer..*pointer + 4].try_into().unwrap());
        *pointer += 4;

        section.number_of_relocations =
            u16::from_le_bytes(buf[*pointer..*pointer + 2].try_into().unwrap());
        *pointer += 2;

        section.number_of_line_numbers =
            u16::from_le_bytes(buf[*pointer..*pointer + 2].try_into().unwrap());
        *pointer += 2;

        section.characteristics =
            u32::from_le_bytes(buf[*pointer..*pointer + 4].try_into().unwrap());
        *pointer += 4;


        sections.sections.push(section);
    }

    sections
}

}