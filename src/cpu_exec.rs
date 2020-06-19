use crate::mca8086::{X86Instruction, rmf,regf, modf};
use crate::cpu::{Cpu};
use crate::mca8086::X86Flags::X86DispFlag;

//
// functions lookup table
//
static FUNCS: &'static [fn(&mut Cpu, &X86Instruction)] = &[
//         0x00  0x01  0x02  0x03  0x04  0x05  0x06  0x07  0x08  0x09  0x0A  0x0B  0x0C  0x0D  0x0E  0x0F
/* 0x00 */ opxx, op01, opxx, opxx, op04, op05, opxx, opxx, opxx, op09, opxx, opxx, opxx, opxx, opxx, opxx,
/* 0x10 */ opxx, opxx, opxx, opxx, opxx, opxx, opxx, opxx, opxx, op19, opxx, opxx, opxx, opxx, opxx, opxx,
/* 0x20 */ op20, opxx, opxx, opxx, opxx, opxx, opxx, opxx, opxx, op29, opxx, opxx, opxx, opxx, opxx, opxx,
/* 0x30 */ opxx, op31, opxx, opxx, opxx, opxx, opxx, opxx, opxx, op39, opxx, opxx, op3c, opxx, opxx, opxx,
/* 0x40 */ op40, op40, op40, op40, op40, op40, op40, op40, op4b, op4b, op4b, op4b, op4b, op4b, op4b, op4b,
/* 0x50 */ op51, op51, op51, op51, op51, op51, op51, op51, op58, op58, op58, op58, op58, op58, op58, op58,
/* 0x60 */ opxx, opxx, opxx, opxx, opxx, opxx, opxx, opxx, opxx, opxx, opxx, opxx, opxx, opxx, opxx, opxx,
/* 0x70 */ opxx, opxx, op72, opxx, op74, op75, op76, op77, opxx, op79, opxx, opxx, opxx, opxx, opxx, opxx,
/* 0x80 */ op81, op81, op81, op81, opxx, opxx, op86, opxx, op88, op89, op8a, op8b, opxx, opxx, opxx, opxx,
/* 0x90 */ op90, opxx, op92, opxx, opxx, opxx, opxx, opxx, opxx, opxx, opxx, opxx, opxx, opxx, opxx, opxx,
/* 0xA0 */ opxx, opxx, opxx, opxx, opxx, opxx, opxx, opxx, opxx, opxx, opxx, opxx, opxx, opxx, opxx, opxx,
/* 0xB0 */ opb0, opb0, opb0, opb0, opb0, opb0, opb0, opb0, opbc, opbc, opbc, opbc, opbc, opbc, opbc, opbc,
/* 0xC0 */ opxx, opxx, opxx, opc3, opxx, opxx, opc6, opc6, opxx, opxx, opxx, opxx, opxx, opxx, opxx, opxx,
/* 0xD0 */ opxx, opxx, opxx, opxx, opxx, opxx, opxx, opxx, opxx, opxx, opxx, opxx, opxx, opxx, opxx, opxx,
/* 0xE0 */ opxx, opxx, opxx, opxx, opxx, opxx, opxx, opxx, ope8, opxx, opxx, opeb, opxx, opxx, opxx, opxx,
/* 0xF0 */ opxx, opxx, opxx, opxx, opf4, opxx, opxx, opxx, opxx, opf9, opxx, opxx, opxx, opxx, opfe, opxx,
];

// Effective Address (without segments)
fn get_ea(cpu : &mut Cpu, instr : &X86Instruction, modf : u8, rmf : u8) -> usize {
    let mut ea = match rmf {
        0x00 => unsafe {(cpu.get_reg16(bx!()) + cpu.get_reg16( si!())) as usize},
        0x01 => unsafe {(cpu.get_reg16(bx!()) + cpu.get_reg16( di!())) as usize},
        0x02 => unsafe {(cpu.get_reg16(bp!()) + cpu.get_reg16( si!())) as usize},
        0x03 => unsafe {(cpu.get_reg16(bp!()) + cpu.get_reg16( di!())) as usize},
        0x04 => unsafe {cpu.get_reg16(si!()) as usize},
        0x05 => unsafe {cpu.get_reg16(di!()) as usize},
        0x06 => {
            match modf {
                0x00 => {instr.disp as usize} ,
                0x01 | 0x02 => unsafe {cpu.get_reg16( bp!()) as usize }

                _ => {0 as usize}
            }
        },
        0x07 => unsafe {cpu.get_reg16( bx!()) as usize},

        _ => {0 as usize}
    };

    ea = if modf == 1 || modf == 2 { ea as usize + instr.disp as usize } else {ea};

    ea
}

