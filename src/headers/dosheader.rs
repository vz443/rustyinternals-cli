#[derive(Default)]
pub struct DosHeader {
    pub e_magic: u16,
    pub e_cblp: u16,
    pub e_cp: u16,
    pub e_crlc: u16,
    pub e_cparhdr: u16,
    pub e_minalloc: u16,
    pub e_maxalloc: u16,
    pub e_ss: u16,
    pub e_sp: u16,
    pub e_csum: u16,
    pub e_ip: u16,
    pub e_cs: u16,
    pub e_lfarlc: u16,
    pub e_ovno: u16,
    pub e_res: [u16; 4],
    pub e_oemid: u16,
    pub e_oeminfo: u16,
    pub e_res2: [u16; 10],
    pub e_lfanew: u32,
}
#[derive(Debug)]
pub enum ParseError {
    UnexpectedEOF,
    InvalidMagic,
    InvalidOffset,
    InvalidField,
    SliceError,
}

use std::array::TryFromSliceError;

impl From<TryFromSliceError> for ParseError {
    fn from(_: TryFromSliceError) -> Self {
        ParseError::SliceError
    }
}
impl DosHeader {
    pub fn new() -> Self {
        Self::default()
    }
    
pub fn parse(buf: &[u8]) -> Result<Self, ParseError> {
    Ok(Self {
        e_magic:   u16::from_le_bytes(buf[0x00..0x02].try_into()?),
        e_cblp:    u16::from_le_bytes(buf[0x02..0x04].try_into()?),
        e_cp:      u16::from_le_bytes(buf[0x04..0x06].try_into()?),
        e_crlc:    u16::from_le_bytes(buf[0x06..0x08].try_into()?),
        e_cparhdr: u16::from_le_bytes(buf[0x08..0x0A].try_into()?),
        e_minalloc: u16::from_le_bytes(buf[0x0A..0x0C].try_into()?),
        e_maxalloc: u16::from_le_bytes(buf[0x0C..0x0E].try_into()?),
        e_ss:      u16::from_le_bytes(buf[0x0E..0x10].try_into()?),
        e_sp:      u16::from_le_bytes(buf[0x10..0x12].try_into()?),
        e_csum:    u16::from_le_bytes(buf[0x12..0x14].try_into()?),
        e_ip:      u16::from_le_bytes(buf[0x14..0x16].try_into()?),
        e_cs:      u16::from_le_bytes(buf[0x16..0x18].try_into()?),
        e_lfarlc:  u16::from_le_bytes(buf[0x18..0x1A].try_into()?),
        e_ovno:    u16::from_le_bytes(buf[0x1A..0x1C].try_into()?),

        e_res: [
            u16::from_le_bytes(buf[0x1C..0x1E].try_into()?),
            u16::from_le_bytes(buf[0x1E..0x20].try_into()?),
            u16::from_le_bytes(buf[0x20..0x22].try_into()?),
            u16::from_le_bytes(buf[0x22..0x24].try_into()?),
        ],

        e_oemid:   u16::from_le_bytes(buf[0x24..0x26].try_into()?),
        e_oeminfo: u16::from_le_bytes(buf[0x26..0x28].try_into()?),

        e_res2: [
            u16::from_le_bytes(buf[0x28..0x2A].try_into()?),
            u16::from_le_bytes(buf[0x2A..0x2C].try_into()?),
            u16::from_le_bytes(buf[0x2C..0x2E].try_into()?),
            u16::from_le_bytes(buf[0x2E..0x30].try_into()?),
            u16::from_le_bytes(buf[0x30..0x32].try_into()?),
            u16::from_le_bytes(buf[0x32..0x34].try_into()?),
            u16::from_le_bytes(buf[0x34..0x36].try_into()?),
            u16::from_le_bytes(buf[0x36..0x38].try_into()?),
            u16::from_le_bytes(buf[0x38..0x3A].try_into()?),
            u16::from_le_bytes(buf[0x3A..0x3C].try_into()?),
        ],

        e_lfanew:  u32::from_le_bytes(buf[0x3C..0x40].try_into()?),
    })
}
}