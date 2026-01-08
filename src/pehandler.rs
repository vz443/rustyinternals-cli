use std::io::prelude::*;
use std::fs::File;

use crate::cli_utils;
use crate::errors::errors::ParseError;
use crate::headers::dosheader::DosHeader;
use crate::headers::fileheader::FileHeader;
use crate::headers::optionalheader::{ImageDataDirectory, OptionalHeader};
use crate::headers::signature::{self, Signature};

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
        1 => read_headers (1, path),
        2 => read_headers(2, path),
        3 => read_headers(3, path),
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
        2 => print_file_header(&buffer),
        3 => print_optional_header(&buffer),
        _ => println!("Invalid Option"),
    }
}

fn read_dos(buffer: &[u8]) {
    let header = match DosHeader::parse(buffer) {
        Ok(header) => header,
        Err(ParseError::IncorrectSliceLength) => {
            println!("Error parsing the DosHeader, returning...");
            return;
        }
    };

    println!("--- DOS HEADER ---");
    println!("e_magic:                 0x{:04X}", header.e_magic);
    println!("e_cblp:                  0x{:04X}", header.e_cblp);
    println!("e_cp:                    0x{:04X}", header.e_cp);
    println!("e_crlc:                  0x{:04X}", header.e_crlc);
    println!("e_cparhdr:               0x{:04X}", header.e_cparhdr);
    println!("e_minalloc:              0x{:04X}", header.e_minalloc);
    println!("e_maxalloc:              0x{:04X}", header.e_maxalloc);
    println!("e_ss:                    0x{:04X}", header.e_ss);
    println!("e_sp:                    0x{:04X}", header.e_sp);
    println!("e_csum:                  0x{:04X}", header.e_csum);
    println!("e_ip:                    0x{:04X}", header.e_ip);
    println!("e_cs:                    0x{:04X}", header.e_cs);
    println!("e_lfarlc:                0x{:04X}", header.e_lfarlc);
    println!("e_ovno:                  0x{:04X}", header.e_ovno);

    println!("e_res:");
    println!("  [{:04X}, {:04X}, {:04X}, {:04X}]",
        header.e_res[0],
        header.e_res[1],
        header.e_res[2],
        header.e_res[3],
    );

    println!("e_oemid:                 0x{:04X}", header.e_oemid);
    println!("e_oeminfo:               0x{:04X}", header.e_oeminfo);

    println!("e_res2:");
    print!("  [");
    for i in 0..10 {
        if i < 9 {
            print!("{:04X}, ", header.e_res2[i]);
        } else {
            print!("{:04X}", header.e_res2[i]);
        }
    }
    println!("]");

    println!("e_lfanew:                0x{:08X}", header.e_lfanew);

    println!("----------------------");
}

fn print_sig(buf: &Vec<u8>) {
    let sig = match Signature::parse(buf) {
        Ok(sig) => sig,
        Err(ParseError::IncorrectSliceLength) => { return; },
    };

    println!("{:04X}", sig.signature);
    
    if sig.signature == 0x00004550 {
        println!("Valid Signature");
    }
    else {
        println!("Invalid Signature");
    }
}

fn print_file_header(buf: &[u8]) {
    FileHeader::parse(buf);
}

fn print_optional_header(buf: &[u8]) { 
    let fileheader = FileHeader::parse(buf);
    OptionalHeader::parse(buf, &fileheader);
}

fn initialise_pe(path: &str) {
    //write calls to parse headers and initialise things that are not heavy
    //return bool if succesful else return what is wrong on analysis 
    
}