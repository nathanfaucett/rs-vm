extern crate vm;


use vm::Process;


static PROGRAM: [u8; 8] = [0, 0, 0, 0, 0, 0, 0, 1];


#[test]
fn test_next() {
    let mut process = Process::new(&PROGRAM);
    assert_eq!(process.next_usize(), 1usize);
}

#[test]
fn test_push_pop() {
    let mut process = Process::new(&PROGRAM);
    process.push_u64(1);
    assert_eq!(process.pop_u64(), 1);
}
