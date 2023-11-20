#[non_exhaustive]
pub struct OPCODES;

impl OPCODES {
    pub const ALU64_ADD_IMM: u8 = 0x07;
    pub const ALU64_ADD_REG: u8 = 0x0f;
    pub const ALU64_SUB_IMM: u8 = 0x17;
    pub const ALU64_SUB_REG: u8 = 0x1f;
    pub const ALU64_MUL_IMM: u8 = 0x27;
    pub const ALU64_MUL_REG: u8 = 0x2f;
    pub const ALU64_DIV_IMM: u8 = 0x37;
    pub const ALU64_DIV_REG: u8 = 0x3f;
    pub const ALU64_OR_IMM: u8 = 0x47;
    pub const ALU64_OR_REG: u8 = 0x4f;
    pub const ALU64_AND_IMM: u8 = 0x57;
    pub const ALU64_AND_REG: u8 = 0x5f;
    pub const ALU64_LSH_IMM: u8 = 0x67;
    pub const ALU64_LSH_REG: u8 = 0x6f;
    pub const ALU64_RSH_IMM: u8 = 0x77;
    pub const ALU64_RSH_REG: u8 = 0x7f;
    pub const ALU64_NEG: u8 = 0x87;
    pub const ALU64_MOD_IMM: u8 = 0x97;
    pub const ALU64_MOD_REG: u8 = 0x9f;
    pub const ALU64_XOR_IMM: u8 = 0xa7;
    pub const ALU64_XOR_REG: u8 = 0xaf;
    pub const ALU64_MOV_IMM: u8 = 0xb7;
    pub const ALU64_MOV_REG: u8 = 0xbf;
    pub const ALU64_ARSH_IMM: u8 = 0xc7;
    pub const ALU64_ARSH_REG: u8 = 0xcf;
}

#[non_exhaustive]
pub struct BYTESWAP_INSTRUCTIONS;

impl BYTESWAP_INSTRUCTIONS {
    pub const BYTESWAP_LE16: u8 = 0xd4;
    pub const BYTESWAP_LE32: u8 = 0xd4;
    pub const BYTESWAP_LE64: u8 = 0xd4;
    pub const BYTESWAP_BE16: u8 = 0xdc;
    pub const BYTESWAP_BE32: u8 = 0xdc;
    pub const BYTESWAP_BE64: u8 = 0xdc;
}

#[non_exhaustive]
pub struct MEMORY_INSTRUCTIONS;

impl MEMORY_INSTRUCTIONS {
    pub const MEM_LDDW: u8 = 0x18;
    pub const MEM_LDABSW: u8 = 0x20;
    pub const MEM_LDABSH: u8 = 0x28;
    pub const MEM_LDABSB: u8 = 0x30;
    pub const MEM_LDABSDW: u8 = 0x38;
    pub const MEM_LDINDW: u8 = 0x40;
    pub const MEM_LDINDH: u8 = 0x48;
    pub const MEM_LDINDB: u8 = 0x50;
    pub const MEM_LDINDDW: u8 = 0x58;
    pub const MEM_LDXW: u8 = 0x61;
    pub const MEM_LDXH: u8 = 0x69;
    pub const MEM_LDXB: u8 = 0x71;
    pub const MEM_LDXDW: u8 = 0x79;
    pub const MEM_STW: u8 = 0x62;
    pub const MEM_STH: u8 = 0x6a;
    pub const MEM_STB: u8 = 0x72;
    pub const MEM_STDW: u8 = 0x7a;
    pub const MEM_STXW: u8 = 0x63;
    pub const MEM_STXH: u8 = 0x6b;
    pub const MEM_STXB: u8 = 0x73;
    pub const MEM_STXDW: u8 = 0x7b;
}

#[non_exhaustive]
pub struct BRANCH_INSTRUCTIONS;

impl BRANCH_INSTRUCTIONS {
    pub const BRANCH_JA: u8 = 0x05;
    pub const BRANCH_JEQ_IMM: u8 = 0x15;
    pub const BRANCH_JEQ_REG: u8 = 0x1d;
    pub const BRANCH_JGT_IMM: u8 = 0x25;
    pub const BRANCH_JGT_REG: u8 = 0x2d;
    pub const BRANCH_JGE_IMM: u8 = 0x35;
    pub const BRANCH_JGE_REG: u8 = 0x3d;
    pub const BRANCH_JLT_IMM: u8 = 0xa5;
    pub const BRANCH_JLT_REG: u8 = 0xad;
    pub const BRANCH_JLE_IMM: u8 = 0xb5;
    pub const BRANCH_JLE_REG: u8 = 0xbd;
    pub const BRANCH_JSET_IMM: u8 = 0x45;
    pub const BRANCH_JSET_REG: u8 = 0x4d;
    pub const BRANCH_JNE_IMM: u8 = 0x55;
    pub const BRANCH_JNE_REG: u8 = 0x5d;
    pub const BRANCH_JSGT_IMM: u8 = 0x65;
    pub const BRANCH_JSGT_REG: u8 = 0x6d;
    pub const BRANCH_JSGE_IMM: u8 = 0x75;
    pub const BRANCH_JSGE_REG: u8 = 0x7d;
    pub const BRANCH_JSLT_IMM: u8 = 0xc5;
    pub const BRANCH_JSLT_REG: u8 = 0xcd;
    pub const BRANCH_JSLE_IMM: u8 = 0xd5;
    pub const BRANCH_JSLE_REG: u8 = 0xdd;
    pub const BRANCH_CALL: u8 = 0x85;
    pub const BRANCH_EXIT: u8 = 0x95;
}

pub const MEMORY_SIZE: u32 = 65536;

pub struct Instruction {
    pub opcode: u8,
    pub dest: u8,
    pub src: u8,
    pub offset: u16,
    pub imm: u32,
}

// pub type MemByte<'a> = &'a [u8];

pub struct State {
    pub program_counter: isize,
    pub registers: [i64; 11],
    pub memory: Vec<Vec<u8>>,
}

fn main() {
    let a = 8;
    match a {
        OPCODES::ALU64_ADD_IMM => println!("{}", 0x07),
        _ => {
            panic!("invalid opcode")
        }
    }
}
