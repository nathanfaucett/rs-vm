use core::mem;

use vector::Vector;
use collection::Collection;
use stack::Stack;
use remove::Remove;

use instrs::Instr;
use state::State;
use process::Process;


#[derive(Debug)]
pub struct VirtualMachine<'a> {
    id: usize,
    process: Process<'a>,
    processes: Vector<Process<'a>>,
}

impl<'a> VirtualMachine<'a> {

    pub fn new(program: &'a [u8]) -> Self {
        VirtualMachine {
            id: 0,
            process: Process::new(0, program),
            processes: Vector::new(),
        }
    }

    fn next_id(&mut self) -> usize {
        self.id += 1;
        self.id
    }

    #[inline]
    pub fn run(&mut self) {
        loop {
            if self.process.get_state() != State::Terminated {
                if let Some(instr) = self.process.next() {
                    self.evaluate_instr(Process::to_instr(instr));
                } else {
                    self.process.set_state(State::Terminated);
                }
            } else {
                self.switch();

                if self.process.get_state() == State::Terminated && self.processes.len() == 0 {
                    break;
                }
            }
        }
    }

    #[inline]
    pub fn spawn(&mut self, target: usize) {
        let id = self.next_id();
        let process = self.process.spawn(id, target);

        self.process.set_state(State::Waiting);

        let old_process = mem::replace(&mut self.process, process);
        self.processes.push(old_process);
    }

    #[inline]
    pub fn switch(&mut self) {
        let index = 0;
        let mut new_process_index = None;

        self.processes.retain(|process| process.get_state() != State::Terminated);

        for process in self.processes.iter_mut() {
            match process.get_state() {
                State::Waiting => {
                    new_process_index = Some(index);
                    break;
                },
                state => panic!("Invalid process State {:?}", state),
            }
            index += 1;
        }

        if let Some(index) = new_process_index {
            let mut new_process = self.processes.remove(&index);
            new_process.set_state(State::Running);
            let old_process = mem::replace(&mut self.process, new_process);
            self.processes.push(old_process);
        }
    }

