#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DosCode {
    InvalidMagic,
    E_lfanewOutOfBounds,
    StubTooSmall,
    ReadOutOfBounds,
}
