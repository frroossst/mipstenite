use std::collections::HashMap;

use crate::bytecode::{Bytecode, AsmInstruction};

/// usize maps to three things: 
/// - assembly instruction
/// - current label
/// - line number
/// So, the error message can look like:
/// [ERROR] 13: li $t9, 123 in foo
///     VM failed to set register
#[derive(Debug)]
pub struct CompileDebugInfo {
    instructions: Vec<String>,
    asm_instructions: Vec<AsmInstruction>,
    byc_instructions: Vec<Bytecode>,
    label_map: HashMap<String, usize>,
}

impl CompileDebugInfo {

    pub fn new() -> CompileDebugInfo {
        CompileDebugInfo { 
            instructions: Vec::new(),
            asm_instructions: Vec::new(), 
            byc_instructions: Vec::new(),
            label_map: HashMap::new(),
        }
    }

}

/// struct for debug table
/// this stores the stack trace
/// current instruction etc. 
/// for debugging purposes
#[derive(Debug)]
pub struct RuntimeDebugInfo {
    ins_stack_trace: Vec<String>,
    byc_stack_trace: Vec<Bytecode>,
    current_line_number: u32,
}

impl RuntimeDebugInfo {

    pub fn new() -> RuntimeDebugInfo {
        RuntimeDebugInfo {
            ins_stack_trace: Vec::new(),
            byc_stack_trace: Vec::new(),
            current_line_number: 0,
        }
    }

    pub fn push_bytecode(&mut self, byc: Bytecode) {
        self.byc_stack_trace.push(byc)
    }

    pub fn print_debug_info(&self) {
        unimplemented!("print_debug_info")
    }

}
