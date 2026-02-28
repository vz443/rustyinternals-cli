use crate::{diagnostics::{diagnosticcodes::NtHeaderCode, diagnostics::{Diagnostic, DiagnosticCode, DiagnosticSink, Location}}, reader::reader::Reader};


pub struct NtHeaderReader;

//Also check if this unwrap pattern is fine, since we are bounds checking the splice
//TODO: write this into a macro passing in the Diagnostic
impl Reader for NtHeaderReader {
    
    fn read_u8(buf: &[u8], offset: usize, sink: &mut dyn DiagnosticSink, field: &'static str) -> u8 {
        if offset + 1 > buf.len() {
            sink.emit(Diagnostic{
                code: DiagnosticCode::NtHeader(NtHeaderCode::ReadOutOfBounds),
                severity: crate::diagnostics::diagnostics::Severity::Fatal,
                message: "NtHeader read is out of bounds. PE must be malformed".to_string(),
                location: Some(Location::Field(field)),
                evidence: None,
            });
        }
        
        u8::from_le_bytes(buf[offset .. offset + 1].try_into().unwrap())
    }

    fn read_u16(buf: &[u8], offset: usize, sink: &mut dyn DiagnosticSink, field: &'static str) -> u16 {
        if offset + 2 > buf.len() {
            sink.emit(Diagnostic{
                code: DiagnosticCode::NtHeader(NtHeaderCode::ReadOutOfBounds),
                severity: crate::diagnostics::diagnostics::Severity::Fatal,
                message: "NtHeader read is out of bounds. PE must be malformed".to_string(),
                location: Some(Location::Field(field)),
                evidence: None,
            });
        }
        
        u16::from_le_bytes(buf[offset .. offset + 2].try_into().unwrap())
    }

    fn read_u32(buf: &[u8], offset: usize, sink: &mut dyn DiagnosticSink, field: &'static str) -> u32 {
        if offset + 4 > buf.len() {
            sink.emit(Diagnostic{
                code: DiagnosticCode::NtHeader(NtHeaderCode::ReadOutOfBounds),
                severity: crate::diagnostics::diagnostics::Severity::Fatal,
                message: "NtHeader read is out of bounds. PE must be malformed".to_string(),
                location: Some(Location::Field(field)),
                evidence: None,
            });
        }
        
        u32::from_le_bytes(buf[offset .. offset + 4].try_into().unwrap())
    }
    
    fn read_u64(buf: &[u8], offset: usize, sink: &mut dyn DiagnosticSink, field: &'static str) -> u64 {
        if offset + 8 > buf.len() {
            sink.emit(Diagnostic{
                code: DiagnosticCode::NtHeader(NtHeaderCode::ReadOutOfBounds),
                severity: crate::diagnostics::diagnostics::Severity::Fatal,
                message: "NtHeader read is out of bounds. PE must be malformed".to_string(),
                location: Some(Location::Field(field)),
                evidence: None,
            });
        }
        
        u64::from_le_bytes(buf[offset .. offset + 8].try_into().unwrap())
    }
} 