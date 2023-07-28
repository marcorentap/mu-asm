use crate::mu_asm::{CodeDescriptor, InstructionDescriptor, MuAsm, INST_TABLE, REG_NAMES};
use std::{
    io::{BufReader, Read, Write},
    ops::IndexMut,
};

impl MuAsm {
    pub fn disassemble(&self, reader: &mut BufReader<Box<dyn Read>>, writer: &mut Box<dyn Write>) {
        loop {
            let mut buf: [u8; 8] = [0; 8];
            match reader.read_exact(&mut buf) {
                Ok(()) => {
                    let code = u64::from_le_bytes(buf);
                    let mut disasm = self.disassemble_code(&code);
                    disasm += "\n";
                    writer.write_all(disasm.as_bytes()).unwrap();
                }
                _ => {
                    return;
                }
            }
        }
    }

    fn disassemble_code(&self, code: &u64) -> String {
        let code_desc = CodeDescriptor::new(&code);
        let mut disasm = "".to_string();
        let mut mnemonic: String = "".to_string();

        for entry in INST_TABLE {
            if entry.1 == code_desc.fields as u8
                && entry.2 == code_desc.opcode_group as u8
                && entry.3 == code_desc.opcode as u8
            {
                mnemonic = entry.0.to_owned();
                disasm += &format!("{} ", &mnemonic);
            }
        }

        if mnemonic == "" {
            panic!(
                "Invalid fields, opcode group or opcode: {:016x} {:016x} {:016x}",
                code_desc.fields, code_desc.opcode_group, code_desc.opcode
            );
        }

        if code_desc.fields & 0b1000 != 0 {
            disasm += &format!("{} ", REG_NAMES.get(code_desc.rd as usize).unwrap());
        }

        if code_desc.fields & 0b0100 != 0 {
            // self.print_code_broken(code);
            // self.print_code_broken(&u64::MAX);
            // println!(
            // "{} : RS2 {} is {}",
            // mnemonic,
            // code_desc.rs2,
            // REG_NAMES.get(code_desc.rs2 as usize).unwrap()
            // );
            disasm += &format!("{} ", REG_NAMES.get(code_desc.rs1 as usize).unwrap());
        }

        if code_desc.fields & 0b0010 != 0 {
            disasm += &format!("{} ", REG_NAMES.get(code_desc.rs2 as usize).unwrap());
        }

        if code_desc.fields & 0b0001 != 0 {
            disasm += &format!("0x{:08x} ", code_desc.imm);
        }

        disasm
    }
}
