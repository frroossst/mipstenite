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

    PUSH(u32),
    POP(u32),

    HALT,
    DEBUG,
}

pub enum AsmInstruction {
    LI(String, u32),
    ADD(u32, u32, u32),
}

impl AsmInstruction {

    pub fn to_bytecode(&self) -> Vec<Bytecode> {
        match self {
            AsmInstruction::LI(reg, val) => {
                vec![
                    Bytecode::PUSH(*val),
                    Bytecode::STORE(register_to_addr(reg.clone()).unwrap()),
                ]
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
    
}

