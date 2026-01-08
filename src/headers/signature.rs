use crate::errors::errors::ParseError;

#[derive(Default)]
pub struct Signature {
    pub signature: u32,
}

impl Signature {
    pub fn new() -> Self {
       Signature::default()
    }
    
    pub fn parse(buf: &[u8]) -> Result<Self, ParseError> {
        
        let e_lfanew = &buf[0x3c..0x40];
        let ntpointer = u32::from_le_bytes(e_lfanew.try_into()?);

        let res = u32::from_le_bytes(buf[ntpointer as usize..ntpointer as usize + 4].try_into()?);

        return Ok(Self {
            signature: res,
        })
    }
}