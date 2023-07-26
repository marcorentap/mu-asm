use crate::mu_asm::MuAsm;
use std::io::{BufRead, Write};

impl MuAsm {
    pub fn disassemble(&self, reader: &Box<dyn BufRead>, writer: &Box<dyn Write>) {
        println!("Disassemble");
    }
}
