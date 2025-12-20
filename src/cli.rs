use std::io;
use crate::pehandler;

pub fn handler(){
    
    let mut input = String::new();
    
    while input.to_lowercase() != "x" {
        println!("Rusty Internals CLI Tool");
        println!("Enter PE file path: ");
        io::stdin().read_line(&mut input).expect("geeked");

        pehandler::handle_pe(&input);
    }
}