use std::{io::Write, time::{SystemTime, UNIX_EPOCH}};

use serde::{Serialize, Deserialize};
use crate::{bytecode::Bytecode, registers::PrettyFmtRegister, debug_table::{RuntimeDebugInfo, CompileDebugInfo}};

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
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

    #[allow(dead_code)]
    #[cfg(debug_assertions)]
    fn peek(&self) -> Option<&u32> {
        self.data.last()
    }
}

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub enum ConsoleLocation {
    Socket,
    Terminal,
}

impl Default for ConsoleLocation {
    fn default() -> Self {
        ConsoleLocation::Terminal
    }
}

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct Console {
    console: Vec<u8>,
    location: ConsoleLocation,
}

impl Console {

    pub fn new() -> Console {
        Console {
            console: Vec::new(),
            location: Default::default(),
        }
    }

    pub fn set_location(&mut self, location: ConsoleLocation) {
        self.location = location;
    }

    pub fn read_from_console(&mut self) {
        match self.location {
            ConsoleLocation::Socket => {
                unimplemented!()
            },
            ConsoleLocation::Terminal => {
                let mut input = String::new();
                std::io::stdin().read_line(&mut input).expect("Failed to read from stdin");
                self.console = input.as_bytes().to_vec();
            },
        }
    }

    pub fn write_to_console(&self) {
        match self.location {
            ConsoleLocation::Socket => {
                unimplemented!()
            },
            ConsoleLocation::Terminal => {
                println!("{}", String::from_utf8(self.console.clone()).unwrap());
            },
        }
    }

}

// #[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct VirtualMachine {
    registers: [u32; 32],
    memory: Vec<u8>,
    pc: usize,
    program: Vec<Bytecode>,
    stack: Stack,
    console: Console,
    pub runtime_dbg: RuntimeDebugInfo,
}

impl VirtualMachine {
    
    pub fn new() -> VirtualMachine {
        VirtualMachine {
            registers: [0; 32],
            memory: Vec::new(),
            pc: 0,
            program: Vec::new(),
            stack: Stack::new(),
            console: Console::new(),
            runtime_dbg: RuntimeDebugInfo::new(),
        }
    }

    pub fn init(&mut self, mem: Vec<u8>, program: Vec<Bytecode>) {
        self.memory = mem;
        self.program = program;
    }

    pub fn set_program(&mut self, program: Vec<Bytecode>) {
        self.program = program;
    }

    pub fn setup_debug(&mut self, debug: CompileDebugInfo) {
        self.runtime_dbg = RuntimeDebugInfo::new();
        self.runtime_dbg.attach_compile_debug_info(debug);
    }

    pub fn load(&mut self, filename: &str) -> VirtualMachine {
        let file = std::fs::File::open(filename).unwrap();
        let vm: VirtualMachine = bincode::deserialize_from(file).unwrap();
        vm
    }

    pub fn dump(&self) {
        let uid = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs().to_string() + ".bin";
		match bincode::serialize(&self) {
            Ok(s) => {
                let mut file = std::fs::File::create(uid).unwrap();
                file.write_all(s.as_slice()).unwrap();
            },
            Err(_) => {
                eprintln!("Error serializing VM");
            }
        }
    }

    pub fn reg_set(&mut self,reg: u32, value: u32) {
        if reg > 32 {
            panic!("Invalid register");
        } else if reg == 0 {
            return;
        }
        self.registers[reg as usize] = value;
    }

    pub fn reg_get(&self, reg: u32) -> u32 {
        if reg > 32 {
            panic!("Invalid register");
        }
        self.registers[reg as usize]
    }

    pub fn read_from_console(&mut self) {
        return self.console.read_from_console();
    }

    pub fn write_to_console(&self) {
        return self.console.write_to_console();
    }

    // only executes the next instruction
    pub fn execute(&mut self) -> Result<(), ()> {
        if self.pc >= self.program.len() {
            panic!("Program counter out of bounds");
        }

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
            Bytecode::TERMINATOR => {
                eprintln!("Reached end of program without exit instruction");
                return Err(());
            },
            Bytecode::JUMP(where_to) => {
                self.runtime_dbg.push_stack_trace(self.pc);
                self.pc = where_to.to_owned() as usize;
                return Ok(());
            },
            Bytecode::DUMP => {
                self.dump();
            },
            _ => { unimplemented!("Instruction not implemented: {:?}", current_instruction) }
        }
        self.runtime_dbg.push_stack_trace(self.pc);
        self.pc += 1;
        return Ok(());
    }

}

impl std::fmt::Debug for VirtualMachine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Manually implement the Debug trait for VirtualMachine
        f.debug_struct("VirtualMachine")
            .field("registers", &PrettyFmtRegister(&self.registers))
            .field("memory", &self.memory)
            .field("pc", &self.pc)
            .field("program", &self.program)
            .field("stack", &self.stack)
            .field("console", &self.console)
            .field("runtime_dbg", &self.runtime_dbg.print_debug_info())
            .finish()
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

            Bytecode::TERMINATOR,
        ];
        vm.set_program(program);

        // PUSH 1
        vm.execute().unwrap();
        assert!(vm.stack.peek() == Some(&1));

        // SET $t1
        vm.execute().unwrap();
        assert!(matches!(vm.reg_get(register_to_addr("$t1".to_string()).unwrap()), 1));
        assert!(matches!(vm.reg_get(register_to_addr("$t2".to_string()).unwrap()), 0));
        assert!(matches!(vm.reg_get(register_to_addr("$t3".to_string()).unwrap()), 0));

        // PUSH 1
        vm.execute().unwrap();
        assert!(vm.stack.peek() == Some(&1));

        // SET $t2
        vm.execute().unwrap();
        assert!(matches!(vm.reg_get(register_to_addr("$t1".to_string()).unwrap()), 1));
        assert!(matches!(vm.reg_get(register_to_addr("$t2".to_string()).unwrap()), 1));
        assert!(matches!(vm.reg_get(register_to_addr("$t3".to_string()).unwrap()), 0));

        // GETP $t1
        vm.execute().unwrap();
        assert!(vm.stack.peek() == Some(&1));

        // GETP $t2
        vm.execute().unwrap();
        assert!(vm.stack.peek() == Some(&1));

        // ADD
        vm.execute().unwrap();
        assert!(vm.stack.peek() == Some(&2));

        // SET $t3
        vm.execute().unwrap();
        assert!(matches!(vm.reg_get(register_to_addr("$t1".to_string()).unwrap()), 1));
        assert!(matches!(vm.reg_get(register_to_addr("$t2".to_string()).unwrap()), 1));
        assert!(matches!(vm.reg_get(register_to_addr("$t3".to_string()).unwrap()), 2));
        assert!(vm.stack.peek() == None);

    }
}
