enum Token {
    COMMA,

    LABEL(String),
    INSTRUCTION(String),
    REGISTER(String),
    IMMEDIATE(String),

    EOF,
}


