extern crate vm;


use vm::{Process, Instr, State};


/*
call function
jmp function_end

function:
    push_u8 0
    loop:
        push_u8 1
        add_u8
        copy_u8
        push_u8 5
        neq_u8,
        if_jmp loop
    loop_end:

    pop_u8
    ret
function_end:

halt
*/
static PROGRAM: [u8; 30] = [
    Instr::call as u8, Instr::type_int as u8, Instr::size_8 as u8, 8,
    Instr::jmp as u8, Instr::type_int as u8, Instr::size_8 as u8, 29,

    Instr::push_u8 as u8, Instr::type_int as u8, Instr::size_8 as u8, 0,

    // loop
    Instr::push_u8 as u8, Instr::type_int as u8, Instr::size_8 as u8, 1,
    Instr::add_u8 as u8,

    Instr::copy_u8 as u8,
    Instr::push_u8 as u8, Instr::type_int as u8, Instr::size_8 as u8, 3,
    Instr::neq_u8 as u8,
    Instr::if_jmp as u8, Instr::type_int as u8, Instr::size_8 as u8, 11,
    Instr::pop_u8 as u8,
    Instr::ret as u8,
    // loop end

    Instr::halt as u8,
];


#[test]
fn test_next() {
    let mut process = Process::new(&PROGRAM);
    assert_eq!(process.next_u8(), Instr::call as u8);
    assert_eq!(process.next_u8(), Instr::type_int as u8);
    assert_eq!(process.next_u8(), Instr::size_8 as u8);
    assert_eq!(process.next_u8(), 8);
}

#[test]
fn test_push_pop() {
    let mut process = Process::new(&PROGRAM);
    process.push_u8(1);
    process.push_u16(2);
    process.push_u32(3);
    process.push_u64(4);
    assert_eq!(process.pop_u64(), 4);
    assert_eq!(process.pop_u32(), 3);
    assert_eq!(process.pop_u16(), 2);
    assert_eq!(process.pop_u8(), 1);
}

#[test]
fn test_full_program() {
    let mut process = Process::new(&PROGRAM);
    vm::run(&mut process);
    assert_eq!(process.get_state(), State::Terminated);
}
