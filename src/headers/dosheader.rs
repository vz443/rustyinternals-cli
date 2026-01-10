use crate::diagnostics::diagnostics::DiagnosticSink;
use crate::reader::dosreader::DosReader;
use crate::reader::reader::Reader;

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

impl DosHeader {
    pub fn new() -> Self {
        Self::default()
    }
    
    pub fn parse(buf: &[u8], sink: &mut dyn DiagnosticSink) -> Self {
        Self {
            e_magic:    DosReader::read_u16(buf, 0x00, sink, "e_magic"),
            e_cblp:     DosReader::read_u16(buf, 0x02, sink, "e_cblp"),
            e_cp:       DosReader::read_u16(buf, 0x04, sink, "e_cp"),
            e_crlc:     DosReader::read_u16(buf, 0x06, sink, "e_crlc"),
            e_cparhdr:  DosReader::read_u16(buf, 0x08, sink, "e_cparhdr"),
            e_minalloc: DosReader::read_u16(buf, 0x0A, sink, "e_minalloc"),
            e_maxalloc: DosReader::read_u16(buf, 0x0C, sink, "e_maxalloc"),
            e_ss:       DosReader::read_u16(buf, 0x0E, sink, "e_ss"),
            e_sp:       DosReader::read_u16(buf, 0x10, sink, "e_sp"),
            e_csum:     DosReader::read_u16(buf, 0x12, sink, "e_csum"),
            e_ip:       DosReader::read_u16(buf, 0x14, sink, "e_ip"),
            e_cs:       DosReader::read_u16(buf, 0x16, sink, "e_cs"),
            e_lfarlc:   DosReader::read_u16(buf, 0x18, sink, "e_lfarlc"),
            e_ovno:     DosReader::read_u16(buf, 0x1A, sink, "e_ovno"),
            
            e_res: [
                DosReader::read_u16(buf, 0x1C, sink, "e_res[0]"),
                DosReader::read_u16(buf, 0x1E, sink, "e_res[1]"),
                DosReader::read_u16(buf, 0x20, sink, "e_res[2]"),
                DosReader::read_u16(buf, 0x22, sink, "e_res[3]"),
            ],
            
            e_oemid:    DosReader::read_u16(buf, 0x24, sink, "e_oemid"),
            e_oeminfo:  DosReader::read_u16(buf, 0x26, sink, "e_oeminfo"),
            
            e_res2: [
                DosReader::read_u16(buf, 0x28, sink, "e_res2[0]"),
                DosReader::read_u16(buf, 0x2A, sink, "e_res2[1]"),
                DosReader::read_u16(buf, 0x2C, sink, "e_res2[2]"),
                DosReader::read_u16(buf, 0x2E, sink, "e_res2[3]"),
                DosReader::read_u16(buf, 0x30, sink, "e_res2[4]"),
                DosReader::read_u16(buf, 0x32, sink, "e_res2[5]"),
                DosReader::read_u16(buf, 0x34, sink, "e_res2[6]"),
                DosReader::read_u16(buf, 0x36, sink, "e_res2[7]"),
                DosReader::read_u16(buf, 0x38, sink, "e_res2[8]"),
                DosReader::read_u16(buf, 0x3A, sink, "e_res2[9]"),
            ],
            
            e_lfanew:   DosReader::read_u32(buf, 0x3C, sink, "e_lfanew"),
        }
    }
}