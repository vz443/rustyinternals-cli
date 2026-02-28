#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DosCode {
    InvalidMagic,
    ELfanewOutOfBounds,
    StubTooSmall,
    ReadOutOfBounds,
}
