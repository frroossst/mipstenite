use std::collections::{HashMap, BTreeMap};
use std::ops::Range;

use crate::bytecode::{Bytecode, AsmInstruction};


#[derive(Debug, Eq, PartialEq)]
struct LineRange {
    range: Range<usize>,
}

impl Ord for LineRange {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.range.start.cmp(&other.range.start)
    }
}

impl PartialOrd for LineRange {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

/// usize maps to three things: 
/// - assembly instruction
/// - current label
/// - line number
/// So, the error message can look like:
/// [ERROR] 13: li $t9, 123 in foo
///     VM failed to set register
#[derive(Debug)]
pub struct CompileDebugInfo {
    debug_map: BTreeMap<LineRange, (AsmInstruction, Vec<Bytecode>)>,
    label_map: HashMap<String, usize>,
}

impl CompileDebugInfo {

    pub fn new(asm_instructions: Vec<AsmInstruction>) -> CompileDebugInfo {

        let mut debug_map: BTreeMap<LineRange, (AsmInstruction, Vec<Bytecode>)> = BTreeMap::new();
        let mut index = 0;

        for i in asm_instructions {
            let bytecode = i.to_bytecode();
            let range: LineRange = LineRange { range: index..index + bytecode.len() };
            debug_map.insert(range, (i, bytecode.clone()));
            index += bytecode.len();
        }

        CompileDebugInfo { 
            debug_map,
            label_map: HashMap::new(),
        }
    }

    pub fn get(&self, bytecode_number: usize) -> Option<(AsmInstruction, Vec<Bytecode>)> {
        let keys = self.debug_map.keys();
        // check membership of bytecode_number in range of which the key is the range

        // lookup bytecode_number in the range of keys and retuirn the value
        let lookup_key = keys.clone().find_map(|key| {
            if key.range.contains(&bytecode_number) {
                return Some(key);
            }
            None
        });

        if lookup_key.is_some() {
            return self.debug_map.get(lookup_key.unwrap()).cloned();
        } else {
            return None;
        }

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

    pub fn get_instruction_by_bytecode_line_number(&self, line_number: usize) -> Option<(AsmInstruction, Vec<Bytecode>)> {
        self.compile_debug_info.get(line_number)
    }

    pub fn print_debug_info(&self) {
        let mut debug_stack_trace: Vec<(AsmInstruction, Vec<Bytecode>)> = Vec::new();
        unimplemented!()
    }
}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_bytecode_linenum_lookup() {

        let asm_instructions = vec![
            AsmInstruction::LI("$t0".to_string(), 456),
            AsmInstruction::LI("$t1".to_string(), 123),
            AsmInstruction::ADD("$t0".to_string(), "$t1".to_string(), "$t2".to_string()),
        ];

        let debug_info = CompileDebugInfo::new(asm_instructions.clone());

        dbg!(asm_instructions[1].to_bytecode().len());

        assert_eq!(debug_info.get(0), Some((asm_instructions[0].clone(), asm_instructions[0].to_bytecode())));
        assert_eq!(debug_info.get(1), Some((asm_instructions[0].clone(), asm_instructions[0].to_bytecode())));

        assert_eq!(debug_info.get(2), Some((asm_instructions[1].clone(), asm_instructions[1].to_bytecode())));
        assert_eq!(debug_info.get(3), Some((asm_instructions[1].clone(), asm_instructions[1].to_bytecode())));

        assert_eq!(debug_info.get(4), Some((asm_instructions[2].clone(), asm_instructions[2].to_bytecode())));
        assert_eq!(debug_info.get(5), Some((asm_instructions[2].clone(), asm_instructions[2].to_bytecode())));
        assert_eq!(debug_info.get(6), Some((asm_instructions[2].clone(), asm_instructions[2].to_bytecode())));
        assert_eq!(debug_info.get(7), Some((asm_instructions[2].clone(), asm_instructions[2].to_bytecode())));

        assert_eq!(debug_info.get(8), None);


    }
}
