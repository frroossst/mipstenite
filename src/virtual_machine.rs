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
        if reg > 31 {
            panic!("Invalid register");
        }
        self.registers[reg as usize] = value;
    }

    pub fn reg_get(&self, reg: u32) -> u32 {
        if reg > 31 {
            panic!("Invalid register");
        }
        self.registers[reg as usize]
    }

}

