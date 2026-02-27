use crate::headers::dosheader::DosHeader;
use crate::diagnostics::diagnostics::ConsoleDiagnosticSink;
use super::write_utils;

const FIELD_WIDTH: usize = 30;

pub fn write_dos_header(header: &DosHeader) {
    write_utils::print_header_title("DOS HEADER");
    
    write_utils::print_field_u16("e_magic:", header.e_magic, FIELD_WIDTH);
    write_utils::print_field_u16("e_cblp:", header.e_cblp, FIELD_WIDTH);
    write_utils::print_field_u16("e_cp:", header.e_cp, FIELD_WIDTH);
    write_utils::print_field_u16("e_crlc:", header.e_crlc, FIELD_WIDTH);
    write_utils::print_field_u16("e_cparhdr:", header.e_cparhdr, FIELD_WIDTH);
    write_utils::print_field_u16("e_minalloc:", header.e_minalloc, FIELD_WIDTH);
    write_utils::print_field_u16("e_maxalloc:", header.e_maxalloc, FIELD_WIDTH);
    write_utils::print_field_u16("e_ss:", header.e_ss, FIELD_WIDTH);
    write_utils::print_field_u16("e_sp:", header.e_sp, FIELD_WIDTH);
    write_utils::print_field_u16("e_csum:", header.e_csum, FIELD_WIDTH);
    write_utils::print_field_u16("e_ip:", header.e_ip, FIELD_WIDTH);
    write_utils::print_field_u16("e_cs:", header.e_cs, FIELD_WIDTH);
    write_utils::print_field_u16("e_lfarlc:", header.e_lfarlc, FIELD_WIDTH);
    write_utils::print_field_u16("e_ovno:", header.e_ovno, FIELD_WIDTH);
    write_utils::print_array_u16("e_res:", &header.e_res);
    write_utils::print_field_u16("e_oemid:", header.e_oemid, FIELD_WIDTH);
    write_utils::print_field_u16("e_oeminfo:", header.e_oeminfo, FIELD_WIDTH);
    write_utils::print_array_u16("e_res2:", &header.e_res2);
    write_utils::print_field_u32("e_lfanew:", header.e_lfanew, FIELD_WIDTH);
    
    write_utils::print_section_separator();
}

pub fn write_dos_header_with_diagnostics(header: &DosHeader, sink: &ConsoleDiagnosticSink) {
    if sink.has_fatal() {
        write_utils::print_subsection("DIAGNOSTICS:");
        write_utils::print_error("Fatal errors encountered while parsing DOS header");
    }
    
    write_dos_header(header);
    
    if header.e_magic == 0x5A4D {
        write_utils::print_validation("Valid DOS signature (MZ)", true);
    } else {
        write_utils::print_validation(&format!("Invalid DOS signature: expected 0x5A4D, got 0x{:04X}", header.e_magic), false);
    }
}
