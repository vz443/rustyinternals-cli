use crate::diagnostics::diagnostics::Diagnostic;

pub trait Validator {
    fn validate() -> Vec<Diagnostic>;
}