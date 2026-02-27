use crate::{headers::fileheader::FileHeader, sections::sections::Sections};

#[derive(Default)]
pub struct OptionalHeader {
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
    minor_operationg_system_version: u16,
    major_image_version: u16,
    minor_image_version: u16,
    major_subsystem_version: u16,
    minor_subsystem_version: u16,
    win32_version_value: u32,
    size_of_image: u32,
    size_of_headers: u32,
    check_sum: u32,
    subsystem: u16,
    dll_characteristics: u16,
    size_of_stack_reserve: u32,
    size_of_stack_commit: u32,
    size_of_heap_reserve: u32,
    size_of_heap_commit: u32,
    loader_flags: u32,
    number_of_rva_and_sizes: u32,
    data_directory: ImageDataDirectory,
}

pub struct OptionalHeaderPe32Plus {
    magic: u16,
    major_linker_version: u8,
    minor_linker_version: u8,
    size_of_code: u32,
    size_of_initialized_data: u32,
    size_of_uninitialized_data: u32,
    address_of_entry_point: u32,
    base_of_code: u32,
    base_of_data: u32,
    image_base: u64,
    section_alignment: u32,
    file_alignment: u32,
    major_operating_system_version: u16,
    minor_operationg_system_version: u16,
    major_image_version: u16,
    minor_image_version: u16,
    major_subsystem_version: u16,
    minor_subsystem_version: u16,
    win32_version_value: u32,
    size_of_image: u32,
    size_of_headers: u32,
    check_sum: u32,
    subsystem: u16,
    dll_characteristics: u16,
    size_of_stack_reserve: u64,
    size_of_stack_commit: u64,
    size_of_heap_reserve: u64,
    size_of_heap_commit: u64,
    loader_flags: u32,
    number_of_rva_and_sizes: u32,
    data_directory: ImageDataDirectory,
}

#[derive(Default, Debug, Clone, Copy)]
pub struct ImageDataDirectory {
    pub virtual_address: u32,
    pub size: u32,
}

#[derive(Default, Debug, Clone, Copy)]
pub struct ImageDataDirectoryBase16 {
    pub export_table: ImageDataDirectory,
    pub import_table: ImageDataDirectory,
    pub resource_table: ImageDataDirectory,
    pub exception_table: ImageDataDirectory,
    pub certificate_table: ImageDataDirectory,
    pub base_relocation_table: ImageDataDirectory,
    pub debug: ImageDataDirectory,
    pub architecture: ImageDataDirectory,
    pub global_ptr: ImageDataDirectory,
    pub tls_table: ImageDataDirectory,
    pub load_config_table: ImageDataDirectory,
    pub bound_import: ImageDataDirectory,
    pub iat: ImageDataDirectory,
    pub delay_import_descriptor: ImageDataDirectory,
    pub clr_runtime_header: ImageDataDirectory,
    pub reserved: ImageDataDirectory,
}

#[derive(Default)]
struct UnsanitizedImageDataDirectory {
    data_directories: Vec<ImageDataDirectory>,
}

impl UnsanitizedImageDataDirectory {
    pub fn new() -> Self {
        UnsanitizedImageDataDirectory::default()
    }
}

impl ImageDataDirectory {
    pub fn new() -> Self {
        ImageDataDirectory::default()
    }

    pub fn parse(buf: &[u8], pointer: &mut usize, countofentries: &u32) {
        let mut directory = UnsanitizedImageDataDirectory::new();

        for i in 0..*countofentries {
            let result = u64::from_le_bytes(buf[*pointer .. *pointer + 8].try_into().unwrap());
            
            let lower: u32 = (result & 0xFFFF_FFFF) as u32;
            let upper: u32 = (result >> 32) as u32;
            
            directory.data_directories.push(ImageDataDirectory { virtual_address: lower, size: upper });
            *pointer += 8; 

            println!("Address: {}", i); 
            println!("Size: {:04X}", upper);
            println!("Address: {:04X}", lower);
        }
    }
}

