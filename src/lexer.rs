enum Token {
    COMMA,

    LABEL(String),
    INSTRUCTION(String),
    REGISTER(String),
    IMMEDIATE(String),

    EOF,
}


/// Lexer to parse MIPS assmebly code
/// to AsmInstruction struct
pub struct Lexer {
    pub input: String,
    pub position: usize,
    pub read_position: usize,
    pub ch: char,
}

impl Lexer {

    pub fn new(input: String) -> Lexer {
        let mut l = Lexer {
            input,
            position: 0,
            read_position: 0,
            ch: '\0',
        };
        l.read_char();
        l
    }

    pub fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = '\0';
        } else {
            self.ch = self.input.chars().nth(self.read_position).unwrap();
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        let tok = match self.ch {
            ',' => Token::COMMA,
            '\0' => Token::EOF,
            _ => {
                if self.ch.is_alphabetic() {
                    let literal = self.read_identifier();
                    match literal.as_str() {
                        "add" => Token::INSTRUCTION(literal),
                        "addi" => Token::INSTRUCTION(literal),
                        "sub" => Token::INSTRUCTION(literal),
                        "lw" => Token::INSTRUCTION(literal),
                        "sw" => Token::INSTRUCTION(literal),
                        "beq" => Token::INSTRUCTION(literal),
                        "bne" => Token::INSTRUCTION(literal),
                        "j" => Token::INSTRUCTION(literal),
                        "jal" => Token::INSTRUCTION(literal),
                        "jr" => Token::INSTRUCTION(literal),
                        "jalr" => Token::INSTRUCTION(literal),
                        "nop" => Token::INSTRUCTION(literal),
                        _ => Token::LABEL(literal),
                    }
                } else if self.ch.is_numeric() {
                    let literal = self.read_number();
                    Token::IMMEDIATE(literal)
                } else if self.ch == '$' {
                    let literal = self.read_register();
                    Token::REGISTER(literal)
                } else {
                    panic!("unexpected character: {}", self.ch);
                }
            }
        };
        self.read_char();
        tok
    }


}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next_token() {
        let input = "add $t0, $t1, $t2";
        let mut l = Lexer::new(input.to_string());

        let tests = vec![
            Token::INSTRUCTION("add".to_string()),
            Token::REGISTER("$t0".to_string()),
            Token::COMMA,
            Token::REGISTER("$t1".to_string()),
            Token::COMMA,
            Token::REGISTER("$t2".to_string()),
            Token::EOF,
        ];

        for t in tests {
            let tok = l.next_token();
            assert_eq!(tok, t);
        }
    }
}



