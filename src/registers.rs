use std::collections::HashMap;


/// $zero 
/// $v0-$v1
/// $a0-$a3
/// $t0-$t9
/// $s0-$s7
/// $ra
/// $sp
pub struct RegisterSet {
    registers: HashMap<String, Register>,
}

pub struct Register {
    name: String,
    value: u32,
}

impl Register {
    pub fn new(name: String) -> Self {
        Register { name, value: 0 }
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn get(&self) -> u32 {
        self.value
    }

    pub fn set(&mut self, value: u32) {
        self.value = value;
    }
}

