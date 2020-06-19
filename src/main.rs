mod mca8086;
#[macro_use] mod cpu;
mod cpu_exec;

use std::fs::{File};
use std::{fs, io};
use std::io::Read;

use crate::mca8086::decode;
use crate::mca8086::X86Instruction;
use crate::cpu::{Cpu, init};
use std::char;
use crate::cpu_exec::exec;

unsafe fn print_internal_state(cpu :&mut Cpu){
    println!("------------------ CPU INTERNAL STATE -----------------");
    println!("\tGENERAL PURPOSE REGISTERS");
    println!("16-bit\t\tAX: {:X}, BX: {:X}, CX: {:X}, DX: {:X}",
             cpu.get_reg16(ax!()), cpu.get_reg16(bx!()),
             cpu.get_reg16(cx!()), cpu.get_reg16(dx!())
    );

    println!("8-bit\t\tAH: {:X}, BH: {:X}, CH: {:X}, DH: {:X}\n\t\t\tAL: {:X}, BL: {:X}, CL: {:X}, DL: {:X}",
             cpu.get_reg8(ah!()), cpu.get_reg8(bh!()),
             cpu.get_reg8(ch!()), cpu.get_reg8(dh!()),
             cpu.get_reg8(al!()), cpu.get_reg8(bl!()),
             cpu.get_reg8(cl!()), cpu.get_reg8(dl!())
    );

    println!("\n\tSTACK POINTER AND INDEXES");
    println!("16-bit\t\tSI: {:X}, DI: {:X}, SP: {:X}, BP: {:X}, IP: {:X}",
             cpu.get_reg16(si!()), cpu.get_reg16(di!()),
             cpu.get_reg16(sp!()), cpu.get_reg16(bp!()), cpu.get_ip()
    );
    println!("\n\tFLAGS");
    println!("16-bit\t\tx x x x o d i t s z x a x p x c");
    println!("\t\t\t\t\t{} {} {} {} {} {}   {}   {}   {}",
              cpu.get_of(), cpu.get_df(), cpu.get_if(), cpu.get_tf(), cpu.get_sf(),
              cpu.get_zf(), cpu.get_af(), cpu.get_pf(), cpu.get_cf());
    println!("-------------------------------------------------------\n");
}

fn print_vram(cpu: &mut Cpu) {

    let off = 0x8000; // Video Memory offset
    for r in 0..25 {
        for c in 0..80 {
            let ch  = cpu.get_memory(off + (r * 80) + c);
            print!("{}", (if ch == 0 { ' ' } else {ch as char}));
        }
        println!();
    }
}

fn read_file(file_name : &str) -> Vec<u8> {
    let mut hfile = File::open(file_name).expect("Error opening file");
    let meta = fs::metadata(file_name).expect("No metadata found");
    let mut buff = vec![0;meta.len() as usize];
    hfile.by_ref().read(&mut buff).expect("Error reading bytes");

    buff
}

fn main() {
    let filename = "codegolf.com";
    let data = read_file(filename);

    let mut cpu;

    unsafe {
         cpu = init(data);
         //print_internal_state(&mut cpu);
    }

    loop {
        let mut instr: X86Instruction = X86Instruction::default();
        let len = decode(&mut instr, cpu.get_ip() as usize, cpu.get_mem_buff().as_ref());

        exec(&mut cpu, &instr);

        if !cpu.get_jmp_taken() {
            cpu.inc_ip(len as u16);
        }

        if cpu.get_suspend() {
            break;
        }

        if cpu.get_unop() {
            break;
        }

        // unsafe {print_internal_state(&mut cpu);}
        cpu.set_jmp_taken(false);
    }

    print_vram(&mut cpu);

    let mut stdin = io::stdin();
    let _ = stdin.read(&mut [0u8]).unwrap();
}