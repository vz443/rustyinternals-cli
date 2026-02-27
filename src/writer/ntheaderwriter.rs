use crate::headers::ntheader::NtHeader;
use super::write_utils;

pub fn write_nt_header(_header: &NtHeader) {
    write_utils::print_header_title("NT HEADER");
    write_utils::print_info("Note: NtHeader structure needs to expose its components");
    write_utils::print_info("This writer will be completed once NtHeader provides proper accessors");
    write_utils::print_section_separator();
}