    #[inline]
    pub fn evaluate_instr(&mut self, instr: Instr) {
        match instr {
            Instr::nop => (),
            Instr::halt => self.process.halt(),
            Instr::wait => self.process.wait(),
            Instr::spawn => {let v = self.process.next_usize(); self.spawn(v)},

            // branching
            Instr::jmp => self.process.jmp(),
            Instr::if_jmp => self.process.if_jmp(),

            // function calls
            Instr::call => self.process.call(),
            Instr::ret => self.process.ret(),

            // push instructions
            Instr::push_u8 => {let v = self.process.next_u8(); self.process.push_u8(v)},
            Instr::push_u16 => {let v = self.process.next_u16(); self.process.push_u16(v)},
            Instr::push_u32 => {let v = self.process.next_u32(); self.process.push_u32(v)},
            Instr::push_u64 => {let v = self.process.next_u64(); self.process.push_u64(v)},

            Instr::push_i8 => {let v = self.process.next_u8(); self.process.push_u8(v)},
            Instr::push_i16 => {let v = self.process.next_u16(); self.process.push_u16(v)},
            Instr::push_i32 => {let v = self.process.next_u32(); self.process.push_u32(v)},
            Instr::push_i64 => {let v = self.process.next_u64(); self.process.push_u64(v)},

            Instr::push_f32 => {let v = self.process.next_u32(); self.process.push_u32(v)},
            Instr::push_f64 => {let v = self.process.next_u64(); self.process.push_u64(v)},

            // pop instructions
            Instr::pop_u8 => {self.process.pop_u8();},
            Instr::pop_u16 => {self.process.pop_u16();},
            Instr::pop_u32 => {self.process.pop_u32();},
            Instr::pop_u64 => {self.process.pop_u64();},

            Instr::pop_i8 => {self.process.pop_u8();},
            Instr::pop_i16 => {self.process.pop_u16();},
            Instr::pop_i32 => {self.process.pop_u32();},
            Instr::pop_i64 => {self.process.pop_u64();},

            Instr::pop_f32 => {self.process.pop_u32();},
            Instr::pop_f64 => {self.process.pop_u64();},

            // push instructions
            Instr::copy_u8 => self.process.copy_u8(),
            Instr::copy_u16 => self.process.copy_u16(),
            Instr::copy_u32 => self.process.copy_u32(),
            Instr::copy_u64 => self.process.copy_u64(),

            Instr::copy_i8 => self.process.copy_u8(),
            Instr::copy_i16 => self.process.copy_u16(),
            Instr::copy_i32 => self.process.copy_u32(),
            Instr::copy_i64 => self.process.copy_u64(),

            Instr::copy_f32 => self.process.copy_u32(),
            Instr::copy_f64 => self.process.copy_u64(),

            // add instructions
            Instr::add_u8 => self.process.add_u8(),
            Instr::add_u16 => self.process.add_u16(),
            Instr::add_u32 => self.process.add_u32(),
            Instr::add_u64 => self.process.add_u64(),

            Instr::add_i8 => self.process.add_i8(),
            Instr::add_i16 => self.process.add_i16(),
            Instr::add_i32 => self.process.add_i32(),
            Instr::add_i64 => self.process.add_i64(),

            Instr::add_f32 => self.process.add_f32(),
            Instr::add_f64 => self.process.add_f64(),

            // sub instructions
            Instr::sub_u8 => self.process.sub_u8(),
            Instr::sub_u16 => self.process.sub_u16(),
            Instr::sub_u32 => self.process.sub_u32(),
            Instr::sub_u64 => self.process.sub_u64(),

            Instr::sub_i8 => self.process.sub_i8(),
            Instr::sub_i16 => self.process.sub_i16(),
            Instr::sub_i32 => self.process.sub_i32(),
            Instr::sub_i64 => self.process.sub_i64(),

            Instr::sub_f32 => self.process.sub_f32(),
            Instr::sub_f64 => self.process.sub_f64(),

            // mul instructions
            Instr::mul_u8 => self.process.mul_u8(),
            Instr::mul_u16 => self.process.mul_u16(),
            Instr::mul_u32 => self.process.mul_u32(),
            Instr::mul_u64 => self.process.mul_u64(),

            Instr::mul_i8 => self.process.mul_i8(),
            Instr::mul_i16 => self.process.mul_i16(),
            Instr::mul_i32 => self.process.mul_i32(),
            Instr::mul_i64 => self.process.mul_i64(),

            Instr::mul_f32 => self.process.mul_f32(),
            Instr::mul_f64 => self.process.mul_f64(),

            // div instructions
            Instr::div_u8 => self.process.div_u8(),
            Instr::div_u16 => self.process.div_u16(),
            Instr::div_u32 => self.process.div_u32(),
            Instr::div_u64 => self.process.div_u64(),

            Instr::div_i8 => self.process.div_i8(),
            Instr::div_i16 => self.process.div_i16(),
            Instr::div_i32 => self.process.div_i32(),
            Instr::div_i64 => self.process.div_i64(),

            Instr::div_f32 => self.process.div_f32(),
            Instr::div_f64 => self.process.div_f64(),

            // rem instructions
            Instr::rem_u8 => self.process.rem_u8(),
            Instr::rem_u16 => self.process.rem_u16(),
            Instr::rem_u32 => self.process.rem_u32(),
            Instr::rem_u64 => self.process.rem_u64(),

            Instr::rem_i8 => self.process.rem_i8(),
            Instr::rem_i16 => self.process.rem_i16(),
            Instr::rem_i32 => self.process.rem_i32(),
            Instr::rem_i64 => self.process.rem_i64(),

            Instr::rem_f32 => self.process.rem_f32(),
            Instr::rem_f64 => self.process.rem_f64(),

            // eq instructions
            Instr::eq_u8 => self.process.eq_u8(),
            Instr::eq_u16 => self.process.eq_u16(),
            Instr::eq_u32 => self.process.eq_u32(),
            Instr::eq_u64 => self.process.eq_u64(),

            Instr::eq_i8 => self.process.eq_i8(),
            Instr::eq_i16 => self.process.eq_i16(),
            Instr::eq_i32 => self.process.eq_i32(),
            Instr::eq_i64 => self.process.eq_i64(),

            Instr::eq_f32 => self.process.eq_f32(),
            Instr::eq_f64 => self.process.eq_f64(),

            // not eq instructions
            Instr::neq_u8 => self.process.neq_u8(),
            Instr::neq_u16 => self.process.neq_u16(),
            Instr::neq_u32 => self.process.neq_u32(),
            Instr::neq_u64 => self.process.neq_u64(),

            Instr::neq_i8 => self.process.neq_i8(),
            Instr::neq_i16 => self.process.neq_i16(),
            Instr::neq_i32 => self.process.neq_i32(),
            Instr::neq_i64 => self.process.neq_i64(),

            Instr::neq_f32 => self.process.neq_f32(),
            Instr::neq_f64 => self.process.neq_f64(),

            _ => panic!("Invalid Instruction {:?}", instr),
        }
    }
}
