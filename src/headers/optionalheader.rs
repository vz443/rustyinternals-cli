use crate::{headers::fileheader::FileHeader, sections::sections::Sections};

#[derive(Default)]
pub struct OptionalHeader {
    Magic: u16,
    MajorLinkerVersion: u8,
    MinorLinkerVersion: u8,
    SizeOfCode: u32,
    SizeOfInitializedData: u32,
    SizeOfUninitializedData: u32,    
    AddressOfEntryPoint: u32,
    BaseOfCode: u32,
    BaseOfData: u32,
    ImageBase: u32,
    SectionAlignment: u32,
    FileAlignment: u32,
    MajorOperatingSystemVersion: u16,
    MinorOperationgSystemVersion: u16,
    MajorImageVersion: u16,
    MinorImageVersion: u16,
    MajorSubsystemVersion: u16,
    MinorSubsystemVersion: u16,
    Win32VersionValue: u32,
    SizeOfImage: u32,
    SizeOfHeaders: u32,
    CheckSum: u32,
    Subsystem: u16,
    DLLCharacteristics: u16,
    SizeOfStackReserve: u32,
    SizeOfStackCommit: u32,
    SizeOfHeapReserve: u32,
    SizeOfHeapCommit: u32,
    LoaderFlags: u32,
    NumberOfRvaAndSizes: u32,
    DataDirectory: ImageDataDirectory,
}

pub struct OptionalHeader_PE32Plus {
    Magic: u16,
    MajorLinkerVersion: u8,
    MinorLinkerVersion: u8,
    SizeOfCode: u32,
    SizeOfInitializedData: u32,
    SizeOfUninitializedData: u32,    
    AddressOfEntryPoint: u32,
    BaseOfCode: u32,
    BaseOfData: u32,
    ImageBase: u64,
    SectionAlignment: u32,
    FileAlignment: u32,
    MajorOperatingSystemVersion: u16,
    MinorOperationgSystemVersion: u16,
    MajorImageVersion: u16,
    MinorImageVersion: u16,
    MajorSubsystemVersion: u16,
    MinorSubsystemVersion: u16,
    Win32VersionValue: u32,
    SizeOfImage: u32,
    SizeOfHeaders: u32,
    CheckSum: u32,
    Subsystem: u16,
    DLLCharacteristics: u16,
    SizeOfStackReserve: u64,
    SizeOfStackCommit: u64,
    SizeOfHeapReserve: u64,
    SizeOfHeapCommit: u64,
    LoaderFlags: u32,
    NumberOfRvaAndSizes: u32,
    DataDirectory: ImageDataDirectory,
}

#[derive(Default, Debug, Clone, Copy)]
pub struct ImageDataDirectory {
    pub VirtualAddress: u32,
    pub Size: u32,
}

#[derive(Default, Debug, Clone, Copy)]
pub struct ImageDataDirectoryBase16 {
    pub ExportTable: ImageDataDirectory,
    pub ImportTable: ImageDataDirectory,
    pub ResourceTable: ImageDataDirectory,
    pub ExceptionTable: ImageDataDirectory,
    pub CertificateTable: ImageDataDirectory,
    pub BaseRelocationTable: ImageDataDirectory,
    pub Debug: ImageDataDirectory,
    pub Architecture: ImageDataDirectory,
    pub GlobalPtr: ImageDataDirectory,
    pub TLSTable: ImageDataDirectory,
    pub LoadConfigTable: ImageDataDirectory,
    pub BoundImport: ImageDataDirectory,
    pub IAT: ImageDataDirectory,
    pub DelayImportDescriptor: ImageDataDirectory,
    pub CLRRuntimeHeader: ImageDataDirectory,
    pub Reserved: ImageDataDirectory,
}

#[derive(Default)]
struct UnsanitizedImageDataDirectory {
    DataDirectories: Vec<ImageDataDirectory>,
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
            