#[allow(arithmetic_overflow)]
fn cpu_readmem16(cpu : &mut Cpu, off : usize) -> u16 {
    let n:u16 = (cpu.get_memory(off + 1)) as u16;
    n << 8 | (cpu.get_memory(off)) as u16
}

fn cpu_readmem8(cpu : &mut Cpu, off : usize) -> u8 {
    (cpu.get_memory(off)) as u8
}

fn cpu_writemem8(cpu : &mut Cpu, off : usize, val : u8) {
    cpu.set_memory(off, val);
}

fn readmem16(cpu : &mut Cpu, instr : &X86Instruction) -> u16 {
    if instr.instr_flags & X86DispFlag  as u16 == X86DispFlag as u16 {
        let off = get_ea(cpu, instr, modf(instr.mod_reg_rm), rmf(instr.mod_reg_rm));
        cpu_readmem16(cpu, off)
    }
    else {
        unsafe {cpu.get_reg16(rmf(instr.mod_reg_rm) as usize)}
    }
}

fn readmem8(cpu : &mut Cpu, instr : &X86Instruction) -> u8 {
    if instr.instr_flags & X86DispFlag  as u16 == X86DispFlag as u16 {
        let off = get_ea(cpu, instr, modf(instr.mod_reg_rm), rmf(instr.mod_reg_rm));
        cpu_readmem8(cpu, off)
    }
    else {
        unsafe {cpu.get_reg8(rmf(instr.mod_reg_rm) as usize)}
    }
}

fn writemem16(cpu : &mut Cpu, instr : &X86Instruction, val : u16) {
    if modf(instr.mod_reg_rm) < 3 {
        let ea = get_ea(cpu, instr, modf(instr.mod_reg_rm), rmf(instr.mod_reg_rm));
        cpu_writemem8(cpu,ea, val as u8);
        cpu_writemem8(cpu,ea+1, (val >> 8) as u8);
    }
    else {
        unsafe{cpu.set_reg16(rmf(instr.mod_reg_rm) as usize, val);}
    }
}

fn writemem8(cpu : &mut Cpu, instr : &X86Instruction, val : u8) {
    if modf(instr.mod_reg_rm) < 3 {
        let ea = get_ea(cpu, instr, modf(instr.mod_reg_rm), rmf(instr.mod_reg_rm));
        cpu_writemem8(cpu,ea, val as u8);
    }
    else {
        unsafe{cpu.set_reg8(rmf(instr.mod_reg_rm) as usize, val);}
    }
}

// Begin instruction implementation
fn set_flag_xor16(cpu : &mut Cpu, op1 : u16, op2 : u16) {
    cpu.set_cf(0);
    cpu.set_of(0);
    setflag_sz16(cpu, op1^op2);
}

fn set_flag_or16(cpu : &mut Cpu, op1 : u16, op2 : u16) {
    cpu.set_cf(0);
    cpu.set_of(0);
    setflag_sz16(cpu, op1 | op2);
}

fn set_flag_or8(cpu : &mut Cpu, op1 : u8, op2 : u8) {
    cpu.set_cf(0);
    cpu.set_of(0);
    setflag_sz8(cpu, op1 | op2);
}

fn set_flag_sub16(cpu : &mut Cpu, op1 : u16, op2 : u16) {
    let res : u32 = (op1 as i16 - op2 as i16) as u32;
    setflag_sz16(cpu, res as u16);
    cpu.set_cf(if (res & 0xFFFF0000) > 0x00008000 { 1 } else { 0 });
}

fn set_flag_add16(cpu : &mut Cpu, op1 : u16, op2 : u16) {
    let res : u32 = op1 as u32 + op2 as u32;
    cpu.set_cf(if (res & 0xFFFF0000) > 0x00008000 { 1 } else { 0 });
}

fn set_flag_add8(cpu : &mut Cpu, op1 : u8, op2 : u8) {
    let res : u16 = op1 as u16 + op2 as u16;
    cpu.set_cf(if (res & 0xFF00) > 0x0800 { 1 } else { 0 });
}

