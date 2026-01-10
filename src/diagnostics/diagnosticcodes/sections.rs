#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SectionCode {
    Overlap,
    SizeMismatch,
    ExecutableWritable,
}
