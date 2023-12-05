use std::collections::HashSet;

use crate::{bytecode::AsmInstruction, debug_table::CompileDebugInfo};

enum Token {
    COMMA,

    LABEL(String),
    INSTRUCTION(String),
    REGISTER(String),
    IMMEDIATE(String),

    EOF,
}

pub struct ParsedInstruction {
    asm_ins: AsmInstruction,
    line_num: usize,
    label: String,
}

pub struct Lexer {
    source: String,
    lines: Vec<String>,
    line_number: usize,
    labels: HashSet<String>,
    compile_dbg: CompileDebugInfo,
}

impl Lexer {

    pub fn new(source: String) -> Lexer {
        let lines = source.split("\n").map(|s| s.to_string()).collect();
        Lexer {
            source,
            lines,
            line_number: 1,
            labels: HashSet::new(),
            compile_dbg: CompileDebugInfo::new(),
        }
    }

    pub fn next(&mut self) -> Option<String> {
        match self.lines.get(self.line_number - 1) {
            Some(line) => {
                self.line_number += 1;
                Some(line.clone())
            },
            None => None,
        }
    }

    // parses a line and then calls on specific functions to parse the line
    pub fn parse(&mut self) {
        let line = self.next();
        match line {
            Some(line) => {
                let mut line = line.clone();
                line = self.consume_whitespace(line);
                line = self.consume_comment(line);
                let label = self.parse_label(line.clone());
                let instruction = self.parse_instruction(line.clone());
                let register = self.parse_register(line.clone());
                let immediate = self.parse_immediate(line.clone());
                match label {
                    Some(label) => {
                        self.labels.insert(label);
                    },
                    None => (),
                }
                match instruction {
                    Some(instruction) => {
                        let parsed_instruction = ParsedInstruction {
                            asm_ins: AsmInstruction::new(instruction),
                            line_num: self.line_number,
                            label: "".to_string(),
                        };
                        self.compile_dbg.add_instruction(parsed_instruction);
                    },
                    None => (),
                }
                match register {
                    Some(register) => {
                        self.compile_dbg.add_register(register);
                    },
                    None => (),
                }
                match immediate {
                    Some(immediate) => {
                        self.compile_dbg.add_immediate(immediate);
                    },
                    None => (),
                }
            },
            None => (),
        }
    }

    pub fn parse_label(&mut self, line: String) -> Option<String> {
        None
    }

    pub fn parse_instruction(&mut self, line: String) -> Option<String> {
        None
    }

    pub fn parse_register(&mut self, line: String) -> Option<String> {
        None
    }

    pub fn parse_immediate(&mut self, line: String) -> Option<String> {
        None
    }

    pub fn consume_whitespace(&mut self, line: String) -> Option<String> {
        None
    }

    pub fn consume_comment(&mut self, line: String) -> Option<String> {
        None
    }

}

#[cfg(test)]
mod tests {

    #[test]
    fn test_hello_world() {
        let src = r#"
            .text
            main:
                li $v0, 4
                la $a0, msg
                syscall
                li $v0, 10
                syscall
            .data
            msg: .asciiz "Hello, world!"#;
        let mut lexer = super::Lexer::new(src.to_string());
        let line = lexer.next();
        println!("{:?}", line);
        assert!(false == true);
    }

}

