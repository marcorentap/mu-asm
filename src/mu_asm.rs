pub mod assembler {
    #[derive(Debug)]
    struct InstructionDescriptor {
        text: String,
        address: u32,
        mnemonic: String,
        field1: String,
        field2: String,
        field3: String,
        imm: String,
    }

    impl InstructionDescriptor {
        fn new() -> Self {
            InstructionDescriptor {
                text: "".to_string(),
                address: 0,
                mnemonic: "".to_string(),
                field1: "".to_string(),
                field2: "".to_string(),
                field3: "".to_string(),
                imm: "".to_string(),
            }
        }
    }

    #[derive(Debug)]
    struct LabelDescriptor {
        address: u32,
        name: String,
    }

    fn parse_instruction(s: &String) -> InstructionDescriptor {
        let mut inst = InstructionDescriptor::new();
        let words: Vec<String> = s.split_whitespace().map(|word| word.to_string()).collect();
        let default_field: String = "".to_string();

        inst.text = s.clone();
        inst.mnemonic = words.get(0).unwrap().clone();
        inst.field1 = words.get(1).unwrap_or(&default_field).clone();
        inst.field2 = words.get(2).unwrap_or(&default_field).clone();
        inst.field3 = words.get(3).unwrap_or(&default_field).clone();
        inst
    }

    pub fn assemble(lines: &Vec<String>) {
        let mut addr_counter: u32 = 0;
        let mut label_table: Vec<LabelDescriptor> = Vec::new();
        let mut inst_table: Vec<InstructionDescriptor> = Vec::new();
        let mut lines = lines.clone();

        lines = remove_comments(&lines);
        lines = remove_empty_lines(&lines);

        for line in &lines {
            if line.starts_with('@') {
                label_table.push(LabelDescriptor {
                    address: addr_counter,
                    name: line.clone(),
                })
            } else {
                let mut inst: InstructionDescriptor = parse_instruction(&line);
                inst.address = addr_counter;
                addr_counter += 8;
                inst_table.push(inst);
            }
        }

        for label in label_table {
            println!("{:?}", label);
        }

        println!("\n");

        for inst in inst_table {
            println!("{:?}", inst);
        }

        // let mut instructions: Vec<Instruction> =
        // lines.iter().map(|line| Instruction::new(line)).collect();
    }

    fn remove_empty_lines(lines: &Vec<String>) -> Vec<String> {
        lines
            .into_iter()
            .filter(|line| !line.is_empty())
            .map(|line| line.trim().to_string())
            .collect()
    }

    fn remove_comments(lines: &Vec<String>) -> Vec<String> {
        lines
            .into_iter()
            .map(|line| match line.split_once(';') {
                Some(pair) => pair.0.to_string(),
                None => line.to_string(),
            })
            .collect()
    }
}
