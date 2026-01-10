use crate::diagnostics::diagnostics::DiagnosticSink;

pub trait Reader {
    fn read_u8(buf: &[u8], offset: usize,sink: &mut dyn DiagnosticSink, field: &'static str) -> u8;
    fn read_u16(buf: &[u8], offset: usize, sink: &mut dyn DiagnosticSink, field: &'static str) -> u16; 
    fn read_u32(buf: &[u8], offset: usize, sink: &mut dyn DiagnosticSink, field: &'static str) -> u32; 
    fn read_u64(buf: &[u8], offset: usize, sink: &mut dyn DiagnosticSink, field: &'static str) -> u64; 
}