fn setflag_sz16(cpu : &mut Cpu, val : u16) {
    let zf = if val == 0 { 1 } else {0};
    cpu.set_zf(zf);
    cpu.set_sf(((val & 0x8000) >> 15) as u8);
}

fn setflag_sz8(cpu : &mut Cpu, val : u8) {
    let zf = if val == 0 { 1 } else {0};

    cpu.set_zf(zf);
    cpu.set_sf(((val & 0x80) >> 7) as u8);
}

fn set_flag16(cpu : &mut Cpu, op1 : u16, op2 : u16) {
    let res : u32 = (op1 as i32 - op2 as i32) as u32;
    setflag_sz16(cpu, res as u16);
    cpu.set_cf(if res & 0xFFFF0000 > 0 {1} else {0});
}

fn set_flag8(cpu : &mut Cpu, op1 : u8, op2 : u8) {
    let res  = (op1 as i8 - op2 as i8) as u16;
    setflag_sz8(cpu, res as u8);
    cpu.set_cf(if res & 0xFF00 > 0 {1} else {0});
}

fn set_flag_and16(cpu : &mut Cpu, op1 : u16, op2 : u16) {
    let res = op1 & op2;
    setflag_sz16(cpu, res);
    cpu.set_cf(0);
    cpu.set_of(0);
}

fn set_flag_and8(cpu : &mut Cpu, op1 : u8, op2 : u8) {
    let res = op1 & op2;
    setflag_sz8(cpu, res);
    cpu.set_cf(0);
    cpu.set_of(0);
}

fn push(cpu : &mut Cpu, val : u16) {
    unsafe {cpu.set_reg16(sp!(), cpu.get_reg16(sp!()) - 2);}

    let off : usize;
    unsafe {off = cpu.get_reg16(sp!()) as usize;}
    cpu.set_memory(off, (val & 0xFF) as u8);
    cpu.set_memory(off-1, ((val >> 8) & 0xFF) as u8);
}

fn pop(cpu : &mut Cpu) -> u16 {
    let off : usize;
    unsafe{off = cpu.get_reg16(sp!()) as usize;}
    let mut val : u16 = cpu_readmem8(cpu, off - 1) as u16;
    val = ((val << 8) | (cpu_readmem8(cpu, off)) as u16) as u16;
    unsafe {cpu.set_reg16(sp!(), cpu.get_reg16(sp!()) + 2);}

    val
}

fn opxx(cpu : &mut Cpu, instr : &X86Instruction) {
    cpu.set_unop(true);
    println!("Unsupported Operation: 0x{:X}, ModRm: 0x{:X}", instr.op, instr.mod_reg_rm);
}

/*
 * ADD  Eb, Gb
 */
fn op01(cpu : &mut Cpu, instr : &X86Instruction) {
    unsafe {
        let op1 = cpu.get_reg16(regf(instr.mod_reg_rm) as usize);
        let op2 = readmem16(cpu, instr);
        set_flag_add16(cpu, op1, op2);
        writemem16(cpu, instr, (op1 as i32 + op2 as i32) as u16);
    }
}

/*
 * ADD  AL, Ib
 */
fn op04(cpu : &mut Cpu, instr : &X86Instruction) {
    unsafe {
        let dst = cpu.get_reg8(al!());
        let src = instr.imm as u8;
        cpu.set_reg8(al!(), dst+src);
        set_flag_add8(cpu, dst, src);
    }
}

/*
 * ADD  AX, Iv
 */
fn op05(cpu : &mut Cpu, instr : &X86Instruction) {
    unsafe {
        let dst = cpu.get_reg16(ax!());
        let src = instr.imm;
        cpu.set_reg16(ax!(), dst+src);
        set_flag_add16(cpu, dst, src);
    }
}

/*
 * OR  Ev, Gv
 */
fn op09(cpu : &mut Cpu, instr : &X86Instruction) {
    unsafe {
        let src = cpu.get_reg16(regf(instr.mod_reg_rm) as usize);
        let dst = readmem16(cpu, instr);
        set_flag_or16(cpu, dst, src);
        writemem16(cpu, instr, dst | src);
    }
}

/*
 * SBB  Ev, Gv
 *
 */
