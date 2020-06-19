//
// MCA - Machine Code Analyzer
// 8086 instruction decoder
//

use crate::mca8086::X86Flags::*;
use crate::mca8086::Gr25Prefix::*;


macro_rules! x86    {() => { 0x01 }}

// Coprocessor Escape
macro_rules! x87 {() => {0x02}}

// data size constants
macro_rules! b {()   => {0x01}}
macro_rules! w {()   => {0x02}}


static PREFIXES: &'static [u8] = &[
    //       00  01  02  03  04  05  06  07  08  09  0A  0B  0C  0D  0E  0F
    /* 00 */ 0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
    /* 10 */ 0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
    /* 20 */ 0,  0,  0,  0,  0,  0,  x86!(), 0,  0,  0,  0,  0,  0,  0,  x86!(),0,
    /* 30 */ 0,  0,  0,  0,  0,  0,  x86!(), 0,  0,  0,  0,  0,  0,  0,  x86!(),0,
    /* 40 */ 0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
    /* 50 */ 0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
    /* 60 */ 0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
    /* 70 */ 0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
    /* 70 */ 0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
    /* 90 */ 0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
    /* A= */ 0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
    /* B0 */ 0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
    /* C0 */ 0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
    /* D0 */ 0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
    /* E0 */ 0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
    /* F0 */ 0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0
];

static MOD_RM_1B: &'static [u8] = &[
    //      00 01 02 03 04 05 06 07 08 09 0A 0B 0C 0D 0E 0F
    /* 00 */ 1, 1, 1, 1, 0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 0,
    /* 10 */ 1, 1, 1, 1, 0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 0,
    /* 20 */ 1, 1, 1, 1, 0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 0,
    /* 30 */ 1, 1, 1, 1, 0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 0,
    /* 40 */ 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    /* 50 */ 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    /* 60 */ 0, 0, 1, 1, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0,
    /* 70 */ 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    /* 80 */ 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    /* 90 */ 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    /* A0 */ 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    /* B0 */ 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    /* C0 */ 1, 1, 0, 0, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0,
    /* D0 */ 1, 1, 1, 1, 0, 0, 0, 0, x87!(), x87!(), x87!(), x87!(), x87!(), x87!(), x87!(), x87!(),
    /* E0 */ 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    /* F0 */ 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 1, 1
];

static IMM_1B : &'static [u8] = &[
    //      00 01 02 03 04 05 06 07 08 09 0A 0B 0C 0D 0E 0F
    /* 00 */ 0, 0, 0, 0, b!(), w!(), 0, 0, 0, 0, 0, 0, b!(), w!(), 0, 0,
    /* 10 */ 0, 0, 0, 0, b!(), w!(), 0, 0, 0, 0, 0, 0, b!(), w!(), 0, 0,
    /* 20 */ 0, 0, 0, 0, b!(), w!(), 0, 0, 0, 0, 0, 0, b!(), w!(), 0, 0,
    /* 30 */ 0, 0, 0, 0, b!(), w!(), 0, 0, 0, 0, 0, 0, b!(), w!(), 0, 0,
    /* 40 */ 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    /* 50 */ 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    /* 60 */ 0, 0, 0, 0, 0, 0, 0, 0, w!(), w!(), b!(), b!(), 0, 0, 0, 0,
    /* 70 */ b!(), b!(), b!(), b!(), b!(), b!(), b!(), b!(), b!(), b!(), b!(), b!(), b!(), b!(), b!(), b!(),
    /* 80 */ b!(), w!(), b!(), b!(), 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    /* 90 */ 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, w!(), 0, 0, 0, 0, 0,
    /* A0 */ w!(), w!(), w!(), w!(), 0, 0, 0, 0, b!(), w!(), 0, 0, 0, 0, 0, 0,
    /* B0 */ b!(), b!(), b!(), b!(), b!(), b!(), b!(), b!(), w!(), w!(), w!(), w!(), w!(), w!(), w!(), w!(),
    /* C0 */ b!(), b!(), w!(), 0, 0, 0, b!(), w!(), w!(), 0, w!(), 0, 0, b!(), 0, 0,
    /* D0 */ 0, 0, 0, 0, b!(), b!(), 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    /* E0 */ b!(), b!(), b!(), b!(), b!(), b!(), b!(), b!(), w!(), w!(), w!(), b!(), 0, 0, 0, 0,
    /* F0 */ 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
];

