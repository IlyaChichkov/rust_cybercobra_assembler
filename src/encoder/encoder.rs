use core::{arch, num};

use crate::decoder::decoder::{self, Command, Instructions};


fn get_alu_op(operation: &Instructions) -> Vec<u8> {
    match operation {
        Instructions::ADD   =>  vec![0, 0, 0, 0, 0],
        Instructions::SUB   =>  vec![0, 1, 0, 0, 0],
        Instructions::XOR   =>  vec![0, 0, 1, 0, 0],
        Instructions::OR    =>  vec![0, 0, 1, 1, 0],
        Instructions::AND   =>  vec![0, 0, 1, 1, 1],
        Instructions::SRA   =>  vec![0, 1, 1, 0, 1],
        Instructions::SLL   =>  vec![0, 0, 0, 0, 1],
        Instructions::SRL   =>  vec![0, 0, 1, 0, 1],
        Instructions::SLTS  =>  vec![0, 0, 0, 1, 0],
        Instructions::SLTU  =>  vec![0, 0, 0, 1, 1],
        Instructions::BLT   =>  vec![1, 1, 1, 0, 0],
        Instructions::BLTU  =>  vec![1, 1, 1, 1, 0],
        Instructions::BGE   =>  vec![1, 1, 1, 0, 1],
        Instructions::BGEU  =>  vec![1, 1, 1, 1, 1],
        Instructions::BEQ   =>  vec![1, 1, 0, 0, 0],
        Instructions::BNE   =>  vec![1, 1, 0, 0, 1],
        Instructions::J     =>  vec![0, 0, 0, 0, 0],
        _ => panic!("[get_op_code] Invalid instruction."),
    }
}

fn get_op_code(instruction: &Instructions) -> Vec<u8> {
    match instruction {
        Instructions::None  =>  vec![0, 0, 0, 0, 0, 0, 0, 0], // todo!
        Instructions::ADD   =>  vec![0, 0, 1, 1, 0, 0, 1, 1],
        Instructions::SUB   =>  vec![0, 0, 1, 1, 0, 0, 1, 1],
        Instructions::XOR   =>  vec![0, 0, 1, 1, 0, 0, 1, 1],
        Instructions::OR    =>  vec![0, 0, 1, 1, 0, 0, 1, 1],
        Instructions::AND   =>  vec![0, 0, 1, 1, 0, 0, 1, 1],
        Instructions::SRA   =>  vec![0, 0, 1, 1, 0, 0, 1, 1],
        Instructions::SLL   =>  vec![0, 0, 1, 1, 0, 0, 1, 1],
        Instructions::SRL   =>  vec![0, 0, 1, 1, 0, 0, 1, 1],
        Instructions::SLTS  =>  vec![0, 0, 0, 0, 0, 0, 0, 0], // todo!
        Instructions::SLTU  =>  vec![0, 0, 1, 1, 0, 0, 1, 1],
        Instructions::BLT   =>  vec![0, 1, 1, 0, 0, 0, 1, 1],
        Instructions::BLTU  =>  vec![0, 1, 1, 0, 0, 0, 1, 1],
        Instructions::BGE   =>  vec![0, 1, 1, 0, 0, 0, 1, 1],
        Instructions::BGEU  =>  vec![0, 1, 1, 0, 0, 0, 1, 1],
        Instructions::BEQ   =>  vec![0, 1, 1, 0, 0, 0, 1, 1],
        Instructions::BNE   =>  vec![0, 1, 1, 0, 0, 0, 1, 1],
        Instructions::LI    =>  vec![0, 0, 1, 1, 0, 1, 1, 1], // check!
        Instructions::J     =>  vec![0, 1, 1, 0, 1, 1, 1, 1], // check!
        Instructions::CIN   =>  vec![0, 0, 0, 0, 0, 0, 0, 0], // todo!
        Instructions::COUT  =>  vec![0, 0, 0, 0, 0, 0, 0, 0], // todo!
        _ => panic!("[get_op_code] Invalid instruction."),
    }
}

fn string_to_binary_array(data: &str) -> Vec<u8> {
    let number = i8::from_str_radix(data, 10).expect("Failed to parse string as number") as u8;
    let mut binary_array: Vec<u8> = vec![];

    for i in 0..8 {
        binary_array.push((number >> (7 - i)) & 1);  // Shift and extract bits
    }
    return binary_array;
}

fn create_binary_array() -> Vec<u8> {
    let mut data: Vec<u8> = Vec::with_capacity(32);  // Pre-allocate for efficiency
    for _ in 0..32 {
      data.push(if rand::random() { 0 } else { 1 });  // Push random 0 or 1
    }
    data
} 

fn binary_array_to_int(binary_data: &Vec<u8>) -> u32 {
    let mut value: u32 = 0;
    for (i, bit) in binary_data.iter().enumerate() {
        value |= (*bit as u32) << (31 - i); // Shift and set bits
    }
    //println!("Converted u32 value: {:#10x}", value);
    return value;
}

