use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

use crate::cli_utils;

pub fn handle_pe(path: &String){
    let mut input = String::new(); 

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

fn read_dos(buffer: &Vec<u8>) {
    let offset = &buffer[0x00..0x40];
    
    if buffer[0x00] != 77 && buffer[0x01] != 90 {
        println!("Invalid PE File, MZ Doesn't exist");
    }

    for b in offset {
        print!("{:02X} ", b);
    }

    let e_lfanew = &buffer[0x3c..0x40];
    let ntpointer = u32::from_le_bytes(e_lfanew.try_into().unwrap());

    //finish reading the rest of dos but idk what is important maybe dump everything in the struct 
}

fn read_nt_headers(buffer: &Vec<u8>) {
    let e_lfanew = &buffer[0x3c..0x40];
    let ntpointer = u32::from_le_bytes(e_lfanew.try_into().unwrap());

    print!("NT SIGNATURE: ");
    
    for i in 0..4 {
        let digits = &buffer[ntpointer as usize + i];

        print!("{:02X} ", digits);
    }
    println!();
}


