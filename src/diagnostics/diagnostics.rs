pub struct Diagnostic {
    pub code: DiagnosticCode,
    pub severity: Severity,
    pub message: String,
    pub location: Option<Location>,
    pub evidence: Option<Evidence>,
}

pub enum Severity {
    Info,
    Warning,
    Suspicious,
    Malicious,
    Fatal,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Evidence {
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),

    FileOffset(u64),
    Rva(u32),

    Range {
        start: u64,
        end: u64,
    },

    PairU32 {
        expected: u32,
        actual: u32,
    },

    SectionName([u8; 8]),

    StaticStr(&'static str),
}

pub enum Location {
    FileOffset(u64),
    Rva(u32),
    Section { name: [u8; 8] },
    Field(&'static str),
}

pub enum DiagnosticCode {
    InvalidDosMagic,
    E_lfanewOutOfBounds,
    SectionOverlap,
    SectionSizeMismatch,
    ExecutableWritableSection,
    ImportTableOutsideImage,
    SuspiciousEntryPoint,
}

pub trait DiagnosticSink {
    fn emit(&mut self, diag: Diagnostic);
}