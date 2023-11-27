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
    // ! should be a vector of byte code IR
    program: Vec<u32>,
    stack: Stack,
}

