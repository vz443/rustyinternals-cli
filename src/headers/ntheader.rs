use crate::headers::{fileheader::FileHeader, optionalheader::OptionalHeader, signature::Signature};


pub struct NtHeader {
    signature: Signature,
    fileheader: FileHeader,
    optionalheader: OptionalHeader, 
}
