use crate::diagnostics::diagnostics::{Diagnostic, DiagnosticCode, DiagnosticSink, Location};
use crate::diagnostics::diagnosticcodes::DosCode;
use crate::reader::reader::Reader;

pub struct DosReader;

//I think it should be fatal if we cannot read past dos header since that means pe is definitely malformed
impl Reader for DosReader {
    fn read_u8(buf: &[u8], offset: usize, sink: &mut dyn DiagnosticSink, field: &'static str) -> u8 {
        if offset + 1 > buf.len() {
            sink.emit(Diagnostic{
                code: DiagnosticCode::Dos(DosCode::ReadOutOfBounds),
                severity: crate::diagnostics::diagnostics::Severity::Fatal,
                message: "DosHeader read is out of bounds. PE must be malformed".to_string(),
                location: Some(Location::Field(field)),
                evidence: None,
            });
        }
        
        u8::from_le_bytes(buf[offset .. offset + 1].try_into().unwrap())
    }

    fn read_u16(buf: &[u8], offset: usize, sink: &mut dyn DiagnosticSink, field: &'static str) -> u16 {
        if offset + 2 > buf.len() {
            sink.emit(Diagnostic{
                code: DiagnosticCode::Dos(DosCode::ReadOutOfBounds),
                severity: crate::diagnostics::diagnostics::Severity::Fatal,
                message: "DosHeader read is out of bounds. PE must be malformed".to_string(),
                location: Some(Location::Field(field)),
                evidence: None,
            });
        }
        
        u16::from_le_bytes(buf[offset .. offset + 2].try_into().unwrap())
    }

    fn read_u32(buf: &[u8], offset: usize, sink: &mut dyn DiagnosticSink, field: &'static str) -> u32 {
        if offset + 4 > buf.len() {
            sink.emit(Diagnostic{
                code: DiagnosticCode::Dos(DosCode::ReadOutOfBounds),
                severity: crate::diagnostics::diagnostics::Severity::Fatal,
                message: "DosHeader read is out of bounds. PE must be malformed".to_string(),
                location: Some(Location::Field(field)),
                evidence: None,
            });
        }
        
        u32::from_le_bytes(buf[offset .. offset + 4].try_into().unwrap())
    }
    
    fn read_u64(buf: &[u8], offset: usize, sink: &mut dyn DiagnosticSink, field: &'static str) -> u64 {
        if offset + 8 > buf.len() {
            sink.emit(Diagnostic{
                code: DiagnosticCode::Dos(DosCode::ReadOutOfBounds),
                severity: crate::diagnostics::diagnostics::Severity::Fatal,
                message: "DosHeader read is out of bounds. PE must be malformed".to_string(),
                location: Some(Location::Field(field)),
                evidence: None,
            });
        }
        
        u64::from_le_bytes(buf[offset .. offset + 8].try_into().unwrap())
    }
}