use crate::mu_asm::InstructionDescriptor;
use crate::mu_asm::MuAsm;

impl MuAsm {
    pub fn parse_instruction(&self, s: &str) -> InstructionDescriptor {
        let mut inst = InstructionDescriptor::new();
        let words: Vec<String> = s.split_whitespace().map(|word| word.to_string()).collect();
        let default_field: String = "".to_string();

        inst.text = s.to_string().to_owned();
        inst.mnemonic = words.get(0).unwrap().clone();
        inst.field1 = words.get(1).unwrap_or(&default_field).clone();
        inst.field2 = words.get(2).unwrap_or(&default_field).clone();
        inst.field3 = words.get(3).unwrap_or(&default_field).clone();
        inst
    }
}
