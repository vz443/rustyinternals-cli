use std::io;

pub fn get_pe_reader_options() -> u8 {
   let mut input = String::new();

   while input != "x" {
        println!("Choose inspection options");
        println!("1. Read Headers");
        io::stdin().read_line(&mut input).expect("geeked");
        let is_num = input.trim().parse::<u8>().is_ok();

        println!("IsNum: {}, input: {}", is_num, input); 

        if is_num {
            return input.trim().parse::<u8>().unwrap();
        }
        else {
            0;
        }
        
        input.clear();
   } 
   0
}

pub fn get_pe_reader_header_options() -> u8 {
    
    let mut input = String::new();

    while input.to_lowercase() != "x" {
        println!("which header to dump?");
        println!("1. dos header");
        println!("2. NT Header");
        
        io::stdin().read_line(&mut input).expect("geeked");
        let is_num = input.trim().parse::<u8>().is_ok();
        if is_num {
            return input.trim().parse::<u8>().unwrap();
        }
        else {
            0;
        }

        input.clear();
    }

    0
}