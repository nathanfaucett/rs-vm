extern crate vm;


pub use vm::*;


/*
push_u8 255

loop:
    push_u8 1
    sub_u8

    copy_u8
    push_u8 0
    eq_u8,
    if_jmp loop

loop:
    push_u8 1
    add_u8

    copy_u8
    push_u8 255
    eq_u8,
    if_jmp loop

halt
*/
static PROGRAM: [u8; 35] = [
    Instr::push_u8 as u8, 255,

    Instr::push_u8 as u8, 1,
    Instr::sub_u8 as u8,

    Instr::copy_u8 as u8,
    Instr::push_u8 as u8, 0,
    Instr::neq_u8 as u8,
    Instr::if_jmp as u8, 0, 0, 0, 0, 0, 0, 0, 2,


    Instr::push_u8 as u8, 1,
    Instr::add_u8 as u8,

    Instr::copy_u8 as u8,
    Instr::push_u8 as u8, 255,
    Instr::eq_u8 as u8,
    Instr::if_jmp as u8, 0, 0, 0, 0, 0, 0, 0, 16,


    Instr::halt as u8,
];


fn main() {
    let mut process = Process::new(&PROGRAM);
    VirtualMachine::run(&mut process);
    println!("{:?}", process);
}
