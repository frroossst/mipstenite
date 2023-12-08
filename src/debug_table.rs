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
    debug_map: Vec<(AsmInstruction, Vec<Bytecode>)>,
    label_map: HashMap<String, usize>,
}

impl CompileDebugInfo {

    pub fn new(asm_instructions: Vec<AsmInstruction>) -> CompileDebugInfo {

        let mut debug_map: Vec<(AsmInstruction, Vec<Bytecode>)> = Vec::new();

        for (x, i) in asm_instructions.into_iter().enumerate() {
            let member = (i.clone(), i.to_bytecode());
            debug_map.push(member);
        }

        CompileDebugInfo { 
            debug_map,
            label_map: HashMap::new(),
        }
    }

    // method to easily access the assembly instruction and bytecode for a given bytecode line number
    /*
    debug_map = vec![
        (AmsInstruction::Li, vec![Bytecode::Li, Bytecode::Reg, Bytecode::Imm]),
                                      0            1               2    
        (AsmINstruction::Li2, vec![Bytecode::Li2, Bytecode::Reg, Bytecode::Imm]),
                                      3            4               5      
    ]

    for the above the 0th element of debug_map will be returned for byte code line number 2
    as the second bytecode Bytecode::Imm resides within the tuple (AsmInstruction::Li, vec![Bytecode::Li, Bytecode::Reg, Bytecode::Imm])

    for the above the 1st element of debug_map will be returned for bytecode line number 3
    as the third bytecode Bytecode::Li2 resides within the tuple (AsmInstruction::Li2, vec![Bytecode::Li2, Bytecode::Reg, Bytecode::Imm])
    */
    pub fn get(&self, line_number: usize) -> (AsmInstruction, Vec<Bytecode>) {
        unimplemented!()
    }

}

/// struct for debug table
/// this stores the stack trace
/// current instruction etc. 
/// for debugging purposes
#[derive(Debug)]
pub struct RuntimeDebugInfo {
    compile_debug_info: CompileDebugInfo,
    stack_trace: Vec<usize>,
}

impl RuntimeDebugInfo {

    pub fn new() -> RuntimeDebugInfo {
        RuntimeDebugInfo {
            compile_debug_info: CompileDebugInfo::new(Vec::new()),
            stack_trace: Vec::new(),
        }
    }

    pub fn attach_compile_debug_info(&mut self, compile_debug_info: CompileDebugInfo) {
        self.compile_debug_info = compile_debug_info;
    }

    pub fn push_stack_trace(&mut self, line_number: usize) {
        self.stack_trace.push(line_number);
    }

    pub fn print_debug_info(&self) {
        let mut debug_stack_trace: Vec<(AsmInstruction, Vec<Bytecode>)> = Vec::new();

        // the self.stack_trace stores the bytecode that is being executed
        // we need to print the isntruction + the bytecode as part of our debug stack trace
        // for example:
        // when a bytecode of 4 is encountered and the strcut of CompileDebugInfo has the following:
        /*
            vec![
                (AmsInstruction::Li, vec![Bytecode::Li, Bytecode::Reg, Bytecode::Imm]),
                (AmsInstruction::Li, vec![Bytecode::Li, Bytecode::Reg, Bytecode::Imm]),
            ]
        */
        // the byte code to be fetched is first element of the second tuple in the vector

        // construct a new debug_stack_trace accordingly

        for i in self.stack_trace.clone() {

        }




     
    }
}
