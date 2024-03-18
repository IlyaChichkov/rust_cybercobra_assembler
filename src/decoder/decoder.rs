pub enum Instructions {
    None,
    ADD,
    SUB,
    XOR,
    OR,
    AND,
    SRA,
    SLL,
    SRL,
    SLTS,
    SLTU,
    BLT,
    BLTU,
    BGE,
    BGEU,
    BEQ,
    BNE,
    LI,
    J,
    CIN,
    COUT
}

pub enum InstructionType {
    None,
    Compute,
    Branch,
    Li,
    In,
    Jump
}

impl std::fmt::Display for Instructions {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
        Instructions::None      =>      write!(f, "None"),
        Instructions::ADD       =>      write!(f, "ADD"),
        Instructions::SUB       =>      write!(f, "SUB"),
        Instructions::XOR       =>      write!(f, "XOR"),
        Instructions::OR        =>      write!(f, "OR"),
        Instructions::AND       =>      write!(f, "AND"),
        Instructions::SRA       =>      write!(f, "SRA"),
        Instructions::SLL       =>      write!(f, "SLL"),
        Instructions::SRL       =>      write!(f, "SRL"),
        Instructions::SLTS      =>      write!(f, "SLTS"),
        Instructions::SLTU      =>      write!(f, "SLTU"),
        Instructions::BLT       =>      write!(f, "BLT"),
        Instructions::BLTU      =>      write!(f, "BLTU"),
        Instructions::BGE       =>      write!(f, "BGE"),
        Instructions::BGEU      =>      write!(f, "BGEU"),
        Instructions::BEQ       =>      write!(f, "BEQ"),
        Instructions::BNE       =>      write!(f, "BNE"),
        Instructions::LI        =>      write!(f, "LI"),
        Instructions::J         =>      write!(f, "J"),
        Instructions::CIN       =>      write!(f, "CIN"),
        Instructions::COUT      =>      write!(f, "COUT"),
    }
  }
}

pub enum ArgType {
    None,
    Register,
    Constant,
    Label
}

impl std::fmt::Display for ArgType {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      ArgType::None => write!(f, "None"),
      ArgType::Register => write!(f, "Register"),
      ArgType::Constant => write!(f, "Constant"),
      ArgType::Label => write!(f, "Label"),
    }
  }
}

pub struct CommandArg {
    pub c_type: ArgType,
    pub c_value: String
}

pub struct Command {
    pub text: String,
    pub instruction: Instructions,
    pub instruction_type: InstructionType,
    pub arguments: Vec<CommandArg>,
    pub index: u32,
    pub has_label: bool
}

pub struct Label {
    pub name: String,
    pub index: u32
}

pub fn get_command_instruction(command_line: &String) -> Instructions {
    let parts = command_line.split_whitespace().collect::<Vec<_>>();
    if parts.len() < 2 {
        panic!("Invalid instruction format: {}", command_line);
    }
    let opcode = parts[0].to_lowercase();
    match opcode.as_str() {
        "add" => Instructions::ADD,
        "sub" => Instructions::SUB,
        "xor" => Instructions::XOR,
        "or" => Instructions::OR,
        "and" => Instructions::AND,
        "sra" => Instructions::SRA,
        "sll" => Instructions::SLL,
        "srl" => Instructions::SRL,
        "slts" => Instructions::SLTS,
        "sltu" => Instructions::SLTU,
        "blt" => Instructions::BLT,
        "bltu" => Instructions::BLTU,
        "bge" => Instructions::BGE,
        "bgeu" => Instructions::BGEU,
        "beq" => Instructions::BEQ,
        "bne" => Instructions::BNE,
        "li" => Instructions::LI,
        "j" => Instructions::J,
        "cin" => Instructions::CIN,
        "cout" => Instructions::COUT,
        _ => panic!("Invalid instruction: {}", command_line),
      }
}

