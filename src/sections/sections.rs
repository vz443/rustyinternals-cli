#[derive(Default)]
struct Section {
    Name: [u8; 8],
    VirtualSize: u32,
    VirtualAddress: u32,
    SizeOfRawData: u32,
    PointerToRawData: u32,
    PointerToRelocations: u32,
    PointerToLineNumbers: u32,
    NumberOfRelocations: u16,
    NumberOfLineNumbers: u16,
    Characteristics: u32,
}

#[derive(Default)]
pub struct Sections {
    Sections: Vec<Section>,    
}

impl Sections {
    pub fn new() -> Self {
        Sections::default()
    }
    
    pub fn parse_all_sections(buf: &[u8], pointer: &mut usize, sectionscount: &u8) -> Sections {
    let mut sections = Sections::default();

    println!("=== Section Headers ===");
    println!("Number of sections: {}\n", sectionscount);

    for i in 0..*sectionscount {
        let mut section = Section::default();

        section.Name = buf[*pointer..*pointer + 8].try_into().unwrap();
        *pointer += 8;

        section.VirtualSize =
            u32::from_le_bytes(buf[*pointer..*pointer + 4].try_into().unwrap());
        *pointer += 4;

        section.VirtualAddress =
            u32::from_le_bytes(buf[*pointer..*pointer + 4].try_into().unwrap());
        *pointer += 4;

        section.SizeOfRawData =
            u32::from_le_bytes(buf[*pointer..*pointer + 4].try_into().unwrap());
        *pointer += 4;

        section.PointerToRawData =
            u32::from_le_bytes(buf[*pointer..*pointer + 4].try_into().unwrap());
        *pointer += 4;

        section.PointerToRelocations =
            u32::from_le_bytes(buf[*pointer..*pointer + 4].try_into().unwrap());
        *pointer += 4;

        section.PointerToLineNumbers =
            u32::from_le_bytes(buf[*pointer..*pointer + 4].try_into().unwrap());
        *pointer += 4;

        section.NumberOfRelocations =
            u16::from_le_bytes(buf[*pointer..*pointer + 2].try_into().unwrap());
        *pointer += 2;

        section.NumberOfLineNumbers =
            u16::from_le_bytes(buf[*pointer..*pointer + 2].try_into().unwrap());
        *pointer += 2;

        section.Characteristics =
            u32::from_le_bytes(buf[*pointer..*pointer + 4].try_into().unwrap());
        *pointer += 4;


        sections.Sections.push(section);
    }

    sections
}

}