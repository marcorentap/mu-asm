mod assembler;
mod disassembler;
mod parser;

use std::collections::HashMap;

enum FieldKind {
    IMM(u32),
    REG(u8),
}

const REG_NAMES: &'static [&'static str] = &[
    "R0", "R1", "R2", "R3", "R4", "R5", "R6", "R7", "R8", "R9", "R10", "R11", "R12", "R13", "R14",
    "R15", "R16", "R17", "R18", "R19", "R20", "R21", "R22", "R23", "R24", "R25", "R26", "R27",
    "R28", "R29", "R30", "R31", "COND", "RIP", "RSP",
];

// mnemonic, fields, group, opcode
const INST_TABLE: &'static [(&str, u8, u8, u8)] = &[
    // Sets with IMM
    ("SET", 0x01, 0x00, 0x01),
    ("SETH", 0x01, 0x00, 0x02),
    // Loads with RD, RS1
    ("LD1", 0x0c, 0x01, 0x01),
    ("LD2", 0x0c, 0x01, 0x02),
    ("LD4", 0x0c, 0x01, 0x03),
    ("LD8", 0x0c, 0x01, 0x04),
    // Loads with RD, IMM
    ("LD1", 0x09, 0x02, 0x01),
    ("LD2", 0x09, 0x02, 0x02),
    ("LD4", 0x09, 0x02, 0x03),
    ("LD8", 0x09, 0x02, 0x04),
    // Stores with RD, RS1
    ("ST1", 0x0c, 0x03, 0x01),
    ("ST2", 0x0c, 0x03, 0x02),
    ("ST4", 0x0c, 0x03, 0x03),
    ("ST8", 0x0c, 0x03, 0x04),
    // Stores with RD, IMM
    ("ST1", 0x09, 0x04, 0x01),
    ("ST2", 0x09, 0x04, 0x02),
    ("ST4", 0x09, 0x04, 0x03),
    ("ST8", 0x09, 0x04, 0x04),
    // Arithmetic with RD, RS1, RS2
    ("ADD", 0x0e, 0x05, 0x01),
    ("SUB", 0x0e, 0x05, 0x02),
    ("MUL", 0x0e, 0x05, 0x03),
    ("DIV", 0x0e, 0x05, 0x04),
    // Arithmetic with RD, RS1, IMM
    ("ADD", 0x0d, 0x06, 0x01),
    ("SUB", 0x0d, 0x06, 0x02),
    ("MUL", 0x0d, 0x06, 0x03),
    ("DIV", 0x0d, 0x06, 0x04),
    // Comparison with RS1, RS2
    ("GEQ", 0x06, 0x07, 0x01),
    ("GRE", 0x06, 0x07, 0x02),
    ("EQU", 0x06, 0x07, 0x03),
    ("LEQ", 0x06, 0x07, 0x04),
    ("LES", 0x06, 0x07, 0x05),
    // Comparison with RS1, IMM
    ("GEQ", 0x05, 0x08, 0x01),
    ("GRE", 0x05, 0x08, 0x02),
    ("EQU", 0x05, 0x08, 0x03),
    ("LEQ", 0x05, 0x08, 0x04),
    ("LES", 0x05, 0x08, 0x05),
    // Branching with RS1
    ("J", 0x04, 0x09, 0x01),
    ("CJ", 0x04, 0x09, 0x02),
    // Branching with IMM
    ("J", 0x01, 0x10, 0x01),
    ("CJ", 0x01, 0x10, 0x02),
    // Function calls
    ("CALL", 0x00, 0x11, 0x01),
    ("RET", 0x00, 0x11, 0x02),
];

#[derive(Debug)]
pub struct LabelDescriptor {
    pub address: u32,
    pub name: String,
}

#[derive(Debug)]
pub struct InstructionDescriptor {
    pub text: String,
    pub address: u32,
    pub mnemonic: String,
    pub field1: String,
    pub field2: String,
    pub field3: String,
    pub imm: String,
}

impl InstructionDescriptor {
    pub fn new() -> Self {
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

pub struct MuAsm {
    inst_table: Vec<InstructionDescriptor>,
    symbol_map: HashMap<String, u32>,
    isa_map: HashMap<&'static str, &'static (&'static str, u8, u8, u8)>,
}

impl MuAsm {
    pub fn new() -> Self {
        let mut muasm = MuAsm {
            inst_table: Vec::new(),
            symbol_map: HashMap::new(),
            isa_map: HashMap::new(),
        };

        for entry in INST_TABLE {
            muasm.isa_map.insert(entry.0, entry);
        }

        muasm
    }
}
