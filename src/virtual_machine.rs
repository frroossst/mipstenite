use crate::bytecode::Bytecode;

struct Stack {
    data: Vec<u32>,
}

impl Stack {
    pub fn new() -> Stack {
        Stack { data: Vec::new() }
    }

    pub fn push(&mut self, value: u32) {
        self.data.push(value);
    }

    pub fn pop(&mut self) -> Option<u32> {
        self.data.pop()
    }
}

pub struct VirtualMachine {
    registers: [u32; 32],
    memory: Vec<u8>,
    pc: u32,
    program: Vec<Bytecode>,
    stack: Stack,
}

impl VirtualMachine {
    
    pub fn new() -> VirtualMachine {
        VirtualMachine {
            registers: [0; 32],
            memory: vec![0; 1024],
            pc: 0,
            program: Vec::new(),
            stack: Stack::new(),
        }
    }

    pub fn reg_set(&mut self,reg: u32, value: u32) {
        if reg > 32 {
            panic!("Invalid register");
        }
        self.registers[reg as usize] = value;
    }

    pub fn reg_get(&self, reg: u32) -> u32 {
        if reg > 32 {
            panic!("Invalid register");
        }
        self.registers[reg as usize]
    }

    pub fn execute(&mut self) {
        let current_instruction = self.program[self.pc as usize];
        match current_instruction {
            Bytecode::PUSH(reg) => {
                let reg_value = self.reg_get(reg);
                self.stack.push(reg_value);
            },
            _ => { unimplemented!("Instruction not implemented") }
        }
    }

}