fn op19(cpu : &mut Cpu, instr : &X86Instruction) {
    unsafe {
        let op1 = readmem16(cpu, instr);
        let op2 = cpu.get_reg16(regf(instr.mod_reg_rm) as usize);

        let cf = cpu.get_cf() as u16;
        writemem16(cpu, instr,op1-op2+ cf);
        set_flag16(cpu, op1, op2 + cf);
    }
}

/*
 * AND  Eb, Gb
 */
fn op20(cpu : &mut Cpu, instr : &X86Instruction) {
    unsafe {
        let op1 = cpu.get_reg16(regf(instr.mod_reg_rm) as usize);
        let op2 = readmem16(cpu, instr);
        set_flag_and16(cpu, op2, op1);
        writemem16(cpu, instr, op1 & op2);
    }
}

/*
 * SUB  Ev, Gv
 */
fn op29(cpu : &mut Cpu, instr : &X86Instruction) {
    unsafe {
        let src = cpu.get_reg16(regf(instr.mod_reg_rm) as usize);
        let dst = readmem16(cpu, instr);
        set_flag16(cpu, dst, src);
        writemem16(cpu, instr, (dst as i32 - src as i32) as u16);
    }
}

/*
 * XOR  Ev, Gv
 */
fn op31(cpu : &mut Cpu, instr : &X86Instruction) {
    unsafe {
        let src = cpu.get_reg16(regf(instr.mod_reg_rm) as usize);
        let dst = readmem16(cpu, instr);
        set_flag_xor16(cpu, dst, src);
        writemem16(cpu, instr, dst^src);
    }
}

/*
 * CMP  Ev, Gv
 */
fn op39(cpu : &mut Cpu, instr : &X86Instruction) {
    unsafe {
        let op1 = readmem16(cpu, instr);
        let op2 = cpu.get_reg16(regf(instr.mod_reg_rm) as usize);
        set_flag16(cpu, op1, op2);
    }
}

/*
 * CMP  AL, Ib
 */
fn op3c(cpu : &mut Cpu, instr : &X86Instruction) {
    unsafe {
        let op2 = instr.imm as u8;
        let op1 = cpu.get_reg8(al!());
        set_flag8(cpu, op1, op2);
    }
}

/*
 *  INC  Zv
 */
fn op40(cpu : &mut Cpu, instr : &X86Instruction) {
    unsafe {
        let cf = cpu.get_cf();
        let op1= cpu.get_reg16((instr.op & 0x07) as usize);
        let op2 :u16 = 1;

        set_flag_add16(cpu, op1 as u16, op2 as u16);

        cpu.set_reg16((instr.op & 0x07) as usize, (op1 as i16 + op2 as i16) as u16);
        cpu.set_cf(cf);
    }
}

/*
 *  DEC  Zv
 */
fn op4b(cpu : &mut Cpu, instr : &X86Instruction) {
    let cf = cpu.get_cf();
    let op1;
    unsafe {op1 = cpu.get_reg16((instr.op & 0x07) as usize);}
    let op2 = 1;

    set_flag_sub16(cpu, op1, op2);
    unsafe {cpu.set_reg16((instr.op & 0x07) as usize, (op1 as i16 - op2 as i16) as u16);}
    cpu.set_cf(cf);
}

/*
 * PUSH  Zv
 */
fn op51(cpu : &mut Cpu, instr : &X86Instruction) {
    unsafe {push(cpu, cpu.get_reg16((instr.op & 0x07) as usize));}
}

/*
 * POP  Zv
 */
fn op58(cpu : &mut Cpu, instr : &X86Instruction) {
    let val = pop(cpu);
    unsafe {cpu.set_reg16((instr.op & 0x07) as usize, val);}
}

/*
 * JB/JNAE/JC  Jbs
 */
fn op72(cpu : &mut Cpu, instr : &X86Instruction) {
    cpu.set_jmp_taken(false);

    if cpu.get_cf() == 1 {
        let imm : i8 = instr. imm as i8;
        cpu.set_ip((instr.addr as i16 + imm as i16 + instr.length as i16) as u16);
        cpu.set_jmp_taken(true);
    }
}

/*
 * JZ/JE  Jbs
 */
fn op74(cpu : &mut Cpu, instr : &X86Instruction) {
    cpu.set_jmp_taken(false);

    if cpu.get_zf() == 1 {
        let imm : i8 = instr. imm as i8;
        cpu.set_ip((instr.addr as i16 + imm as i16 + instr.length as i16) as u16);
        cpu.set_jmp_taken(true);
    }
}

