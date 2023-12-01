use crate::registers::{Register, register_to_addr};

pub enum Bytecode {
    // loads register values
    LOAD(u32),
    // stores register values
    STORE(u32),
    // sets value to memory
    MEMSET(u32),
    // gets value from memory
    MEMGET(u32),

    ADD,
    SUB,

    JUMP(u32),

    PUSH(Value),
    POP(Value),

    HALT,
    DEBUG,
}

pub enum Value {
    Register(Register),
    Immediate(u32),
}

pub enum AsmInstruction {
    LI(String, u32),
    ADD(u32, Value, Value),
}

impl AsmInstruction {

    pub fn to_bytecode(&self) -> Vec<Bytecode> {
        match self {
            AsmInstruction::LI(reg, val) => {
                return Self::convert_li(*self);
            }
            AsmInstruction::ADD(rd, rs, rt) => {
                vec![
                    Bytecode::PUSH(*rt),
                    Bytecode::PUSH(*rs),
                    Bytecode::ADD,
                    Bytecode::POP(*rd),
                ]
            }
            _ => { unimplemented!() }
        }
    }

    fn convert_li() -> Vec<Bytecode> {

    }
    
}

