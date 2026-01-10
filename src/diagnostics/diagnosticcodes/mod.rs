pub mod dos;
pub mod nt;
pub mod sections;
pub mod imports;
pub mod heuristics;

use dos::DosCode;
use sections::SectionCode;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiagnosticCode {
    Dos(DosCode),
    Section(SectionCode),
    //Nt(nt::NtCode),
    //Import(imports::ImportCode),
    //Heuristic(heuristics::HeuristicCode),
}
