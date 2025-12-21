use std::io::prelude::*;
use std::fs::File;

use crate::cli_utils;
use crate::headers::dosheader::DosHeader;

pub struct PeFile {
    buf: Vec<u8>, 
    dos: DosHeader, 
    //coff: CoffHeader,
    //optional: OptionalHeader,
    //sections: Vec<SectionHeader>,
}

pub fn handle_pe(path: &String){
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
        2 => read_nt_headers(&buffer),
        _ => println!("Invalid Option"),
    }
}

fn read_dos(buffer: &[u8]) {
    let header = DosHeader::parse(buffer).unwrap();

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


fn read_nt_headers(buffer: &Vec<u8>) {
    let buffer = buffer.clone();

    let e_lfanew = &buffer[0x3c..0x40];
    let mut ntpointer = u32::from_le_bytes(e_lfanew.try_into().unwrap());

    print!("NT SIGNATURE: ");
    
    for i in 0..4 {
        let digits = &buffer[ntpointer as usize + i];
        print!("{:02X} ", digits);
    }
    println!();
    println!();

    ntpointer += 4;
    print!("NT File Header: ");
    println!();

    let machine = &buffer[ntpointer as usize..ntpointer as usize + 2];

    let machine = u16::from_le_bytes(machine.try_into().unwrap());

    print!("Machine: {:02X}", machine);
    if machine == 0x8664 {
        print!(" (AMD64)");
    }
    else if machine == 0x14c {
        print!(" (i386");
    }
    
    println!();

    println!("Optional Header");


}


