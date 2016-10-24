extern crate vm;


pub use vm::*;


/*
push_u8 255

loop1:
    push_u8 1
    sub_u8

    copy_u8
    push_u8 0
    eq_u8,
    if_jmp loop1
    pop_u8

push_u8 0

loop2:
    push_u8 1
    add_u8

    copy_u8
    push_u8 255
    neq_u8,
    if_jmp loop2
    pop_u8

halt
*/
static PROGRAM: [u8; 39] = [
    Instr::push_u8 as u8, 255,

    Instr::push_u8 as u8, 1,
    Instr::sub_u8 as u8,

    Instr::copy_u8 as u8,
    Instr::push_u8 as u8, 0,
    Instr::neq_u8 as u8,
    Instr::if_jmp as u8, 0, 0, 0, 0, 0, 0, 0, 2,
    Instr::pop_u8 as u8,


    Instr::push_u8 as u8, 0,

    Instr::push_u8 as u8, 1,
    Instr::add_u8 as u8,

    Instr::copy_u8 as u8,
    Instr::push_u8 as u8, 255,
    Instr::neq_u8 as u8,
    Instr::if_jmp as u8, 0, 0, 0, 0, 0, 0, 0, 20,
    Instr::pop_u8 as u8,


    Instr::halt as u8,
];


fn main() {
    let mut process = Process::new(&PROGRAM);
    VirtualMachine::run(&mut process);
    println!("{:?}", process);
}