enum Gr25Prefix {
    OpCs = 1,
    OpDs = 2,
    OpEs = 4,
    OpSs = 8,
}

pub enum X86Flags {
    X86PrefixFlag = 1,
    X86ModregFlag = 2,
    X86DispFlag   = 4,
    X86ImmFlag    = 8,
}

#[derive(Default)]
pub struct X86Instruction {
    pub setprefix : u16,
    pub prefixes : [u8; 16],
    pub prefix_cnt : u8,

    pub op : u8,
    pub instr_flags:u16,
    pub mod_reg_rm : u8,

    pub imm : u16,
    pub disp: u16,
    pub disp_len: usize,
    pub addr : usize,
    pub length : usize
}


pub fn modf(val : u8) -> u8 {
    (val >> 6) as u8
}

pub fn regf(val : u8) -> u8 {
    (val >> 3) & 0x7 as u8
}

pub fn rmf(val : u8) -> u8 {
    (val & 0x07) as u8
}


fn read_val(data : &Vec<u8>, index : usize, val_size : usize) -> u16{
    match val_size {
        1 => { data[index] as u16 },
        2 => { ((data[index + 1] as u16) << 8) | (data[index]) as u16},
        _ => { 0 }
    }
}

fn disp_decode(mod_val : u32, rm_val : u32) -> usize {
    if rm_val == 6 && mod_val == 0 {
        return 2;
    }
    mod_val as usize
}

fn mod_decode(instr: &mut X86Instruction, off : usize, data : &Vec<u8>) {
    let mut index = off;

    if MOD_RM_1B[instr.op as usize] > 0 {
        instr.instr_flags = instr.instr_flags | X86ModregFlag as u16;
        instr.mod_reg_rm = data[off];
        instr.length += 1;
        index += 1;

        // mod = 11b ? displacement present
        if modf(instr.mod_reg_rm) < 3 {
            instr.instr_flags = instr.instr_flags | X86DispFlag as u16;
            instr.disp_len = disp_decode(modf(instr.mod_reg_rm) as u32, rmf(instr.mod_reg_rm) as u32);

            instr.disp = read_val(data, index, instr.disp_len);

            instr.length+=instr.disp_len;
            index += instr.disp_len;
        }
    }

    // let's check imm
    let imm_size = IMM_1B[instr.op as usize] as usize;
    if imm_size > 0 {
        instr.instr_flags = instr.instr_flags | X86ImmFlag as u16;
        instr.imm = read_val(data, index, imm_size);
        instr.length += imm_size as usize;
    }
}

pub fn decode(instr: &mut X86Instruction, off : usize, data : &Vec<u8>) -> usize {
    let mut cur_byte = data[off];
    instr.addr = off;

    while PREFIXES[cur_byte as usize] != 0 {
        instr.instr_flags = instr.instr_flags | X86PrefixFlag as u16;

        match cur_byte {
            0x26 => { instr.setprefix = instr.setprefix | OpEs as u16},
            0x2E => { instr.setprefix = instr.setprefix | OpCs as u16},
            0x36 => { instr.setprefix = instr.setprefix | OpSs as u16},
            0x3E => { instr.setprefix = instr.setprefix | OpDs as u16},
            _ => {}
        };

        instr.prefix_cnt+=1;
        instr.prefixes[instr.prefix_cnt as usize] = cur_byte;
        instr.length+=1;

        cur_byte = data[off+instr.length];
    }

    instr.length+=1;
    instr.op = cur_byte;

    mod_decode(instr, off+instr.length, data);

    instr.length
}