use crate::bytecode::AsmInstruction;

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
    current_label: Option<String>,
}

impl Lexer {

    pub fn new(source: String) -> Lexer {
        let lines = source.split("\n").map(|s| s.to_string()).collect();
        Lexer {
            source,
            lines,
            line_number: 1,
            current_label: None,
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
        let mut line = lexer.next();
        println!("{:?}", line);
        assert!(false == true);
    }

}

