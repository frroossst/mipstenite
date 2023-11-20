pub enum IR {
    // Arithmetic
    ADD,
    SUB,
    MUL,
    DIV,
    LT,
    GT,
    LTE,
    GTE,
    // Control flow
    JUMP,
    // Memory
    LOAD,
    STORE,
    // Immediate
    LOCAL,
}

