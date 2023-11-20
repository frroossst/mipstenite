pub enum Token {
    Label(String),
    Mnemonic(String),
    Register(String),
    Immediate(i32),
    Comment(String),
    NewLine,
}

pub enum Operand {
    Register(String),
    Immediate(String),
    Label(String),
}

pub enum Instruction {
    RType {
        mnemonic: String,
        rd: Operand,
        rs: Operand,
        rt: Operand,
    },
    IType {
        mnemonic: String,
        rd: Operand,
        rs: Operand,
        imm: Operand,
    },
    JType {
        mnemonic: String,
        target: String,
    },
}

pub struct AsmInstruction {
    label: Option<String>,
    instruction: Option<Instruction>,
}

pub struct AsmLexer;

impl AsmLexer {
    pub fn tokenize_line(line: &str) -> Vec<Token> {
        let mut tokens = Vec::new();

        let parts: Vec<&str> = line.split_whitespace().collect();

        for i in parts {
            // match label
            if i.ends_with(':') {
                tokens.push(Token::Label(i.trim_end_matches(':').to_string()));
            // match mnemonic
            } else if Self::is_mnemonic(i) {
                tokens.push(Token::Mnemonic(i.to_string()));
            // match register
            } else if i.starts_with('$') {
                tokens.push(Token::Register(i.to_string()));
            // match immediate
            } else if let Ok(value) = i.parse::<i32>() {
                tokens.push(Token::Immediate(value));
            // match comments
            } else if i.starts_with('#') {
                tokens.push(Token::Comment(i.to_string()));
            } else if i.starts_with('\n') {
                tokens.push(Token::NewLine);
            }
        }
        tokens
    }

    pub fn is_mnemonic(token: &str) -> bool {
        match token {
            "add" | "sub" | "and" | "or" | "xor" | "nor" | "slt" | "sll" | "srl" | "sra" | "jr"
            | "addi" | "andi" | "ori" | "xori" | "slti" | "lw" | "sw" | "beq" | "bne" | "j"
            | "jal" => true,
            _ => false,
        }
    }
}



