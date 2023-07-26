use crate::mu_asm::{InstructionDescriptor, LabelDescriptor, MuAsm};
use std::io::Error;
use std::io::{BufRead, Write};

use super::FieldKind;
use super::REG_NAMES;

impl MuAsm {
    pub fn assemble(&mut self, reader: &mut Box<dyn BufRead>, writer: &mut Box<dyn Write>) {
        let addr_counter = 0;

        let lines: Vec<String> = reader
            .lines()
            .into_iter()
            .map(|line| line.unwrap().to_owned())
            .collect();

        // Build symbol table
        for line in &lines {
            let mut line = line.trim();

            // Remove comments
            line = match line.split_once(';') {
                Some(pair) => pair.0,
                None => line,
            };

            // Skip empty line
            if line.is_empty() {
                continue;
            }

            // Define symbols
            if line.starts_with("@") {
                self.symbol_map.insert(line.to_string(), addr_counter);
                continue;
            }

            let inst = self.parse_instruction(line);
            self.inst_table.push(inst);
        }

        for inst in &self.inst_table {
            let code: u64 = self.assemble_instruction(&inst).unwrap();
            writer.write_all(&code.to_le_bytes()).unwrap();
        }
    }

    fn encode_field(&self, field: &str) -> FieldKind {
        if field.starts_with("@") {
            return FieldKind::IMM(self.symbol_map.get(field).unwrap().to_owned());
        }

        if field.starts_with("0x") {
            let num = u32::from_str_radix(&field[2..], 16);
            if num.is_ok() {
                return FieldKind::IMM(num.unwrap() as u32);
            }
        }

        let num = u32::from_str_radix(&field, 10);
        if num.is_ok() {
            return FieldKind::IMM(num.unwrap() as u32);
        }
        let reg = 0x9;
        // let reg = REG_NAMES.iter().position(|name| name == &field).unwrap() as u8;
        FieldKind::REG(reg)
    }

    pub fn assemble_instruction(&self, inst: &InstructionDescriptor) -> Result<u64, Error> {
        let isa_entry = self.isa_map.get(inst.mnemonic.as_str()).unwrap();
        let mut code: u64 = 0x00;
        let fields: u8 = isa_entry.1;
        let opcode_group = isa_entry.2;
        let opcode = isa_entry.3;

        let mut rd: u8 = 0;
        let mut rs1: u8 = 0;
        let mut rs2: u8 = 0;
        let mut imm: u32 = 0;

        match self.encode_field(&inst.field1) {
            FieldKind::REG(reg) => {
                rd = reg;
            }
            FieldKind::IMM(num) => {
                imm = num;
            }
        }

        match self.encode_field(&inst.field2) {
            FieldKind::IMM(num) => {
                imm = num;
            }
            FieldKind::REG(num) => {
                rs1 = num;
            }
        }

        match self.encode_field(&inst.field3) {
            FieldKind::IMM(num) => {
                imm = num;
            }
            FieldKind::REG(num) => {
                rs2 = num;
            }
        }

        code += imm as u64;
        code += (rs2 as u64) << 32;
        code += (rs1 as u64) << 37;
        code += (rd as u64) << 42;
        code += (opcode as u64) << 47;
        code += (opcode_group as u64) << 57;
        code += (fields as u64) << 61;

        Ok(code)
    }
}