/*
 * JNZ/JNE  Jbs
 */
fn op75(cpu : &mut Cpu, instr : &X86Instruction) {
    cpu.set_jmp_taken(false);

    if cpu.get_zf() == 0 {
        let imm : i8 = instr.imm as i8;
        cpu.set_ip((instr.addr as i16 + (imm as i16) + instr.length as i16) as u16);
        cpu.set_jmp_taken(true);
    }
}

/*
 * JBE/JNA  Jbs
 */
fn op76(cpu : &mut Cpu, instr : &X86Instruction) {
    cpu.set_jmp_taken(false);

    if cpu.get_cf() == 1 || cpu.get_zf() == 1 {
        let imm : i8 = instr. imm as i8;
        cpu.set_ip((instr.addr as i16 + imm as i16 + instr.length as i16) as u16);
        cpu.set_jmp_taken(true);
    }
}

/*
 * JNBE/JA  Jbs
 */
fn op77(cpu : &mut Cpu, instr : &X86Instruction) {
    cpu.set_jmp_taken(false);

    if cpu.get_cf() == 0 && cpu.get_zf() == 0 {
        let imm : i8 = instr. imm as i8;
        cpu.set_ip((instr.addr as i16 + imm as i16 + instr.length as i16) as u16);
        cpu.set_jmp_taken(true);
    }
}

/*
 * JNS  Jbs
 */
fn op79(cpu : &mut Cpu, instr : &X86Instruction) {
    cpu.set_jmp_taken(false);

    if cpu.get_sf() == 0 {
        let imm : i8 = instr. imm as i8;
        cpu.set_ip((instr.addr as i16 + imm as i16 + instr.length as i16) as u16);
        cpu.set_jmp_taken(true);
    }
}

/*
 * SUB  Ev, Ibs
 */
fn op81_sub(cpu : &mut Cpu, instr : &X86Instruction) {
    if instr.op & 1 == 1 {
        let mut imm = instr.imm;
        let dst = readmem16(cpu, instr);

        if instr.op & 0x02 == 0x02 {
            imm = if imm & 0x80 == 0x80 {imm | 0xFF00 } else {imm};
        }

        set_flag16(cpu, dst, imm);
        writemem16(cpu, instr, (dst as i16 - imm as i16) as u16);
    }
    else {
        let mut imm = instr.imm as u8;
        let dst = readmem8(cpu, instr);

        if instr.op & 0x02 == 0x02 {
            imm = if imm & 0x80 == 0x80 {imm | 0xF0 } else {imm};
        }

        set_flag8(cpu, dst, imm);
        writemem8(cpu, instr, (dst as i8 - imm as i8) as u8);
    }
}

/*
 * OR  Eb, Ib
 */
fn op81_or(cpu : &mut Cpu, instr : &X86Instruction) {
    if instr.op & 1 == 1 {
        let imm = instr.imm;
        let dst = readmem16(cpu, instr);

        set_flag_or16(cpu, dst, imm);
        writemem16(cpu, instr, dst | imm);
    }
    else {
        let imm = instr.imm as u8;
        let dst = readmem8(cpu, instr);

        set_flag_or8(cpu, dst, imm);
        writemem8(cpu, instr, (dst | imm ) as u8);
    }
}


/*
 * AND  Eb, Ibs
 *
 */
fn op81_and(cpu : &mut Cpu, instr : &X86Instruction) {
    if instr.op & 1 == 1 {
        let mut imm = instr.imm;
        let dst = readmem16(cpu, instr);

        if instr.op & 0x02 == 0x02 {
            imm = if imm & 0x80 == 0x80 {imm | 0xFF00 } else {imm};
        }

        set_flag_and16(cpu, dst, imm);
        writemem16(cpu, instr, dst & imm);
    }
    else {
        let mut imm = instr.imm as u8;
        let dst = readmem8(cpu, instr);

        if instr.op & 0x02 == 0x02 {
            imm = if imm & 0x80 == 0x80 {imm | 0xF0 } else {imm};
        }

        set_flag_and8(cpu, dst, imm);
        writemem8(cpu, instr, (dst & imm ) as u8);
    }
}

/*
 * ADC  Ev, Ibs
 */
