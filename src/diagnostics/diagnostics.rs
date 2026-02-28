use crate::diagnostics::diagnosticcodes::{DosCode, FileHeaderCode, NtHeaderCode, OptHeaderCode, SectionCode};

pub struct Diagnostic {
    pub code: DiagnosticCode,
    pub severity: Severity,
    pub message: String,
    pub location: Option<Location>,
    pub evidence: Option<Evidence>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiagnosticCode {
    Dos(DosCode),
    Section(SectionCode),
    FileHeader(FileHeaderCode),
    NtHeader(NtHeaderCode),
    OptHeader(OptHeaderCode),
    //Import(imports::ImportCode),
    //Heuristic(heuristics::HeuristicCode),
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

pub trait DiagnosticSink {
    fn emit(&mut self, diag: Diagnostic);
}

pub struct ConsoleDiagnosticSink {
    diagnostics: Vec<Diagnostic>,
}

impl ConsoleDiagnosticSink {
    pub fn new() -> Self {
        Self {
            diagnostics: Vec::new(),
        }
    }

    pub fn has_fatal(&self) -> bool {
        self.diagnostics.iter().any(|d| matches!(d.severity, Severity::Fatal))
    }

    pub fn print_all(&self) {
        for diag in &self.diagnostics {
            self.print_diagnostic(diag);
        }
    }

    fn print_diagnostic(&self, diag: &Diagnostic) {
        let severity_str = match diag.severity {
            Severity::Info => "INFO",
            Severity::Warning => "WARNING",
            Severity::Suspicious => "SUSPICIOUS",
            Severity::Malicious => "MALICIOUS",
            Severity::Fatal => "FATAL",
        };

        print!("[{}] ", severity_str);

        if let Some(ref location) = diag.location {
            match location {
                Location::FileOffset(offset) => print!("@0x{:X} ", offset),
                Location::Rva(rva) => print!("@RVA:0x{:X} ", rva),
                Location::Section { name } => {
                    let section_name = String::from_utf8_lossy(name);
                    print!("@Section:{} ", section_name.trim_end_matches('\0'));
                }
                Location::Field(field) => print!("Field:{} ", field),
            }
        }

        println!("{}", diag.message);

        if let Some(ref evidence) = diag.evidence {
            print!("  Evidence: ");
            match evidence {
                Evidence::U8(v) => println!("0x{:02X}", v),
                Evidence::U16(v) => println!("0x{:04X}", v),
                Evidence::U32(v) => println!("0x{:08X}", v),
                Evidence::U64(v) => println!("0x{:016X}", v),
                Evidence::FileOffset(offset) => println!("FileOffset: 0x{:X}", offset),
                Evidence::Rva(rva) => println!("RVA: 0x{:X}", rva),
                Evidence::Range { start, end } => println!("Range: 0x{:X} - 0x{:X}", start, end),
                Evidence::PairU32 { expected, actual } => {
                    println!("Expected: 0x{:08X}, Actual: 0x{:08X}", expected, actual)
                }
                Evidence::SectionName(name) => {
                    let section_name = String::from_utf8_lossy(name);
                    println!("Section: {}", section_name.trim_end_matches('\0'));
                }
                Evidence::StaticStr(s) => println!("{}", s),
            }
        }
    }
}

impl DiagnosticSink for ConsoleDiagnosticSink {
    fn emit(&mut self, diag: Diagnostic) {
        self.diagnostics.push(diag);
    }
}