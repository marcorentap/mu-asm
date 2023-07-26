mod assembler;
mod disassembler;

pub struct MuAsm {}

impl MuAsm {
    pub fn new() -> Self {
        println!("MuAsm new!");
        return MuAsm {};
    }

    pub fn assemble(&self, lines: &Vec<String>) {
        println!("Assembling");
        assembler::assemble(lines);
    }

    pub fn disassemble(&self, lines: &Vec<String>) {
        println!("Disassemble");
        // assembler::assemble(lines);
    }
}