fn op81_adc(cpu : &mut Cpu, instr : &X86Instruction) {
    if instr.op & 1 == 1 {
        let mut imm = instr.imm;
        let dst = readmem16(cpu, instr);

        if instr.op & 0x02 == 0x02 {
            imm = if imm & 0x80 == 0x80 {imm | 0xFF00 } else {imm};
        }

        let cf = cpu.get_cf();
        writemem16(cpu, instr, dst + imm + cf as u16);
        set_flag16(cpu, dst, imm);

    }
    else {
        let mut imm = instr.imm as u8;
        let dst = readmem8(cpu, instr);

        if instr.op & 0x02 == 0x02 {
            imm = if imm & 0x80 == 0x80 {imm | 0xF0 } else {imm};
        }

        let cf = cpu.get_cf();
        writemem8(cpu, instr, (dst as i8 + imm as i8 + cf as i8) as u8);
        set_flag8(cpu, dst, imm);
    }
}

/*
 * ADD  Eb, Ib
 */
fn op81_add(cpu : &mut Cpu, instr : &X86Instruction) {
    if instr.op & 1 == 1 {
        let mut imm = instr.imm;
        let dst = readmem16(cpu, instr);

        if instr.op & 0x02 == 0x02 {
            imm = if imm & 0x80 == 0x80 {imm | 0xFF00 } else {imm};
        }

        set_flag_add16(cpu, dst, imm);
        writemem16(cpu, instr, (dst as i16 + imm as i16) as u16);
    }
    else {
        let mut imm = instr.imm as u8;
        let dst = readmem8(cpu, instr);

        if instr.op & 0x02 == 0x02 {
            imm = if imm & 0x80 == 0x80 {imm | 0xF0 } else {imm};
        }

        set_flag_add8(cpu, dst, imm);
        writemem8(cpu, instr, (dst as i8 + imm as i8) as u8);
    }
}

/*
 * CMP  Ev, Iv
 */
fn op81_cmp(cpu : &mut Cpu, instr : &X86Instruction) {
    if instr.op & 0x01 == 0x01 {
        let mut imm = instr.imm;

        if instr.op & 0x02 == 0x02 {
            imm = if imm & 0x80 == 0x80 {imm | 0xFF00 } else {imm};
        }

        let op1 = readmem16(cpu, instr);
        set_flag16(cpu, op1, imm);
    }
    else {
        opxx(cpu, instr);
    }
}

fn op81(cpu : &mut Cpu, instr : &X86Instruction) {
    static OP81FUNC: &'static [fn(&mut Cpu, &X86Instruction)] = &[
        op81_add,op81_or,op81_adc,opxx,op81_and,op81_sub,opxx,op81_cmp,
    ];

    OP81FUNC[regf(instr.mod_reg_rm) as usize](cpu, &instr);
}

/*
 * XCHG  Eb, Gb
 *
 */
fn op86(cpu : &mut Cpu, instr : &X86Instruction) {
    unsafe {
        let op1 = cpu.get_reg8(regf(instr.mod_reg_rm) as usize);
        let op2 = readmem8(cpu, instr);
        cpu.set_reg8(regf(instr.mod_reg_rm) as usize, op2);
        writemem8(cpu, instr, op1);
    }
}

/*
 * MOV  Eb, Gb
 */
fn op88(cpu : &mut Cpu, instr : &X86Instruction) {
    unsafe{writemem8(cpu, instr, cpu.get_reg8(regf(instr.mod_reg_rm) as usize));}
}

/*
 * MOV  Ev, Gv
 */
fn op89(cpu : &mut Cpu, instr : &X86Instruction) {
    unsafe{writemem16(cpu, instr, cpu.get_reg16(regf(instr.mod_reg_rm) as usize));}
}

/*
 * MOV  Eb, Gb
 */
fn op8a(cpu : &mut Cpu, instr : &X86Instruction) {
    unsafe {
        let val = readmem8(cpu, instr);
        cpu.set_reg8(regf(instr.mod_reg_rm) as usize, val);
    }
}

/*
 * NOP
 */
fn op90(_cpu : &mut Cpu, _instr : &X86Instruction) {
    // No operation
}

/*
 * XCHG  Gv, Ev
 */
fn op92(cpu : &mut Cpu, instr : &X86Instruction) {
    unsafe {
        let dst = cpu.get_reg16(ax!());
        let src = cpu.get_reg16((instr.op & 0x07) as usize);
        cpu.set_reg16(ax!(), src);
        cpu.set_reg16((instr.op & 0x07) as usize, dst);
    }
}

