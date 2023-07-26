use std::io::BufRead;
use std::io::Write;

mod assembler;
mod disassembler;

pub struct MuAsm {}

impl MuAsm {
    pub fn new() -> Self {
        println!("MuAsm new!");
        return MuAsm {};
    }

    pub fn assemble(&self, reader: &Box<dyn BufRead>, writer: &Box<dyn Write>) -> Vec<u8> {
        println!("Assembling");
        let lines: Vec<String> = reader
            .lines()
            .into_iter()
            .map(|line| line.unwrap())
            .collect();
        assembler::assemble(&lines)
    }

    pub fn disassemble(&self, reader: &Box<dyn BufRead>, writer: &Box<dyn Write>) -> Vec<String> {
        println!("Disassemble");
        ["abc".to_string()].to_vec()
    }
}
