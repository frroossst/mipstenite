use serde::{Serialize, Deserialize};

use crate::registers::register_to_addr;

#[derive(Debug, Clone, PartialEq)]
#[derive(Serialize, Deserialize)]
pub enum Bytecode {
    // VM Specific
    // =======================
    ALLOC,
    DEBUG,
    // terminoator indicated no more insrucitons to execute
    TERMINATOR,
    // exit is a deliberate exit from the program
    EXIT,
    HALT,
    STDIN,
    STDOUT,
    STRACE,
    // dumps state of VM for debugging
    DUMP,
    PUSH(Value),
    POP,
    // gets and pushes value to stack
    GETP(Value),
    // sets and pops value to register
    SETO(Value),

    // System Specific
    // =======================
    SYSCALL,

    // Register Specific
    // =======================
    SET(Value),
    GET(Value),

    // Arithmetic Specifc
    // =======================
    ADD,

    // Branch Specific
    // =======================
    JUMP(u32),

}

#[derive(Debug, Clone, PartialEq)]
#[derive(Serialize, Deserialize)]
pub enum WhereTo {
    Label(String),
    Line(u32),
}

impl WhereTo {

    pub fn lift_label(&self) -> String {
        return match self {
            WhereTo::Label(label) => { label.clone() }
            _ => panic!("cannot lift label")
        }
    }

    pub fn lift_line(&self) -> u32 {
        return match self {
            WhereTo::Line(line) => { *line }
            _ => panic!("cannot lift line")
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[derive(Serialize, Deserialize)]
pub enum Value {
    Register(u32),
    Immediate(i16),
}

impl Value {

    pub fn lift_register(&self) -> u32 {
        return match self {
            Value::Register(reg) => { *reg }
            _ => panic!("cannot lift register value")
        }
    }

    pub fn lift_immediate(&self) -> u32 {
        return match self {
            // i16 to u32
            Value::Immediate(imm) => { *imm as u32 }
            _ => panic!("cannot life immediate value")
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[derive(Serialize, Deserialize)]
pub enum AsmInstruction {
    LI(String, i16),
    ADD(String, String, String),
    JUMP(WhereTo),
}

impl std::str::FromStr for AsmInstruction {
    type Err = String;

    // only instruction is provided the operands are defaulted
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "li" => Ok(AsmInstruction::LI(Default::default(), Default::default())),
            "add" => Ok(AsmInstruction::ADD(Default::default(), Default::default(), Default::default())),
            // "j" => Ok(AsmInstruction::JUMP(Default::default())),
            _ => Err(format!("invalid instruction: {s}"))
        }
    }
}

impl AsmInstruction {

    pub fn to_bytecode(&self) -> Vec<Bytecode> {
        match self {
            AsmInstruction::LI(reg, imm) => {
                let reg_name = register_to_addr(reg.clone()).expect("invalid register name: {reg}");
                translate::convert_li(reg_name, imm.clone())
            },
            AsmInstruction::ADD(src, op1, op2) => {
                let reg_name = register_to_addr(src.clone()).expect("invalid register name: {src}");
                let op1_name = register_to_addr(op1.clone()).expect("invalid register name: {op1}");
                let op2_name = register_to_addr(op2.clone()).expect("invalid register name: {op2}");
                translate::convert_add(reg_name, op1_name, op2_name)
            },
            AsmInstruction::JUMP(where_to) => {
                match where_to {
                    WhereTo::Label(label) => {
                        unimplemented!()
                    },
                    WhereTo::Line(line) => { 
                        translate::convert_jump(line.to_owned())
                    },
                }
            },
            _ => { unimplemented!() }
        }
    }

}

mod translate {
    use super::*;

    pub fn convert_li(reg: u32, imm: i16) -> Vec<Bytecode> {
        vec![
            Bytecode::PUSH(Value::Immediate(imm)),
            Bytecode::SETO(Value::Register(reg)),
        ]
    }

    pub fn convert_add(src: u32, op1: u32, op2: u32) -> Vec<Bytecode> {
        vec![
            Bytecode::GETP(Value::Register(op1)),
            Bytecode::GETP(Value::Register(op2)),
            Bytecode::ADD,
            Bytecode::SETO(Value::Register(src)),
        ]
    }

    pub fn convert_jump(line: u32) -> Vec<Bytecode> {
        vec![
            Bytecode::JUMP(line),
        ]
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_to_li() {
        let asm_in = AsmInstruction::LI("$t0".to_string(), 128);
        let asm_out = asm_in.to_bytecode();

        assert!(asm_out.len() == 2);
        assert!(matches!(asm_out[0], Bytecode::PUSH(Value::Immediate(128))));
        assert!(matches!(asm_out[1], Bytecode::SETO(Value::Register(8))));
    }

    #[test]
    fn test_to_add() {
        let asm_in = AsmInstruction::ADD("$t0".to_string(), "$t1".to_string(), "$t2".to_string());
        let asm_out = asm_in.to_bytecode();

        assert!(asm_out.len() == 4);
        assert!(matches!(asm_out[0], Bytecode::GETP(Value::Register(9))));
        assert!(matches!(asm_out[1], Bytecode::GETP(Value::Register(10))));
        assert!(matches!(asm_out[2], Bytecode::ADD));
        assert!(matches!(asm_out[3], Bytecode::SETO(Value::Register(8))));
    }

    #[test]
    fn test_to_jump() {
        let asm_in = AsmInstruction::JUMP(WhereTo::Line(0));
        let asm_out = asm_in.to_bytecode();

        assert!(asm_out.len() == 1);
        assert!(matches!(asm_out[0], Bytecode::JUMP(0)));

        let asm_in = AsmInstruction::JUMP(WhereTo::Line(123));
        let asm_out = asm_in.to_bytecode();
        assert!(asm_out.len() == 1);
        assert!(matches!(asm_out[0], Bytecode::JUMP(123)));
    }
}