pub fn check_command_has_label(instruction: &Instructions) -> bool {
    match instruction {
        Instructions::None => {
            panic!("Unknown instruction: None");
        }
        Instructions::ADD   =>  false,
        Instructions::SUB   =>  false,
        Instructions::XOR   =>  false,
        Instructions::OR    =>  false,
        Instructions::AND   =>  false,
        Instructions::SRA   =>  false,
        Instructions::SLL   =>  false,
        Instructions::SRL   =>  false,
        Instructions::SLTS  =>  false,
        Instructions::SLTU  =>  false,
        Instructions::BLT   =>  true,
        Instructions::BLTU  =>  true,
        Instructions::BGE   =>  true,
        Instructions::BGEU  =>  true,
        Instructions::BEQ   =>  true,
        Instructions::BNE   =>  true,
        Instructions::LI    =>  false,
        Instructions::J     =>  true,
        Instructions::CIN   =>  false,
        Instructions::COUT  =>  false,
        _ => panic!("[check_command_has_label] Invalid instruction."),
      }
}

fn get_arguments(instruction: &Instructions, command_line: &String) -> Result<Vec<CommandArg>, String> {
    let mut arguments: Vec<CommandArg> = vec![];
    let parts = command_line.split_whitespace().collect::<Vec<_>>();
    let parts = parts[1..].to_vec();
       
    match instruction {
        Instructions::None => {
            panic!("Unknown instruction: None");
        }
        Instructions::ADD   =>  {
            if parts.len() != 3 {
                return Err("Not enough arguments!".to_string());
            }
            arguments.push(CommandArg { c_type: (ArgType::Register), c_value: (parts[0].to_string()) });
            arguments.push(CommandArg { c_type: (ArgType::Register), c_value: (parts[1].to_string()) });
            arguments.push(CommandArg { c_type: (ArgType::Register), c_value: (parts[2].to_string()) });
        }
        Instructions::SUB   =>  {
            if parts.len() != 3 {
                return Err("Not enough arguments!".to_string());
            }
            arguments.push(CommandArg { c_type: (ArgType::Register), c_value: (parts[0].to_string()) });
            arguments.push(CommandArg { c_type: (ArgType::Register), c_value: (parts[1].to_string()) });
            arguments.push(CommandArg { c_type: (ArgType::Register), c_value: (parts[2].to_string()) });
        }
        Instructions::XOR   =>  {
            if parts.len() != 3 {
                return Err("Not enough arguments!".to_string());
            }
            arguments.push(CommandArg { c_type: (ArgType::Register), c_value: (parts[0].to_string()) });
            arguments.push(CommandArg { c_type: (ArgType::Register), c_value: (parts[1].to_string()) });
            arguments.push(CommandArg { c_type: (ArgType::Register), c_value: (parts[2].to_string()) });
        }
        Instructions::OR    =>  {
            if parts.len() != 3 {
                return Err("Not enough arguments!".to_string());
            }
            arguments.push(CommandArg { c_type: (ArgType::Register), c_value: (parts[0].to_string()) });
            arguments.push(CommandArg { c_type: (ArgType::Register), c_value: (parts[1].to_string()) });
            arguments.push(CommandArg { c_type: (ArgType::Register), c_value: (parts[2].to_string()) });
        }
        Instructions::AND   =>  {
            if parts.len() != 3 {
                return Err("Not enough arguments!".to_string());
            }
            arguments.push(CommandArg { c_type: (ArgType::Register), c_value: (parts[0].to_string()) });
            arguments.push(CommandArg { c_type: (ArgType::Register), c_value: (parts[1].to_string()) });
            arguments.push(CommandArg { c_type: (ArgType::Register), c_value: (parts[2].to_string()) });
        }
        Instructions::SRA   =>  {
            if parts.len() != 3 {
                return Err("Not enough arguments!".to_string());
            }
            arguments.push(CommandArg { c_type: (ArgType::Register), c_value: (parts[0].to_string()) });
            arguments.push(CommandArg { c_type: (ArgType::Register), c_value: (parts[1].to_string()) });
            arguments.push(CommandArg { c_type: (ArgType::Register), c_value: (parts[2].to_string()) });
        }
        Instructions::SLL   =>  {
            if parts.len() != 3 {
                return Err("Not enough arguments!".to_string());
            }
            arguments.push(CommandArg { c_type: (ArgType::Register), c_value: (parts[0].to_string()) });
            arguments.push(CommandArg { c_type: (ArgType::Register), c_value: (parts[1].to_string()) });
            arguments.push(CommandArg { c_type: (ArgType::Register), c_value: (parts[2].to_string()) });
        }
        Instructions::SRL   =>  {
            if parts.len() != 3 {
                return Err("Not enough arguments!".to_string());
            }
            arguments.push(CommandArg { c_type: (ArgType::Register), c_value: (parts[0].to_string()) });
            arguments.push(CommandArg { c_type: (ArgType::Register), c_value: (parts[1].to_string()) });
            arguments.push(CommandArg { c_type: (ArgType::Register), c_value: (parts[2].to_string()) });
        }
        Instructions::SLTS  =>  {
            if parts.len() != 3 {
                return Err("Not enough arguments!".to_string());
            }
            arguments.push(CommandArg { c_type: (ArgType::Register), c_value: (parts[0].to_string()) });
            arguments.push(CommandArg { c_type: (ArgType::Register), c_value: (parts[1].to_string()) });
            arguments.push(CommandArg { c_type: (ArgType::Register), c_value: (parts[2].to_string()) });
        }
        Instructions::SLTU  =>  {
            if parts.len() != 3 {
                return Err("Not enough arguments!".to_string());
            }
            arguments.push(CommandArg { c_type: (ArgType::Register), c_value: (parts[0].to_string()) });
            arguments.push(CommandArg { c_type: (ArgType::Register), c_value: (parts[1].to_string()) });
            arguments.push(CommandArg { c_type: (ArgType::Register), c_value: (parts[2].to_string()) });
        }
        Instructions::BLT   =>  {
            if parts.len() != 3 {
                return Err("Not enough arguments!".to_string());
            }
            arguments.push(CommandArg { c_type: (ArgType::Register),  c_value: (parts[0].to_string()) });
            arguments.push(CommandArg { c_type: (ArgType::Register),  c_value: (parts[1].to_string()) });
            arguments.push(CommandArg { c_type: (ArgType::Label),     c_value: (parts[2].to_string()) });
        }
        Instructions::BLTU  =>  {
            if parts.len() != 3 {
                return Err("Not enough arguments!".to_string());
            }
            arguments.push(CommandArg { c_type: (ArgType::Register),  c_value: (parts[0].to_string()) });
            arguments.push(CommandArg { c_type: (ArgType::Register),  c_value: (parts[1].to_string()) });
            arguments.push(CommandArg { c_type: (ArgType::Label),     c_value: (parts[2].to_string()) });
        }
        Instructions::BGE   =>  {
            if parts.len() != 3 {
                return Err("Not enough arguments!".to_string());
            }
            arguments.push(CommandArg { c_type: (ArgType::Register),  c_value: (parts[0].to_string()) });
            arguments.push(CommandArg { c_type: (ArgType::Register),  c_value: (parts[1].to_string()) });
            arguments.push(CommandArg { c_type: (ArgType::Label),     c_value: (parts[2].to_string()) });
        }
        Instructions::BGEU  =>  {
            if parts.len() != 3 {
                return Err("Not enough arguments!".to_string());
            }
            arguments.push(CommandArg { c_type: (ArgType::Register),  c_value: (parts[0].to_string()) });
            arguments.push(CommandArg { c_type: (ArgType::Register),  c_value: (parts[1].to_string()) });
            arguments.push(CommandArg { c_type: (ArgType::Label),     c_value: (parts[2].to_string()) });
        }
        Instructions::BEQ   =>  {
            if parts.len() != 3 {
                return Err("Not enough arguments!".to_string());
            }
            arguments.push(CommandArg { c_type: (ArgType::Register),  c_value: (parts[0].to_string()) });
            arguments.push(CommandArg { c_type: (ArgType::Register),  c_value: (parts[1].to_string()) });
            arguments.push(CommandArg { c_type: (ArgType::Label),     c_value: (parts[2].to_string()) });
        }
        Instructions::BNE   =>  {
            if parts.len() != 3 {
                return Err("Not enough arguments!".to_string());
            }
            arguments.push(CommandArg { c_type: (ArgType::Register),  c_value: (parts[0].to_string()) });
            arguments.push(CommandArg { c_type: (ArgType::Register),  c_value: (parts[1].to_string()) });
            arguments.push(CommandArg { c_type: (ArgType::Label),     c_value: (parts[2].to_string()) });
        }
        Instructions::LI    =>  {
            if parts.len() != 2 {
                return Err("Not enough arguments!".to_string());
            }
            arguments.push(CommandArg { c_type: (ArgType::Register), c_value: (parts[0].to_string()) });
            arguments.push(CommandArg { c_type: (ArgType::Constant), c_value: (parts[1].to_string()) });
        }
        Instructions::J     =>  {
            if parts.len() != 1 {
                return Err("Not enough arguments!".to_string());
            }
            arguments.push(CommandArg { c_type: (ArgType::Label), c_value: (parts[0].to_string()) });
        }
        Instructions::CIN   =>  {

        }
        Instructions::COUT  =>  {

        }
        _ => panic!("[get_arguments] Invalid instruction."),
      }
    return Ok(arguments);
}