/*
 * MOV  Gv, Ev
 */
fn op8b(cpu : &mut Cpu, instr : &X86Instruction) {
    let val = readmem16(cpu,instr);
    unsafe{cpu.set_reg16(regf(instr.mod_reg_rm) as usize, val);}
}

/*
 * MOV  Zv, Iv
 */
fn opbc(cpu : &mut Cpu, instr : &X86Instruction) {
    unsafe {cpu.set_reg16((instr.op & 0x07) as usize, instr.imm); }
}

/*
 * MOV  Zb, Ib
 */
fn opb0(cpu : &mut Cpu, instr : &X86Instruction) {
    unsafe {cpu.set_reg8((instr.op & 0x07) as usize, instr.imm as u8); }
}

/*
 * RETN
 */
fn opc3(cpu : &mut Cpu, _instr : &X86Instruction) {
    let ip = pop(cpu);
    cpu.set_ip(ip);
    cpu.set_jmp_taken(true);
}

/*
 * MOV  Ev, Gv
 * MOV  Eb, Gb
 */
fn opc6_mov(cpu : &mut Cpu, instr : &X86Instruction) {
    if instr.op & 0x01 == 0x01 {
        let imm = instr.imm;
        writemem16(cpu, instr, imm);
    }
    else {
        let imm: u8 = instr.imm as u8;
        writemem8(cpu, instr, imm);
    }
}

fn opc6(cpu : &mut Cpu, instr : &X86Instruction) {
    static OPC6FUNC: &'static [fn(&mut Cpu, &X86Instruction)] = &[
        opc6_mov,opxx,opxx,opxx,opxx,opxx,opxx,opxx,
    ];

    OPC6FUNC[regf(instr.mod_reg_rm) as usize](cpu, &instr);
}


/*
 * CALL  Jv
 */
fn ope8(cpu : &mut Cpu, instr : &X86Instruction) {
    let ip = cpu.get_ip();
    push(cpu,ip + 3);
    cpu.set_ip((instr.addr as i16 + instr.imm as i16 + instr.length as i16) as u16);
    cpu.set_jmp_taken(true);
}

/*
 * JMP  Jbs
 */
fn opeb(cpu : &mut Cpu, instr : &X86Instruction) {
    let imm = instr.imm as i8;
    cpu.set_ip((instr.addr as i16 + imm as i16 + instr.length as i16) as u16);
    cpu.set_jmp_taken(true);
}

/*
 * HLT
 */
fn opf4(cpu : &mut Cpu, _instr : &X86Instruction) {
    cpu.set_suspend(true);
}

/*
 * STC
 */
fn opf9(cpu : &mut Cpu, _instr : &X86Instruction) {
    cpu.set_cf(1);
}

/*
 * INC  Eb
 */
fn opfe_add8(cpu : &mut Cpu, instr : &X86Instruction) {
    let cf = cpu.get_cf();
    let op1 = readmem8(cpu, instr);
    let op2 = 1 as u8;

    set_flag_add8(cpu, op1, op2);
    writemem8(cpu, instr, op1+op2);
    cpu.set_cf(cf);
}

/*
 * DEC  Eb
 */
fn opfe_sub8(cpu : &mut Cpu, instr : &X86Instruction) {
    let cf = cpu.get_cf();
    let op1 = readmem8(cpu, instr);
    let op2 = 1 as u8;

    set_flag8(cpu, op1, op2);
    writemem8(cpu, instr, op1-op2);
    cpu.set_cf(cf);
}

fn opfe(cpu : &mut Cpu, instr : &X86Instruction) {
    match regf(instr.mod_reg_rm) {
        0 => {
            if instr.op & 0x01 == 0x01 {
                opxx(cpu, instr);
            }
            else {
                opfe_add8(cpu, instr);
            }
        }
        1 => {
            if instr.op & 0x01 == 0x01 {
                opxx(cpu, instr);
            }
            else {
                opfe_sub8(cpu, instr);
            }
        }

        _ => {opxx(cpu, instr);}
    }
}

pub fn exec(cpu : &mut Cpu, instr : &X86Instruction) {
    cpu.set_op(instr.op);
    FUNCS[instr.op as usize](cpu, &instr);
}

