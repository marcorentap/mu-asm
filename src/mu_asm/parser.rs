use crate::mu_asm::CodeDescriptor;
use crate::mu_asm::FieldValueKind;
use crate::mu_asm::InstructionDescriptor;
use crate::mu_asm::MuAsm;
use std::collections::HashMap;
use std::mem;

use super::INST_TABLE;
use super::REG_NAMES;

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

    pub fn parse_code(&self, code: &u64) -> InstructionDescriptor {
        let mut inst = InstructionDescriptor::new();

        inst
    }

    pub fn print_code_broken(&self, code: &u64) {
        let code = CodeDescriptor::new(&code);
        println!("{:064b}", code.imm);
        println!("{:064b}", code.rs2 << 32);
        println!("{:064b}", code.rs1 << 37);
        println!("{:064b}", code.rd << 42);
        println!("{:064b}", code.opcode << 47);
        println!("{:064b}", code.opcode_group << 55);
        println!("{:064b}", code.fields << 60);
    }
}

impl InstructionDescriptor {
    fn parse_field(&self, field: &str, symbol_map: &HashMap<String, u64>) -> FieldValueKind {
        if field == "" {
            return FieldValueKind::EMPTY;
        }
        if field.starts_with("@") {
            match symbol_map.get(field) {
                Some(addr) => return FieldValueKind::IMM(addr.to_owned()),
                None => panic!("Invalid symbol {}", field),
            }
        }

        if field.starts_with("0x") {
            let num = u32::from_str_radix(&field[2..], 16);
            if num.is_ok() {
                return FieldValueKind::IMM(num.unwrap() as u64);
            }
        }

        let num = u32::from_str_radix(&field, 10);
        if num.is_ok() {
            return FieldValueKind::IMM(num.unwrap() as u64);
        }

        let reg = match REG_NAMES.iter().position(|name| name == &field) {
            Some(num) => num,
            None => {
                panic!("Invalid register name")
            }
        };
        FieldValueKind::REG(reg as u8)
    }

    // pub fn parse_field
    fn encode_field(
        &self,
        expected_field_value_kind: &FieldValueKind,
        field: &str,
        symbol_map: &HashMap<String, u64>,
    ) -> u64 {
        match self.parse_field(field, symbol_map) {
            FieldValueKind::REG(reg) => {
                if mem::discriminant(&FieldValueKind::REG(reg))
                    == mem::discriminant(&expected_field_value_kind)
                {
                    return reg as u64;
                } else {
                    panic!("Unexpected register {}", field)
                }
            }
            FieldValueKind::IMM(num) => {
                if mem::discriminant(&FieldValueKind::IMM(num))
                    == mem::discriminant(&expected_field_value_kind)
                {
                    return num as u64;
                } else {
                    panic!("Unexpected immediate value {}", field)
                }
            }
            FieldValueKind::EMPTY => {
                if mem::discriminant(&FieldValueKind::EMPTY)
                    == mem::discriminant(&expected_field_value_kind)
                {
                    return 0 as u64;
                } else {
                    panic!("Unexpected empty field {}", field)
                }
            }
        }
    }

    pub fn encode_fields(&self, symbol_map: &HashMap<String, u64>) -> u64 {
        let mut inst_entry: Option<&(&str, u8, u8, u8)> = None;
        let mut code: u64 = 0;
        let mut rd: u64 = 0;
        let mut rs1: u64 = 0;
        let mut rs2: u64 = 0;
        let mut imm: u64 = 0;
        for entry in INST_TABLE {
            if self.mnemonic == entry.0 {
                inst_entry = Some(entry);
            }
        }

        let fields = match inst_entry {
            Some(entry) => entry.1,
            None => panic!("Unknown instruction {}", self.mnemonic),
        };

        match fields {
            // RD, IMM
            0x09 => {
                rd = self.encode_field(&FieldValueKind::REG(0), &self.field1, symbol_map);
                imm = self.encode_field(&FieldValueKind::IMM(0), &self.field2, symbol_map);
                _ = self.encode_field(&FieldValueKind::EMPTY, &self.field3, symbol_map);
            }
            // RD, RS1
            0x0c => {
                rd = self.encode_field(&FieldValueKind::REG(0), &self.field1, symbol_map);
                rs1 = self.encode_field(&FieldValueKind::REG(0), &self.field2, symbol_map);
                _ = self.encode_field(&FieldValueKind::EMPTY, &self.field3, symbol_map);
            }
            // RD, RS1, RS2
            0xe => {
                rd = self.encode_field(&FieldValueKind::REG(0), &self.field1, symbol_map);
                rs1 = self.encode_field(&FieldValueKind::REG(0), &self.field2, symbol_map);
                rs2 = self.encode_field(&FieldValueKind::REG(0), &self.field3, symbol_map);
            }
            // RD, RS1, IMM
            0x0d => {
                rd = self.encode_field(&FieldValueKind::REG(0), &self.field1, symbol_map);
                rs1 = self.encode_field(&FieldValueKind::REG(0), &self.field2, symbol_map);
                imm = self.encode_field(&FieldValueKind::IMM(0), &self.field3, symbol_map);
            }
            // RS1, RS2
            0x06 => {
                rs1 = self.encode_field(&FieldValueKind::REG(0), &self.field1, symbol_map);
                rs2 = self.encode_field(&FieldValueKind::REG(0), &self.field2, symbol_map);
                _ = self.encode_field(&FieldValueKind::EMPTY, &self.field3, symbol_map);
            }
            // RS1, IMM
            0x05 => {
                rs1 = self.encode_field(&FieldValueKind::REG(0), &self.field1, symbol_map);
                imm = self.encode_field(&FieldValueKind::IMM(0), &self.field2, symbol_map);
                _ = self.encode_field(&FieldValueKind::EMPTY, &self.field3, symbol_map);
            }
            // RS1
            0x04 => {
                rs1 = self.encode_field(&FieldValueKind::REG(0), &self.field1, symbol_map);
                _ = self.encode_field(&FieldValueKind::EMPTY, &self.field2, symbol_map);
                _ = self.encode_field(&FieldValueKind::EMPTY, &self.field3, symbol_map);
            }
            // IMM
            0x01 => {
                imm = self.encode_field(&FieldValueKind::IMM(0), &self.field1, symbol_map);
                _ = self.encode_field(&FieldValueKind::EMPTY, &self.field2, symbol_map);
                _ = self.encode_field(&FieldValueKind::EMPTY, &self.field3, symbol_map);
            }
            // EMPTY
            0x00 => {
                _ = self.encode_field(&FieldValueKind::EMPTY, &self.field1, symbol_map);
                _ = self.encode_field(&FieldValueKind::EMPTY, &self.field2, symbol_map);
                _ = self.encode_field(&FieldValueKind::EMPTY, &self.field3, symbol_map);
            }
            _ => {
                panic!("Unknown fields")
            }
        }

        code += imm & 0xffffffff;
        code += (rs2 & 0b11111) << 32;
        code += (rs1 & 0b11111) << 37;
        code += (rd & 0b11111) << 42;
        code
    }
}
