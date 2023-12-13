/// one word = 4 bytes
/// a 32 bit word must be located
/// and accessed using a word aligned
/// address. This means that the address
/// must be divisible by 4 or the last
/// two bits must be 0.
struct Word {
    bytes: [u8; 4],
}

/// half word = 2 bytes
/// a 16 bit half word must be located
/// and accessed using a half word aligned 
/// address. This means that the address
/// the last lower order bit must be 0.
struct HalfWord {
    bytes: [u8; 2],
}

