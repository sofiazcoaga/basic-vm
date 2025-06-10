/// OPCODES
pub const OP_BR: u16 = 0; /* branch */
pub const OP_ADD: u16 = 1; /* add  */
pub const OP_LD: u16 = 2; /* load */
pub const OP_ST: u16 = 3; /* store */
pub const OP_JSR: u16 = 4; /* jump register */
pub const OP_AND: u16 = 5; /* bitwise and */
pub const OP_LDR: u16 = 6; /* load register */
pub const OP_STR: u16 = 7; /* store register */
pub const OP_RTI: u16 = 8; /* unused */
pub const OP_NOT: u16 = 9; /* bitwise not */
pub const OP_LDI: u16 = 10; /* load indirect */
pub const OP_STI: u16 = 11; /* store indirect */
pub const OP_JMP: u16 = 12; /* jump */
pub const OP_RES: u16 = 13; /* reserved (unused) */
pub const OP_LEA: u16 = 14; /* load effective address */
pub const OP_TRAP: u16 = 15;

/// REGISTERS
pub const R0: usize = 0;
pub const R1: usize = 1;
pub const R2: usize = 2;
pub const R3: usize = 3;
pub const R4: usize = 4;
pub const R5: usize = 5;
pub const R6: usize = 6;
pub const R7: usize = 7;
pub const PC: usize = 8; // Program Counter
pub const COND: usize = 9;
pub const REGISTER_COUNT: usize = 10;

/// FLAGS
pub const FL_POS: u16 = 1 << 0; // Positive
pub const FL_ZRO: u16 = 1 << 1; // Zero
pub const FL_NEG: u16 = 1 << 2; // Negative
