use bitflags::bitflags;

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    pub struct Characteristics: u16 {
        const IMAGE_FILE_RELOCS_STRIPPED         = 0x0001;
        const IMAGE_FILE_EXECUTABLE_IMAGE        = 0x0002;
        const IMAGE_FILE_LINE_NUMS_STRIPPED      = 0x0004;
        const IMAGE_FILE_LOCAL_SYMS_STRIPPED     = 0x0008;
        const IMAGE_FILE_AGGRESSIVE_WS_TRIM      = 0x0010;
        const IMAGE_FILE_LARGE_ADDRESS_AWARE     = 0x0020;
        const IMAGE_FILE_RESERVED_0040           = 0x0040;
        const IMAGE_FILE_BYTES_REVERSED_LO       = 0x0080;
        const IMAGE_FILE_32BIT_MACHINE           = 0x0100;
        const IMAGE_FILE_DEBUG_STRIPPED          = 0x0200;
        const IMAGE_FILE_REMOVABLE_RUN_FROM_SWAP = 0x0400;
        const IMAGE_FILE_NET_RUN_FROM_SWAP       = 0x0800;
        const IMAGE_FILE_SYSTEM                  = 0x1000;
        const IMAGE_FILE_DLL                     = 0x2000;
        const IMAGE_FILE_UP_SYSTEM_ONLY          = 0x4000;
        const IMAGE_FILE_BYTES_REVERSED_HI       = 0x8000;
    }
}

#[derive(Default)]
pub struct FileHeader {
    Machine: u16,
    NumberOfSections: u16,
    TimeDateStamp: u32,
    PointerToSymbolTable: u32,
    NumberOfSymbols: u32,
    SizeOfOptionalHeader: u16,
    Characteristics: Characteristics,
    pub EndOfFileHeaderPointer: u32,
}




impl FileHeader {
    pub fn new() -> Self {
        Self::default()
    }
     
    pub fn parse(buf: &[u8]) -> Self { 

        let e_lfanew = u32::from_le_bytes(buf[0x3C..0x40].try_into().unwrap());
        let mut fileheader = FileHeader::new();
        
        
        println!("lfanew: {}", e_lfanew);
        let mut pointer: u32 = e_lfanew;
        pointer += 4;
        
        let machine = u16::from_le_bytes(buf[pointer as usize..pointer as usize + 2].try_into().unwrap());
        println!("Machine; {:04X}", machine);
        fileheader.Machine = machine;
        
        pointer += 2;
        let numofsections = u16::from_le_bytes(buf[pointer as usize..pointer as usize + 2].try_into().unwrap()); 
        println!("NumOfSections; {}", numofsections);
        fileheader.NumberOfSections = numofsections;
        
        pointer += 2;
        let timestamp = u32::from_le_bytes(buf[pointer as usize..pointer as usize + 4].try_into().unwrap());
        println!("{:08X}", timestamp); // check this with something that has a timestamp
        fileheader.TimeDateStamp = timestamp;

        pointer += 4;
        let ptrtosymboltable = u32::from_le_bytes(buf[pointer as usize..pointer as usize + 4].try_into().unwrap());
        println!("Pointer to symbol table: {}", ptrtosymboltable);
        fileheader.PointerToSymbolTable = ptrtosymboltable;

        pointer += 4;
        let numofsymbols = u32::from_le_bytes(buf[pointer as usize..pointer as usize + 4].try_into().unwrap());
        println!("Number of symbols: {}", numofsymbols);
        fileheader.NumberOfSymbols = numofsymbols;

        pointer += 4;
        let sizeofoptionalheader = u16::from_le_bytes(buf[pointer as usize..pointer as usize + 2].try_into().unwrap());
        println!("Size of optional header: {}", sizeofoptionalheader);
        fileheader.SizeOfOptionalHeader = sizeofoptionalheader;

        pointer += 2;
        let characteristics = Characteristics::from_bits_truncate(u16::from_le_bytes(buf[pointer as usize..pointer as usize + 2].try_into().unwrap()));

        print_characteristics(characteristics);

        fileheader.EndOfFileHeaderPointer = pointer + 2;

        fileheader
    }
    
}

fn print_characteristics(characteristics: Characteristics) {
    println!("Characteristics");

    for flag in characteristics.iter() {
        match flag {
            Characteristics::IMAGE_FILE_RELOCS_STRIPPED => {
                println!("  IMAGE_FILE_RELOCS_STRIPPED");
            }
            Characteristics::IMAGE_FILE_EXECUTABLE_IMAGE => {
                println!("  IMAGE_FILE_EXECUTABLE_IMAGE");
            }
            Characteristics::IMAGE_FILE_LINE_NUMS_STRIPPED => {
                println!("  IMAGE_FILE_LINE_NUMS_STRIPPED");
            }
            Characteristics::IMAGE_FILE_LOCAL_SYMS_STRIPPED => {
                println!("  IMAGE_FILE_LOCAL_SYMS_STRIPPED");
            }
            Characteristics::IMAGE_FILE_AGGRESSIVE_WS_TRIM => {
                println!("  IMAGE_FILE_AGGRESSIVE_WS_TRIM");
            }
            Characteristics::IMAGE_FILE_LARGE_ADDRESS_AWARE => {
                println!("  IMAGE_FILE_LARGE_ADDRESS_AWARE");
            }
            Characteristics::IMAGE_FILE_RESERVED_0040 => {
                println!("  IMAGE_FILE_RESERVED_0040");
            }
            Characteristics::IMAGE_FILE_BYTES_REVERSED_LO => {
                println!("  IMAGE_FILE_BYTES_REVERSED_LO");
            }
            Characteristics::IMAGE_FILE_32BIT_MACHINE => {
                println!("  IMAGE_FILE_32BIT_MACHINE");
            }
            Characteristics::IMAGE_FILE_DEBUG_STRIPPED => {
                println!("  IMAGE_FILE_DEBUG_STRIPPED");
            }
            Characteristics::IMAGE_FILE_REMOVABLE_RUN_FROM_SWAP => {
                println!("  IMAGE_FILE_REMOVABLE_RUN_FROM_SWAP");
            }
            Characteristics::IMAGE_FILE_NET_RUN_FROM_SWAP => {
                println!("  IMAGE_FILE_NET_RUN_FROM_SWAP");
            }
            Characteristics::IMAGE_FILE_SYSTEM => {
                println!("  IMAGE_FILE_SYSTEM");
            }
            Characteristics::IMAGE_FILE_DLL => {
                println!("  IMAGE_FILE_DLL");
            }
            Characteristics::IMAGE_FILE_UP_SYSTEM_ONLY => {
                println!("  IMAGE_FILE_UP_SYSTEM_ONLY");
            }
            Characteristics::IMAGE_FILE_BYTES_REVERSED_HI => {
                println!("  IMAGE_FILE_BYTES_REVERSED_HI");
            }
            _ => {
                println!("  UNKNOWN_FLAG");
            }
        }
    }
}