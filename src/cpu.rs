//
// CPU (partial) Emulation
//

#[macro_export]
macro_rules! ram_memory {
    () => {0x100000};
}

#[macro_export]
macro_rules! ax {() => {0x00};}
#[macro_export]
macro_rules! cx {() => {0x01};}
#[macro_export]
macro_rules! dx {() => {0x02};}


#[macro_export]
macro_rules! bx {() => {0x03};}

#[macro_export]
macro_rules! sp {() => {0x04};}
#[macro_export]
macro_rules! bp {() => {0x05};}
#[macro_export]
macro_rules! si {() => {0x06};}
#[macro_export]
macro_rules! di {() => {0x07};}

#[macro_export]
macro_rules! al {() => {0x00};}
#[macro_export]
macro_rules! cl {() => {0x01};}
#[macro_export]
macro_rules! dl {() => {0x02};}
#[macro_export]
macro_rules! bl {() => {0x03};}
#[macro_export]
macro_rules! ah {() => {0x04};}
#[macro_export]
macro_rules! ch {() => {0x05};}
#[macro_export]
macro_rules! dh {() => {0x06};}
#[macro_export]
macro_rules! bh {() => {0x07};}

#[macro_export]
macro_rules! es {() => {0x00};}
#[macro_export]
macro_rules! cs {() => {0x01};}
#[macro_export]
macro_rules! ss {() => {0x02};}
#[macro_export]
macro_rules! ds {() => {0x03};}


static REGMAP : &'static [u8] = &[0x00, 0x02, 0x04, 0x06, 0x01, 0x03, 0x05, 0x07];


pub union Reg {
    pub reg16 : [u16;8], // AX,        CX,       DX,       BX,     SP, BP, SI, DI
    pub reg8  : [u8;8],  // [AL, AH], [CL, CH], [DL, DH], [BL, BH]
}

pub struct Cpu {
    reg :  Reg,

    // 0 1 2 3 4 5 6 7 8 9 A B C D E F
    // x x x x o d i t s z x a x p x c
    flags : [u8; 16],

    segreg : [u16;4], // ES, CS, SS, DS
    ip : u16,
    op : u8,

    unsupported_op : bool,
    jmp_taken : bool,
    hlt_suspend : bool,

    memory : Vec<u8>
}

impl Cpu {
    pub unsafe fn get_reg16(&self, index : usize) -> u16 {
        self.reg.reg16[index]
    }

    pub unsafe fn get_reg8(&self, index : usize) -> u8 {
        self.reg.reg8[REGMAP[index] as usize]
    }

    pub unsafe fn get_segreg(&self, index : usize) -> u16 {
        self.segreg[index]
    }

    pub unsafe fn set_segreg(&mut self, index : usize, value : u16) {
        self.segreg[index] = value;
    }

    pub unsafe fn set_reg16(&mut self, index : usize, value : u16) {
        self.reg.reg16[index] = value;
    }

    pub unsafe fn set_reg8(&mut self, index : usize, value : u8) {
        self.reg.reg8[REGMAP[index] as usize] = value;
    }

    pub fn set_of(&mut self, val : u8) {
        self.flags[4] = val;
    }

    pub fn get_of(&mut self) -> u8 {
        self.flags[4]
    }

    pub fn set_df(&mut self, val : u8) {
        self.flags[5] = val;
    }

    pub fn get_df(&mut self) -> u8 {
        self.flags[5]
    }

    pub fn set_if(&mut self, val : u8) {
        self.flags[6] = val;
    }

    pub fn get_if(&mut self) -> u8 {
        self.flags[6]
    }

    pub fn set_tf(&mut self, val : u8) {
        self.flags[7] = val;
    }

    pub fn get_tf(&mut self) -> u8 {
        self.flags[7]
    }

    pub fn set_sf(&mut self, val : u8) {
        self.flags[8] = val;
    }

    pub fn get_sf(&mut self) -> u8 {
        self.flags[8]
    }

    pub fn set_zf(&mut self, val : u8) {
        self.flags[9] = val;
    }

    pub fn get_zf(&mut self) -> u8 {
        self.flags[9]
    }

    pub fn set_af(&mut self, val : u8) {
        self.flags[11] = val;
    }

    pub fn get_af(&mut self) -> u8 {
        self.flags[11]
    }

    pub fn set_pf(&mut self, val : u8) {
        self.flags[13] = val;
    }

    pub fn get_pf(&mut self) -> u8 {
        self.flags[13]
    }

    pub fn set_cf(&mut self, val : u8) {
        self.flags[15] = val;
    }

    pub fn get_cf(&mut self) -> u8 {
        self.flags[15]
    }

    pub fn set_ip(&mut self, val : u16) {
        self.ip = val;
    }

    pub fn get_ip(&mut self) -> u16 {
        self.ip
    }

    pub fn inc_ip(&mut self, bytes : u16) {
        self.ip += bytes;
    }

    pub fn set_op(&mut self, val : u8) {
        self.op = val;
    }

    pub fn get_op(&mut self) -> u8 {
        self.op
    }

    pub fn set_unop(&mut self, val : bool) {
        self.unsupported_op = val;
    }

    pub fn get_unop(&mut self) -> bool {
        self.unsupported_op
    }

    pub fn set_memory(&mut self, index:usize, val : u8) {
        self.memory[index] = val;
    }

    pub fn get_memory(&mut self, index:usize) -> u8 {
        self.memory[index]
    }

    pub fn get_mem_buff(&mut self) -> &Vec<u8> {
        &self.memory
    }

    pub fn set_jmp_taken(&mut self, val : bool) {
        self.jmp_taken = val;
    }

    pub fn get_jmp_taken(&mut self) -> bool {
        self.jmp_taken
    }

    pub fn set_suspend(&mut self, val : bool) {
        self.hlt_suspend = val;
    }

    pub fn get_suspend(&mut self) -> bool {
        self.hlt_suspend
    }

}


unsafe fn reset(cpu:&mut Cpu,  data: Vec<u8>) {
    cpu.set_segreg(es!(), 0x00);
    cpu.set_segreg(cs!(), 0x00);
    cpu.set_segreg(ss!(), 0x00);
    cpu.set_segreg(ds!(), 0x00);

    cpu.ip = 0x00;
    cpu.set_reg16(sp!(), 0x100);

    cpu.memory.iter_mut().map(|x| *x = 0).count();

    for (dst, src) in cpu.memory.iter_mut().zip(&data[0..data.len()]) {
        *dst = *src;
    }
}

pub unsafe fn init(data : Vec<u8>) ->  Cpu {
    let mut cpu = Cpu {
        reg: Reg{reg16: [0,0,0,0,0,0,0,0]},
        flags:  [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
        segreg: [0,0,0,0],
        ip: 0,
        op: 0,

        hlt_suspend: false,
        unsupported_op : false,
        jmp_taken : false,

        memory: vec![0; ram_memory!()],
    };

    reset(&mut cpu,data);
    return cpu;
}