extern crate vm;


pub use vm::*;


/*
push_u8 5

loop1:
    push_u8 1
    sub_u8

    copy_u8
    push_u8 0
    eq_u8,
    if_jmp loop1
    pop_u8

spawn new_process
jmp new_process_end

new_process:
    push_u8 0

    loop2:
        push_u8 1
        add_u8

        copy_u8
        push_u8 5
        neq_u8,
        wait,
        if_jmp loop2
        pop_u8
        halt
    loop2_end:
new_process_end:

halt
*/
static PROGRAM: [u8; 59] = [
    Instr::push_u8 as u8, 5,

    // loop1
    Instr::push_u8 as u8, 1,
    Instr::sub_u8 as u8,

    Instr::copy_u8 as u8,
    Instr::push_u8 as u8, 0,
    Instr::neq_u8 as u8,
    Instr::if_jmp as u8, 0, 0, 0, 0, 0, 0, 0, 2,
    Instr::pop_u8 as u8,
    // loop1 end


    Instr::spawn as u8, 0, 0, 0, 0, 0, 0, 0, 37,
    Instr::jmp as u8, 0, 0, 0, 0, 0, 0, 0, 58,

    Instr::push_u8 as u8, 0,

    // loop2
    Instr::push_u8 as u8, 1,
    Instr::add_u8 as u8,

    Instr::copy_u8 as u8,
    Instr::push_u8 as u8, 5,
    Instr::neq_u8 as u8,
    Instr::wait as u8,
    Instr::if_jmp as u8, 0, 0, 0, 0, 0, 0, 0, 39,
    Instr::pop_u8 as u8,
    Instr::halt as u8,
    // loop2 end


    Instr::halt as u8,
];


fn main() {
    let mut vm = VirtualMachine::new(&PROGRAM);
    vm.run();
}
