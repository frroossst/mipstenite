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

    #[cfg(debug_assertions)]
    fn peek(&self) -> Option<&u32> {
        self.data.last()
    }
}

pub struct VirtualMachine {
    registers: [u32; 32],
    memory: Vec<u8>,
    pc: usize,
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

    pub fn add_program(&mut self, program: Vec<Bytecode>) {
        self.program = program;
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

    /// only executes the next instruction
    pub fn execute(&mut self) {
        let current_instruction = &self.program[self.pc];
        match current_instruction {
            Bytecode::PUSH(val) => {
                self.stack.push(val.lift_immediate());
            },
            // gets value from register and pushes it to the stack
            Bytecode::GETP(reg) => {
                let value = self.reg_get(reg.lift_register());
                self.stack.push(value);
            },
            // pops value from stack and sets it to register
            Bytecode::SETO(reg) => {
                let value = self.stack.pop().expect("Stack underflow");
                self.reg_set(reg.lift_register(), value);
            },
            // adds two values from the stack and pushes the result
            Bytecode::ADD => {
                let op1 = self.stack.pop().expect("Stack underflow");
                let op2 = self.stack.pop().expect("Stack underflow");
                self.stack.push(op1 + op2);
            },
            _ => { unimplemented!("Instruction not implemented") }
        }
        self.pc += 1;
    }

}

#[cfg(test)]
mod tests {

    use crate::{registers::register_to_addr, bytecode::Value};

    use super::*;

    #[test]
    fn test_vm_add() {
        let mut vm = VirtualMachine::new();

        let program = vec![
            // li $t1, 1
            Bytecode::PUSH(Value::Immediate(1)),
            Bytecode::SETO(Value::Register(register_to_addr("$t1".to_string()).unwrap())),
            // li $t2, 1
            Bytecode::PUSH(Value::Immediate(1)),
            Bytecode::SETO(Value::Register(register_to_addr("$t2".to_string()).unwrap())),
            // add $t3, $t1, $t2
            Bytecode::GETP(Value::Register(register_to_addr("$t1".to_string()).unwrap())),
            Bytecode::GETP(Value::Register(register_to_addr("$t2".to_string()).unwrap())),
            Bytecode::ADD,
            Bytecode::SETO(Value::Register(register_to_addr("$t3".to_string()).unwrap())),
        ];
        vm.add_program(program);

        // PUSH 1
        vm.execute();
        assert!(vm.stack.peek() == Some(&1));

        // SET $t1
        vm.execute();
        assert!(matches!(vm.reg_get(register_to_addr("$t1".to_string()).unwrap()), 1));
        assert!(matches!(vm.reg_get(register_to_addr("$t2".to_string()).unwrap()), 0));
        assert!(matches!(vm.reg_get(register_to_addr("$t3".to_string()).unwrap()), 0));

        // PUSH 1
        vm.execute();
        assert!(vm.stack.peek() == Some(&1));

        // SET $t2
        vm.execute();
        assert!(matches!(vm.reg_get(register_to_addr("$t1".to_string()).unwrap()), 1));
        assert!(matches!(vm.reg_get(register_to_addr("$t2".to_string()).unwrap()), 1));
        assert!(matches!(vm.reg_get(register_to_addr("$t3".to_string()).unwrap()), 0));

        // GETP $t1
        vm.execute();
        assert!(vm.stack.peek() == Some(&1));

        // GETP $t2
        vm.execute();
        assert!(vm.stack.peek() == Some(&1));

        // ADD
        vm.execute();
        assert!(vm.stack.peek() == Some(&2));

        // SET $t3
        vm.execute();
        assert!(matches!(vm.reg_get(register_to_addr("$t1".to_string()).unwrap()), 1));
        assert!(matches!(vm.reg_get(register_to_addr("$t2".to_string()).unwrap()), 1));
        assert!(matches!(vm.reg_get(register_to_addr("$t3".to_string()).unwrap()), 2));

    }
}
