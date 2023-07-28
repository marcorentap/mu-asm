mod assembler;
mod disassembler;
mod parser;

use std::collections::HashMap;

enum FieldKind {
    RD,
    RS1,
    RS2,
    IMM,
    NONE,
}

enum FieldValueKind {
    IMM(u64),
    REG(u8),
    EMPTY,
}

const REG_NAMES: &'static [&'static str] = &[
    "R0", "R1", "R2", "R3", "R4", "R5", "R6", "R7", "R8", "R9", "R10", "R11", "R12", "R13", "R14",
    "R15", "R16", "R17", "R18", "R19", "R20", "R21", "R22", "R23", "R24", "R25", "R26", "R27",
    "R28", "R29", "R30", "R31",
];

// mnemonic, fields, group, opcode
const INST_TABLE: &'static [(&str, u8, u8, u8)] = &[
    // Sets with RD, IMM
    ("SET", 0x09, 0x00, 0x01),
    ("SETH", 0x09, 0x00, 0x02),
    // Sets with RD, RS1
    ("MOV", 0x0c, 0x00, 0x03),
    // Loads with RD, RS1
    ("LD1", 0x0c, 0x01, 0x01),
    ("LD2", 0x0c, 0x01, 0x02),
    ("LD4", 0x0c, 0x01, 0x03),
    ("LD8", 0x0c, 0x01, 0x04),
    // Stores with RD, RS1
    ("ST1", 0x0c, 0x03, 0x01),
    ("ST2", 0x0c, 0x03, 0x02),
    ("ST4", 0x0c, 0x03, 0x03),
    ("ST8", 0x0c, 0x03, 0x04),
    // Arithmetic with RD, RS1, RS2
    ("ADD", 0x0e, 0x05, 0x01),
    ("SUB", 0x0e, 0x05, 0x02),
    ("MUL", 0x0e, 0x05, 0x03),
    ("DIV", 0x0e, 0x05, 0x04),
    // Comparison with RS1, RS2
    ("GEQ", 0x06, 0x07, 0x01),
    ("GRE", 0x06, 0x07, 0x02),
    ("EQU", 0x06, 0x07, 0x03),
    ("LEQ", 0x06, 0x07, 0x04),
    ("LES", 0x06, 0x07, 0x05),
    // Branching with RS1
    ("J", 0x04, 0x09, 0x01),
    ("CJ", 0x04, 0x09, 0x02),
    // Function calls
    ("CALL", 0x04, 0x11, 0x01),
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
        }
    }
}

#[derive(Debug)]
pub struct CodeDescriptor {
    code: u64,
    imm: u64,
    rs2: u64,
    rs1: u64,
    rd: u64,
    opcode: u64,
    opcode_group: u64,
    fields: u64,
}

impl CodeDescriptor {
    pub fn new(code: &u64) -> Self {
        CodeDescriptor {
            code: code.to_owned(),
            imm: code & 0xffffffff,
            rs2: (code & (0b11111 << 32)) >> 32,
            rs1: (code & (0b11111 << 37)) >> 37,
            rd: (code & (0b11111 << 42)) >> 42,
            opcode: (code & (0xff << 47)) >> 47,
            opcode_group: (code & (0b11111 << 55)) >> 55,
            fields: (code & (0b1111 << 60)) >> 60,
        }
    }

    pub fn from_inst(inst: &InstructionDescriptor, symbol_map: &HashMap<String, u64>) -> Self {
        let mut code: u64 = 0;
        let mut inst_entry: Option<&(&str, u8, u8, u8)> = None;

        for entry in INST_TABLE {
            if inst.mnemonic == entry.0 {
                inst_entry = Some(entry);
            }
        }
        match inst_entry {
            Some(entry) => {
                code += (entry.3 as u64 & 0xff) << 47;
                code += (entry.2 as u64 & 0b11111) << 55;
                code += (entry.1 as u64 & 0b1111) << 60;
            }
            None => panic!("Unknown instruction {}", inst.mnemonic),
        };

        code += inst.encode_fields(symbol_map);
        CodeDescriptor::new(&code)
    }

    pub fn construct(
        imm: u64,
        rs2: u64,
        rs1: u64,
        rd: u64,
        opcode: u64,
        opcode_group: u64,
        fields: u64,
    ) -> Self {
        let mut code = 0;
        code += imm & 0xffffffff;
        code += (rs2 & 0b11111) << 32;
        code += (rs1 & 0b11111) << 37;
        code += (rd & 0b11111) << 42;
        code += (opcode & 0xff) << 47;
        code += (opcode_group & 0b11111) << 55;
        code += (fields & 0b1111) << 60;

        CodeDescriptor::new(&code)
    }
}

pub struct MuAsm {
    inst_table: Vec<InstructionDescriptor>,
    symbol_map: HashMap<String, u64>,
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