            directory.DataDirectories.push(ImageDataDirectory { VirtualAddress: lower, Size: upper });
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
        let mut pointer: usize = fileheader.EndOfFileHeaderPointer as usize;

        optionalheader.Magic = u16::from_le_bytes(buf[pointer..pointer + 2].try_into().unwrap());
        println!("Magic: {:04X}", optionalheader.Magic);
        pointer += 2;

        optionalheader.MajorLinkerVersion = u8::from_le_bytes(buf[pointer..pointer + 1].try_into().unwrap());
        println!("Linker Version Major: {:02X}", optionalheader.MajorLinkerVersion);
        pointer += 1;

        optionalheader.MinorLinkerVersion = u8::from_le_bytes(buf[pointer..pointer + 1].try_into().unwrap());
        println!("Linker Version Minor: {:02X}", optionalheader.MinorLinkerVersion);
        pointer += 1;

        optionalheader.SizeOfCode = u32::from_le_bytes(buf[pointer..pointer + 4].try_into().unwrap());
        println!("Size Of Code: {:08X}", optionalheader.SizeOfCode);
        pointer += 4;

        optionalheader.SizeOfInitializedData = u32::from_le_bytes(buf[pointer..pointer + 4].try_into().unwrap());
        println!("Size Of Initialized Data: {:08X}", optionalheader.SizeOfInitializedData);
        pointer += 4;

        optionalheader.SizeOfUninitializedData = u32::from_le_bytes(buf[pointer..pointer + 4].try_into().unwrap());
        println!("Size Of Uninitialized Data: {:08X}", optionalheader.SizeOfUninitializedData);
        pointer += 4;

        optionalheader.AddressOfEntryPoint = u32::from_le_bytes(buf[pointer..pointer + 4].try_into().unwrap());
        println!("Address Of Entry Point: {:08X}", optionalheader.AddressOfEntryPoint);
        pointer += 4;

        optionalheader.BaseOfCode = u32::from_le_bytes(buf[pointer..pointer + 4].try_into().unwrap());
        println!("Base Of Code: {:08X}", optionalheader.BaseOfCode);
        pointer += 4;

        optionalheader.BaseOfData = u32::from_le_bytes(buf[pointer..pointer + 4].try_into().unwrap());
        println!("Base Of Data: {:08X}", optionalheader.BaseOfData);
        pointer += 4;

        optionalheader.ImageBase = u32::from_le_bytes(buf[pointer..pointer + 4].try_into().unwrap());
        println!("Image Base: {:08X}", optionalheader.ImageBase);
        pointer += 4;

        optionalheader.SectionAlignment = u32::from_le_bytes(buf[pointer..pointer + 4].try_into().unwrap());
        println!("Section Alignment: {:08X}", optionalheader.SectionAlignment);
        pointer += 4;

        optionalheader.FileAlignment = u32::from_le_bytes(buf[pointer..pointer + 4].try_into().unwrap());
        println!("File Alignment: {:08X}", optionalheader.FileAlignment);
        pointer += 4;

        optionalheader.MajorOperatingSystemVersion =
            u16::from_le_bytes(buf[pointer..pointer + 2].try_into().unwrap());
        println!("Major Operating System Version: {:04X}", optionalheader.MajorOperatingSystemVersion);
        pointer += 2;

        optionalheader.MinorOperationgSystemVersion =
            u16::from_le_bytes(buf[pointer..pointer + 2].try_into().unwrap());
        println!("Minor Operating System Version: {:04X}", optionalheader.MinorOperationgSystemVersion);
        pointer += 2;

        optionalheader.MajorImageVersion =
            u16::from_le_bytes(buf[pointer..pointer + 2].try_into().unwrap());
        println!("Major Image Version: {:04X}", optionalheader.MajorImageVersion);
        pointer += 2;

        optionalheader.MinorImageVersion =
            u16::from_le_bytes(buf[pointer..pointer + 2].try_into().unwrap());
        println!("Minor Image Version: {:04X}", optionalheader.MinorImageVersion);
        pointer += 2;

