use std::io;
use std::io::prelude::*;
use std::fs;
use std::env;

mod cb_core;
mod decoder;

use crate::decoder::decoder::Instructions;

fn readFile(file_path: &str) -> Result<(), Box<dyn std::error::Error + 'static>> {
    let foo: String = String::from_utf8_lossy(&fs::read(file_path)?).parse()?;
    // println!("> {}", foo);
    decoder::decoder::get_decoded_lines(&foo);
    
    return Ok(());
}

fn main() { 
    let args: Vec<_> = env::args().collect();

    if args.len() > 1 {
        let programPath = &args[1];
        readFile(programPath);
    }
}