pub fn command_decoder(command_line: String, line_index: u32) -> Result<Command, String> {
    let instruction = get_command_instruction(&command_line);
    let has_label = check_command_has_label(&instruction);
    let arguments: Vec<CommandArg> = get_arguments(&instruction, &command_line)?;
    let cmd = Command { text: (command_line), index: (line_index), instruction: (instruction), arguments: (arguments), has_label: (has_label), instruction_type: (InstructionType::None) };
    return Ok(cmd);
}

pub fn replace_registers(command: &mut Command) -> Result<(), String>{
    let mut reg_indexes: Vec<usize> = vec![];
    for (i, arg) in command.arguments.iter().enumerate(){
        if let ArgType::Register = arg.c_type {
            reg_indexes.push(i);
        }
    }
    for index in &reg_indexes[..] {
        command.arguments[*index].c_value = command.arguments[*index].c_value.replace("x", "").to_string();
    }
    return Ok(());
}

pub fn fill_labels(command: &mut Command, labels: &[Label]) -> Result<(), String>{
    if !command.has_label {
        return Ok(());
    }
    let mut label_name = "";
    let mut label_arg_index = 0;
    for (i, arg) in command.arguments.iter().enumerate(){
        if let ArgType::Label = arg.c_type {
            label_name = &arg.c_value;
            label_arg_index = i;
        }
    }
    let mut offset: i64 = 0;
    for label in &labels[..] {
        if label.name == label_name {
            let command_index: i32 = command.index as i32;
            let label_index: i32 = label.index as i32;
            offset = (label_index - command_index) as i64;
            command.arguments[label_arg_index].c_value = offset.to_string();
            break;
        }
    }
    return Ok(());
}

pub fn get_decoded_lines(data: &str) -> Result<Vec<Command>, String>{
    let mut labels: Vec<Label> = vec![];
    let mut commands: Vec<Command>  = vec![];

    let mut instruction_cnt = 0;

    let lines: Vec<String> = data.lines()
        // Collect the lines into a vector of strings
        .map(|line| line.to_string())
        .collect();

    for line in lines {
        if line.len() > 0
        {
            let mut command_line = line;
            // remove commentary
            let hash_index = command_line.find("#");
            if let Some(index) = hash_index {
                command_line.replace_range(index.., "")
            }

            if command_line.chars().last().unwrap() == ':'
            {
                let label_name = command_line.replace(":", "").replace(" ", "");
                labels.push(Label { name: (label_name), index: (instruction_cnt) });
            }
            else {
                let command = command_line.replace(",", "");
                commands.push(command_decoder(command, instruction_cnt)?);
                instruction_cnt += 1;
            }
        }
    }

    for command in &mut commands {
        fill_labels(command, &labels[..])?;
        replace_registers(command);
    }
    
    return Ok(commands);
}