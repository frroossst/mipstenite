use crate::registers::{Register, register_to_addr};

pub enum Bytecode {
    // VM Specific
    // =======================
    ALLOC,
    DEBUG,
    EXIT,
    HALT,
    STDIN,
    STDOUT,
    STRACE,
    PUSH(Value),
    POP,
    // gets and pushes value to stack
    GETP(Value),
    // sets and pops value to register
    SETO(Value),

    // Register Specific
    // =======================
    SET(Value),

    // Arithmetic Specifc
    // =======================
    ADD,


}

pub enum Value {
    Register(u32),
    Immediate(u32),
}

pub enum AsmInstruction {
    LI(String, u32),
    ADD(String, String, String),
}

impl AsmInstruction {

    pub fn to_bytecode(&self) -> Vec<Bytecode> {
        match *self {
            AsmInstruction::LI(reg, imm) => {
                let reg_name = register_to_addr(reg).expect("invalid register name: {reg}");
                Self::convert_li(reg_name, imm)
            },
            AsmInstruction::ADD(src, op1, op2) => {
                let reg_name = register_to_addr(src).expect("invalid register name: {src}");
                let op1_name = register_to_addr(op1).expect("invalid register name: {op1}");
                let op2_name = register_to_addr(op2).expect("invalid register name: {op2}");
                Self::convert_add(reg_name, op1_name, op2_name)
            },
            _ => { unimplemented!() }
        }
    }

    fn convert_li(reg: u32, imm: u32) -> Vec<Bytecode> {
        vec![
            Bytecode::PUSH(Value::Immediate(imm)),
            Bytecode::SET(Value::Register(reg)),
        ]
    }

    fn convert_add(reg: u32, op1: u32, op2: u32) -> Vec<Bytecode> {
        vec![
            Bytecode::GETP(Value::Register(op1)),
            Bytecode::GETP(Value::Register(op2)),
            Bytecode::ADD,
            Bytecode::SETO(Value::Register(reg)),
        ]
    }
    
}

