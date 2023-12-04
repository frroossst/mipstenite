use crate::bytecode::{Bytecode, AsmInstruction};

/// struct for debug table
/// this stores the stack trace
/// current instruction etc. 
/// for debugging purposes
pub struct DebugInfo {
    ins_stack_trace: Vec<String>,
    byc_stack_trace: Vec<Bytecode>,
    line_number: u32,
    current_label: Option<String>,

    instructions: Vec<String>,
    asm_instructions: Vec<AsmInstruction>,
    byc_instructions: Vec<Bytecode>,
}

impl DebugInfo {

    pub fn new() -> DebugInfo {
        DebugInfo {
            ins_stack_trace: Vec::new(),
            byc_stack_trace: Vec::new(),
            line_number: 0,
            current_label: None,
            
            instructions: Vec::new(),
            asm_instructions: Vec::new(),
            byc_instructions: Vec::new(),
        }
    }

    pub fn print_debug_info(&self) {
        unimplemented!("print_debug_info")
    }

}