fn print_binary_array(binary_data: &Vec<u8>) {
    let mut fbits = 0;
    for i in 0..32 {
        if let Some(val) = binary_data.get(i) {
            print!("{}", val);
        }
        else {
            print!("0");
        }
        if fbits >= 3 {
          print!(".");
          fbits = 0;
        }
        else {
          fbits += 1;
        }
      }
}

pub fn encode_commands(commands: &mut Vec<Command>) -> Result<(), String>{
    let mut lines: Vec<u32> = vec![];

    const ASIZE_3_0: [&'static u8; 3] = [&0, &0, &0];
    const ASIZE_3_1: [&'static u8; 3] = [&0, &0, &1];
    const ASIZE_3_2: [&'static u8; 3] = [&0, &1, &0];
    const ASIZE_3_3: [&'static u8; 3] = [&0, &1, &1];
    const ASIZE_3_4: [&'static u8; 3] = [&1, &0, &0];
    const ASIZE_3_5: [&'static u8; 3] = [&1, &0, &1];
    const ASIZE_3_6: [&'static u8; 3] = [&1, &1, &0];
    const ASIZE_3_7: [&'static u8; 3] = [&1, &1, &1];

    for command in &commands[..] {
        let mut command_binary_array: Vec<u8> = vec![];

        match command.instruction {
            Instructions::None   =>  {
                
            }
            Instructions::ADD  |
            Instructions::SUB  |
            Instructions::XOR  |
            Instructions::OR   |
            Instructions::SRA  |
            Instructions::SLL  |
            Instructions::SRL  |
            Instructions::AND  |
            Instructions::SLTS |
            Instructions::SLTU  =>  {
                println!("Add instruction: {}", &command.instruction);

                command_binary_array.push(0);
                command_binary_array.push(0);
                command_binary_array.push(0);
                // WS
                command_binary_array.push(1);
                // ALUop
                command_binary_array.extend(&get_alu_op(&command.instruction));
                // rs1
                command_binary_array.extend(&string_to_binary_array(&command.arguments[1].c_value)[3..]);
                // rs2
                command_binary_array.extend(&string_to_binary_array(&command.arguments[2].c_value)[3..]);
                // func 3
                command_binary_array.push(0);
                command_binary_array.push(0);
                command_binary_array.push(0);
                command_binary_array.push(0);
                command_binary_array.push(0);
                command_binary_array.push(0);
                command_binary_array.push(0);
                command_binary_array.push(0);
                // rd
                command_binary_array.extend(&string_to_binary_array(&command.arguments[0].c_value)[3..]);
                
                let cmd = binary_array_to_int(&command_binary_array);
                lines.push(cmd);
            }
            Instructions::BLT  |
            Instructions::BLTU |
            Instructions::BGE  |
            Instructions::BGEU |
            Instructions::BEQ  |
            Instructions::BNE   =>  {
                println!("Add instruction: {}", &command.instruction);
                println!("Arguments:");
                for arg in &command.arguments {
                    println!("> {}", arg.c_value);
                }

                command_binary_array.push(0);
                command_binary_array.push(1);
                command_binary_array.push(0);
                // WS
                command_binary_array.push(0);
                // ALUop
                command_binary_array.extend(&get_alu_op(&command.instruction));
                // rs1
                command_binary_array.extend(&string_to_binary_array(&command.arguments[0].c_value)[3..]);
                // rs2
                command_binary_array.extend(&string_to_binary_array(&command.arguments[1].c_value)[3..]);
                // offset
                command_binary_array.extend(&string_to_binary_array(&command.arguments[2].c_value));
                // WA
                command_binary_array.push(0);
                command_binary_array.push(0);
                command_binary_array.push(0);
                command_binary_array.push(0);
                command_binary_array.push(0);
                
                let cmd = binary_array_to_int(&command_binary_array);
                lines.push(cmd);
            }
            Instructions::LI    =>  {

            }
            Instructions::J     =>  {
                println!("Add instruction: {}", &command.instruction);
                println!("Arguments:");
                for arg in &command.arguments {
                    println!("> {}", arg.c_value);
                }

                command_binary_array.push(1);
                command_binary_array.push(0);
                command_binary_array.push(0);
                // WS
                command_binary_array.push(0);
                // ALUop
                command_binary_array.extend(&get_alu_op(&command.instruction));
                // rs1
                command_binary_array.push(0);
                command_binary_array.push(0);
                command_binary_array.push(0);
                command_binary_array.push(0);
                command_binary_array.push(0);
                // rs2
                command_binary_array.push(0);
                command_binary_array.push(0);
                command_binary_array.push(0);
                command_binary_array.push(0);
                command_binary_array.push(0);
                // offset
                command_binary_array.extend(&string_to_binary_array(&command.arguments[0].c_value));
                // WA
                command_binary_array.push(0);
                command_binary_array.push(0);
                command_binary_array.push(0);
                command_binary_array.push(0);
                command_binary_array.push(0);
                
                let cmd = binary_array_to_int(&command_binary_array);
                lines.push(cmd);
            }
            Instructions::CIN   =>  {

            }
            Instructions::COUT  =>  {

            }
        }
    }

    for l in lines {
        println!("-> {:#10x}", l);
    }
    return Ok(());
}