        optionalheader.MajorSubsystemVersion =
            u16::from_le_bytes(buf[pointer..pointer + 2].try_into().unwrap());
        println!("Major Subsystem Version: {:04X}", optionalheader.MajorSubsystemVersion);
        pointer += 2;

        optionalheader.MinorSubsystemVersion =
            u16::from_le_bytes(buf[pointer..pointer + 2].try_into().unwrap());
        println!("Minor Subsystem Version: {:04X}", optionalheader.MinorSubsystemVersion);
        pointer += 2;

        optionalheader.Win32VersionValue =
            u32::from_le_bytes(buf[pointer..pointer + 4].try_into().unwrap());
        println!("Win32 Version Value: {:08X}", optionalheader.Win32VersionValue);
        pointer += 4;

        optionalheader.SizeOfImage =
            u32::from_le_bytes(buf[pointer..pointer + 4].try_into().unwrap());
        println!("Size Of Image: {:08X}", optionalheader.SizeOfImage);
        pointer += 4;

        optionalheader.SizeOfHeaders =
            u32::from_le_bytes(buf[pointer..pointer + 4].try_into().unwrap());
        println!("Size Of Headers: {:08X}", optionalheader.SizeOfHeaders);
        pointer += 4;

        optionalheader.CheckSum =
            u32::from_le_bytes(buf[pointer..pointer + 4].try_into().unwrap());
        println!("Checksum: {:08X}", optionalheader.CheckSum);
        pointer += 4;

        optionalheader.Subsystem =
            u16::from_le_bytes(buf[pointer..pointer + 2].try_into().unwrap());
        println!("Subsystem: {:04X}", optionalheader.Subsystem);
        pointer += 2;

        optionalheader.DLLCharacteristics =
            u16::from_le_bytes(buf[pointer..pointer + 2].try_into().unwrap());
        println!("DLL Characteristics: {:04X}", optionalheader.DLLCharacteristics);
        pointer += 2;

        optionalheader.SizeOfStackReserve =
            u32::from_le_bytes(buf[pointer..pointer + 4].try_into().unwrap());
        println!("Size Of Stack Reserve: {:08X}", optionalheader.SizeOfStackReserve);
        pointer += 4;

        optionalheader.SizeOfStackCommit =
            u32::from_le_bytes(buf[pointer..pointer + 4].try_into().unwrap());
        println!("Size Of Stack Commit: {:08X}", optionalheader.SizeOfStackCommit);
        pointer += 4;

        optionalheader.SizeOfHeapReserve =
            u32::from_le_bytes(buf[pointer..pointer + 4].try_into().unwrap());
        println!("Size Of Heap Reserve: {:08X}", optionalheader.SizeOfHeapReserve);
        pointer += 4;

        optionalheader.SizeOfHeapCommit =
            u32::from_le_bytes(buf[pointer..pointer + 4].try_into().unwrap());
        println!("Size Of Heap Commit: {:08X}", optionalheader.SizeOfHeapCommit);
        pointer += 4;

        optionalheader.LoaderFlags =
            u32::from_le_bytes(buf[pointer..pointer + 4].try_into().unwrap());
        println!("Loader Flags: {:08X}", optionalheader.LoaderFlags);
        pointer += 4;

        optionalheader.NumberOfRvaAndSizes =
            u32::from_le_bytes(buf[pointer..pointer + 4].try_into().unwrap());
        println!("Number Of RVA And Sizes: {:08X}", optionalheader.NumberOfRvaAndSizes);
        pointer += 4;
        
        ImageDataDirectory::parse(buf, &mut pointer, &optionalheader.NumberOfRvaAndSizes);
        Sections::parse_all_sections(buf, &mut pointer, &9);

        //optionalheader.DataDirectory = DataDirectory::parse(buf, &mut pointer);
    }


}