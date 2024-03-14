pub enum Instructions {
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

pub enum ArgType {
    None,
    Register,
    Constant
}

pub struct CommandArg {
    pub argType: ArgType
}

pub struct Command {
    pub text: String,
    pub instruction: Instructions,
    pub arguments: Vec<CommandArg>,
    pub index: u32
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

pub fn command_decoder(command_line: String, lineIndex: u32) -> Command {
    let instruction = get_command_instruction(&command_line);
    let arguments: Vec<CommandArg> = vec![];
    let cmd = Command { text: (command_line), index: (lineIndex), instruction: (instruction), arguments: (arguments) };
    return cmd;
}

pub fn get_decoded_lines(data: &str) {
    let mut labels: Vec<Label> = vec![];
    let mut commands: Vec<Command>  = vec![];

    let mut instruction_cnt = 0;

    let lines: Vec<String> = data.lines()
        // Collect the lines into a vector of strings
        .map(|line| line.to_string())
        .collect();

    for line in lines {
        if(line.len() > 0)
        {
            let mut commandLine = line;
            // remove commentary
            let hash_index = commandLine.find("#");
            if let Some(index) = hash_index {
                commandLine.replace_range(index.., "")
            }

            if(commandLine.chars().last().unwrap() == ':')
            {
                let label_name = commandLine.replace(":", "").replace(" ", "");
                //println!("label: _{}_ #{}", line, instruction_cnt);
                labels.push(Label { name: (label_name), index: (instruction_cnt) });
            }
            else {
                //println!("line : _{}_ #{}", line, instruction_cnt);
                commands.push(command_decoder(commandLine, instruction_cnt));
                instruction_cnt += 1;
            }
        }
        else {
            //println!("empty:");
        }
    }

    
    for label in labels {
        println!("label: {} #{}", label.name, label.index);
    }
    for command in commands {
        println!("command: {} #{}", command.text, command.index);
    }
}