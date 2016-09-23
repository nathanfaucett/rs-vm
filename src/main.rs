extern crate vm;


pub use vm::*;

/*
load_u8 %r8_0, 255

loop:
    sub_u8 %r8_0, 1, r8_0
    if_jmp %r8_0, loop

halt
*/
static PROGRAM: [u8; 24] = [
    Instr::load_u8 as u8, RegsId::r8_0 as u8, Instr::type_int as u8, 255,
    Instr::sub_u8 as u8, Instr::type_reg as u8, RegsId::r8_0 as u8, Instr::type_int as u8, 1, Instr::type_reg as u8, RegsId::r8_0 as u8,
    Instr::if_jmp as u8, Instr::type_reg as u8, RegsId::r8_0 as u8, Instr::type_int as u8, 0, 0, 0, 0, 0, 0, 0, 4,
    Instr::halt as u8,
];


fn main() {
    let mut process = Process::new(&PROGRAM);
    VirtualMachine::run(&mut process);
    println!("{:?}", process);
}
