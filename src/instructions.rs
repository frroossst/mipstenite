use crate::registers::Register;

pub enum InstructionMnemonic {
    add,
    addi,
    addiu,
    addu,
    and,
    andi,
    beq,
    blez,
    bne,
    bgtz,
    div,
    divu,
    j,
    jal,
    jr,
    lb,
    lbu,
    lhu,
    lui,
    lw,
    mfhi,
    mthi,
    mflo,
    mtlo,
    mfc0,
    mult,
    multu,
    nor,
    xor,
    or,
    ori,
    sb,
    sh,
    slt,
    slti,
    sltiu,
    sltu,
    sll,
    srl,
    sra,
    sub,
    subu,
    sw,
}

pub struct RInstructions {
    mnemonic: InstructionMnemonic,
    rs: Register,
    rt: Register,
    rd: Register,
}

pub struct IInstructions {
    mnemonic: InstructionMnemonic,
    rs: Register,
    rt: Register,
    imm: u32,
}

pub struct JInstructions {
    mnemonic: InstructionMnemonic,
    addr: u32,
}


