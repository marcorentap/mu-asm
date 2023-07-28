use crate::mu_asm::{CodeDescriptor, InstructionDescriptor, LabelDescriptor, MuAsm};
use std::io::{BufRead, Error};
use std::io::{BufReader, Read, Write};
use std::ops::IndexMut;

use super::FieldValueKind;
use super::REG_NAMES;

enum FieldKind {
    RD,
    RS1,
    RS2,
    IMM,
    NONE,
}

impl MuAsm {
    pub fn assemble(&mut self, reader: &mut BufReader<Box<dyn Read>>, writer: &mut Box<dyn Write>) {
        let reader: Box<dyn BufRead> = Box::new(reader);
        let mut addr_counter: u64 = 0;

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
                Some(pair) => pair.0.trim(),
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
            addr_counter += 0x08;
        }

        for inst in &self.inst_table {
            let code: u64 = self.assemble_instruction(&inst).unwrap();
            writer.write_all(&code.to_le_bytes()).unwrap();
        }
    }

    pub fn assemble_instruction(&self, inst: &InstructionDescriptor) -> Result<u64, Error> {
        let isa_entry = self.isa_map.get(inst.mnemonic.as_str()).unwrap();

        let code = CodeDescriptor::from_inst(&inst, &self.symbol_map);
        Ok(code.code)
    }
}
