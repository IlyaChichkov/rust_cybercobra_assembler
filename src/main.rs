use std::io;
use std::io::prelude::*;
use std::fs;
use std::env;

mod decoder;
mod encoder;

fn read_file(file_path: &str) -> Result<(), Box<dyn std::error::Error + 'static>> {
    let foo: String = String::from_utf8_lossy(&fs::read(file_path)?).parse()?;
    
    let mut commands = decoder::decoder::get_decoded_lines(&foo)?;
    let _ = encoder::encoder::encode_commands(&mut commands);
    return Ok(());
}

fn main() { 
    let args: Vec<_> = env::args().collect();

    println!("Program started!");
    if args.len() > 1 {
        let programPath = &args[1];
        let _ = read_file(programPath);
    }
}
