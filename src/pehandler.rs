use std::io::prelude::*;
use std::fs::File;

use crate::cli_utils;
use crate::diagnostics::diagnostics::ConsoleDiagnosticSink;
use crate::errors::errors::ParseError;
use crate::headers::dosheader::DosHeader;
use crate::headers::fileheader::FileHeader;
use crate::headers::optionalheader::OptionalHeader;
use crate::headers::signature::Signature;
use crate::sections::sections::Sections;
use crate::writer::dosheaderwriter;
use crate::writer::sectionwriter;
use crate::writer::signaturewriter;

//this is a mess but i am primarily working on thte lib crates now, this will be cleaned up later with sequential reading and storage  rather than parsing 1000 times
pub struct PeFile {
    buf: Vec<u8>, 
    dos: DosHeader, 
    file: FileHeader,
    //optional: OptionalHeader,
    //sections: Vec<SectionHeader>,
}

pub fn handle_pe(path: &String){
    initialise_pe(&path);
    let option = cli_utils::get_pe_reader_options(); 
    match option {
        1 => handle_pe_reader(path),
        _ => println!("Invalid Option"),
    }
}

fn handle_pe_reader(path: &String) {
    let option = cli_utils::get_pe_reader_header_options();

    match option {
        1 => read_headers(1, path), // DOS Header
        2 => read_headers(2, path), // PE Signature
        3 => read_headers(3, path), // File Header
        4 => read_headers(4, path), // Optional Header
        5 => read_headers(5, path), // NT Header (Full)
        6 => read_headers(6, path), // Section Headers
        _ => println!("Invalid Option")
    }
}

fn read_headers(name: u8, path: &String) {
    println!("Path is: {}", path);
    let mut file = File::open(path.trim()).unwrap();

    let mut buffer: Vec<u8> = Vec::new();

    file.read_to_end(&mut buffer).unwrap();
    
    match name {
        1 => read_dos(&buffer),
        2 => print_sig(&buffer),
        3 => print_file_header(&buffer),
        4 => print_optional_header(&buffer),
        5 => print_nt_header(&buffer),
        6 => print_section_headers(&buffer),
        _ => println!("Invalid Option"),
    }
}

fn read_dos(buffer: &[u8]) {
    let mut sink = ConsoleDiagnosticSink::new();
    let header = DosHeader::parse(buffer, &mut sink);

    // Print any diagnostics
    sink.print_all();
    
    if sink.has_fatal() {
        println!("\n!!! Fatal errors encountered while parsing DOS header !!!\n");
        return;
    }

    // Use the new writer
    dosheaderwriter::write_dos_header_with_diagnostics(&header, &sink);
}

fn print_section_headers(buf: &[u8]) {
    let fileheader =FileHeader::parse(buf); 
    let  optheader = OptionalHeader::parse(buf, &fileheader);
    let mut pointer =  fileheader.end_of_file_header_pointer as usize;
    let sections = Sections::parse_all_sections(buf, &mut pointer.clone(), &fileheader.number_of_sections); // fix the clone and the mutable borrow here
    sectionwriter::write_sections(&sections);
}

#[allow(dead_code)]
fn print_sig(buf: &Vec<u8>) {
    let sig = match Signature::parse(buf) {
        Ok(sig) => sig,
        Err(ParseError::IncorrectSliceLength) => { 
            println!("Error parsing PE signature");
            return; 
        },
    };

    // Use the new writer
    signaturewriter::write_signature(&sig);
}

fn print_file_header(buf: &[u8]) {
    FileHeader::parse(buf);
}

fn print_optional_header(buf: &[u8]) { 
    let fileheader = FileHeader::parse(buf);
    OptionalHeader::parse(buf, &fileheader);
}

fn print_nt_header(buf: &[u8]) {
    print_sig(&buf.to_vec());
    print_file_header(buf);
    print_optional_header(buf);
}

fn initialise_pe(_path: &str) {
    //write calls to parse headers and initialise things that are not heavy
    //return bool if succesful else return what is wrong on analysis 
}