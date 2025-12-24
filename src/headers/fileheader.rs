use crate::headers::dosheader::ParseError; // move somewhere else 


#[derive(Default)]
struct FileHeader {
    Machine: u32,
    NumberOfSections: u32,
    TimeDateStamp: u16,
    PointerToSymbolTable: u16,
    NumberOfSymbols: u16,
    SizeOfOptionalHeader: u32,
    Characteristics: u32 
}

impl FileHeader {
    pub fn new() -> Self {
        Self::default()
    }
     
    pub fn parse() -> Result<Self, ParseError> {
         let mut fileheader = FileHeader::new();
    }
}