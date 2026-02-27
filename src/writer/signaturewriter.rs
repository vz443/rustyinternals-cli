use crate::headers::signature::Signature;
use super::write_utils;

const FIELD_WIDTH: usize = 30;
const PE_SIGNATURE: u32 = 0x00004550;

pub fn write_signature(sig: &Signature) {
    write_utils::print_header_title("PE SIGNATURE");
    
    write_utils::print_field_u32("Signature:", sig.signature, FIELD_WIDTH);
    
    if sig.signature == PE_SIGNATURE {
        write_utils::print_validation("Valid PE signature", true);
        write_utils::print_info("Signature matches PE\\0\\0 (0x00004550)");
    } else {
        write_utils::print_validation("Invalid PE signature", false);
        write_utils::print_error(&format!("Expected 0x{:08X}, found 0x{:08X}", PE_SIGNATURE, sig.signature));
    }
    
    write_utils::print_section_separator();
}