impl OptionalHeader {
    pub fn new() -> Self {
        OptionalHeader::default()    
    }

    pub fn parse(buf: &[u8], fileheader: &FileHeader) {
        let mut optionalheader = OptionalHeader::new();
        let mut pointer: usize = fileheader.end_of_file_header_pointer as usize;

        optionalheader.magic = u16::from_le_bytes(buf[pointer..pointer + 2].try_into().unwrap());
        println!("Magic: {:04X}", optionalheader.magic);
        pointer += 2;

        optionalheader.major_linker_version = u8::from_le_bytes(buf[pointer..pointer + 1].try_into().unwrap());
        println!("Linker Version Major: {:02X}", optionalheader.major_linker_version);
        pointer += 1;

        optionalheader.minor_linker_version = u8::from_le_bytes(buf[pointer..pointer + 1].try_into().unwrap());
        println!("Linker Version Minor: {:02X}", optionalheader.minor_linker_version);
        pointer += 1;

        optionalheader.size_of_code = u32::from_le_bytes(buf[pointer..pointer + 4].try_into().unwrap());
        println!("Size Of Code: {:08X}", optionalheader.size_of_code);
        pointer += 4;

        optionalheader.size_of_initialized_data = u32::from_le_bytes(buf[pointer..pointer + 4].try_into().unwrap());
        println!("Size Of Initialized Data: {:08X}", optionalheader.size_of_initialized_data);
        pointer += 4;

        optionalheader.size_of_uninitialized_data = u32::from_le_bytes(buf[pointer..pointer + 4].try_into().unwrap());
        println!("Size Of Uninitialized Data: {:08X}", optionalheader.size_of_uninitialized_data);
        pointer += 4;

        optionalheader.address_of_entry_point = u32::from_le_bytes(buf[pointer..pointer + 4].try_into().unwrap());
        println!("Address Of Entry Point: {:08X}", optionalheader.address_of_entry_point);
        pointer += 4;

        optionalheader.base_of_code = u32::from_le_bytes(buf[pointer..pointer + 4].try_into().unwrap());
        println!("Base Of Code: {:08X}", optionalheader.base_of_code);
        pointer += 4;

        optionalheader.base_of_data = u32::from_le_bytes(buf[pointer..pointer + 4].try_into().unwrap());
        println!("Base Of Data: {:08X}", optionalheader.base_of_data);
        pointer += 4;

        optionalheader.image_base = u32::from_le_bytes(buf[pointer..pointer + 4].try_into().unwrap());
        println!("Image Base: {:08X}", optionalheader.image_base);
        pointer += 4;

        optionalheader.section_alignment = u32::from_le_bytes(buf[pointer..pointer + 4].try_into().unwrap());
        println!("Section Alignment: {:08X}", optionalheader.section_alignment);
        pointer += 4;

        optionalheader.file_alignment = u32::from_le_bytes(buf[pointer..pointer + 4].try_into().unwrap());
        println!("File Alignment: {:08X}", optionalheader.file_alignment);
        pointer += 4;

        optionalheader.major_operating_system_version =
            u16::from_le_bytes(buf[pointer..pointer + 2].try_into().unwrap());
        println!("Major Operating System Version: {:04X}", optionalheader.major_operating_system_version);
        pointer += 2;

        optionalheader.minor_operationg_system_version =
            u16::from_le_bytes(buf[pointer..pointer + 2].try_into().unwrap());
        println!("Minor Operating System Version: {:04X}", optionalheader.minor_operationg_system_version);
        pointer += 2;

        optionalheader.major_image_version =
            u16::from_le_bytes(buf[pointer..pointer + 2].try_into().unwrap());
        println!("Major Image Version: {:04X}", optionalheader.major_image_version);
        pointer += 2;

        optionalheader.minor_image_version =
            u16::from_le_bytes(buf[pointer..pointer + 2].try_into().unwrap());
        println!("Minor Image Version: {:04X}", optionalheader.minor_image_version);
        pointer += 2;

        optionalheader.major_subsystem_version =
            u16::from_le_bytes(buf[pointer..pointer + 2].try_into().unwrap());
        println!("Major Subsystem Version: {:04X}", optionalheader.major_subsystem_version);
        pointer += 2;

        optionalheader.minor_subsystem_version =
            u16::from_le_bytes(buf[pointer..pointer + 2].try_into().unwrap());
        println!("Minor Subsystem Version: {:04X}", optionalheader.minor_subsystem_version);
        pointer += 2;

        optionalheader.win32_version_value =
            u32::from_le_bytes(buf[pointer..pointer + 4].try_into().unwrap());
        println!("Win32 Version Value: {:08X}", optionalheader.win32_version_value);
        pointer += 4;

        optionalheader.size_of_image =
            u32::from_le_bytes(buf[pointer..pointer + 4].try_into().unwrap());
        println!("Size Of Image: {:08X}", optionalheader.size_of_image);
        pointer += 4;

        optionalheader.size_of_headers =
            u32::from_le_bytes(buf[pointer..pointer + 4].try_into().unwrap());
        println!("Size Of Headers: {:08X}", optionalheader.size_of_headers);
        pointer += 4;

        optionalheader.check_sum =
            u32::from_le_bytes(buf[pointer..pointer + 4].try_into().unwrap());
        println!("Checksum: {:08X}", optionalheader.check_sum);
        pointer += 4;

        optionalheader.subsystem =
            u16::from_le_bytes(buf[pointer..pointer + 2].try_into().unwrap());
        println!("Subsystem: {:04X}", optionalheader.subsystem);
        pointer += 2;

        optionalheader.dll_characteristics =
            u16::from_le_bytes(buf[pointer..pointer + 2].try_into().unwrap());
        println!("DLL Characteristics: {:04X}", optionalheader.dll_characteristics);
        pointer += 2;

        optionalheader.size_of_stack_reserve =
            u32::from_le_bytes(buf[pointer..pointer + 4].try_into().unwrap());
        println!("Size Of Stack Reserve: {:08X}", optionalheader.size_of_stack_reserve);
        pointer += 4;

        optionalheader.size_of_stack_commit =
            u32::from_le_bytes(buf[pointer..pointer + 4].try_into().unwrap());
        println!("Size Of Stack Commit: {:08X}", optionalheader.size_of_stack_commit);
        pointer += 4;

        optionalheader.size_of_heap_reserve =
            u32::from_le_bytes(buf[pointer..pointer + 4].try_into().unwrap());
        println!("Size Of Heap Reserve: {:08X}", optionalheader.size_of_heap_reserve);
        pointer += 4;

        optionalheader.size_of_heap_commit =
            u32::from_le_bytes(buf[pointer..pointer + 4].try_into().unwrap());
        println!("Size Of Heap Commit: {:08X}", optionalheader.size_of_heap_commit);
        pointer += 4;

        optionalheader.loader_flags =
            u32::from_le_bytes(buf[pointer..pointer + 4].try_into().unwrap());
        println!("Loader Flags: {:08X}", optionalheader.loader_flags);
        pointer += 4;

        optionalheader.number_of_rva_and_sizes =
            u32::from_le_bytes(buf[pointer..pointer + 4].try_into().unwrap());
        println!("Number Of RVA And Sizes: {:08X}", optionalheader.number_of_rva_and_sizes);
        pointer += 4;
        
        ImageDataDirectory::parse(buf, &mut pointer, &optionalheader.number_of_rva_and_sizes);
        Sections::parse_all_sections(buf, &mut pointer, &9);

        //optionalheader.DataDirectory = DataDirectory::parse(buf, &mut pointer);
    }


}