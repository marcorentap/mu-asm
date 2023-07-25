pub mod assembler {
    #[derive(Debug)]
    struct InstructionDescriptor {
        text: String,
        address: u32,
        mnemonic: String,
        rd: String,
        rs1: String,
        rs2: String,
        imm: String,
    }

    impl InstructionDescriptor {
        fn new() -> Self {
            InstructionDescriptor {
                text: "".to_string(),
                address: 0,
                mnemonic: "".to_string(),
                rd: "".to_string(),
                rs1: "".to_string(),
                rs2: "".to_string(),
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

        inst.text = s.clone();
        inst.mnemonic = words.get(0).unwrap().clone();
        inst.rd = words.get(1).unwrap().clone();
    }

    pub fn assemble(lines: &Vec<String>) {
        let mut addr_counter: u32 = 0;
        let mut label_table: Vec<LabelDescriptor> = Vec::new();
        let mut instruction_table: Vec<InstructionDescriptor> = Vec::new();
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
                instruction_table.push(inst);
            }
        }

        println!("{:?}", label_table);
        println!("\n");
        println!("{:?}", instruction_table);

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
