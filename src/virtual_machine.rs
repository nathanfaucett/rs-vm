use instrs::Instr;
use state::State;
use process::Process;


pub struct VirtualMachine;


impl VirtualMachine {

    #[inline]
    pub fn run<'a>(process: &mut Process<'a>) {

        process.set_state(State::Running);

        while process.get_state() == State::Running {
            if let Some(instr) = process.next() {
                Self::evaluate(process, Process::to_instr(instr));
            } else {
                process.set_state(State::Terminated);
                break;
            }
        }
    }

    #[inline]
    fn evaluate(process: &mut Process, instr: Instr) {
        match instr {
            Instr::halt => process.halt(),
            Instr::nop => (),

            // branching
            Instr::jmp => process.jmp(),
            Instr::if_jmp => process.if_jmp(),

            // load instructions
            Instr::load_u8 => process.load_u8(),
            Instr::load_u16 => process.load_u16(),
            Instr::load_u32 => process.load_u32(),
            Instr::load_u64 => process.load_u64(),

            Instr::load_i8 => process.load_u8(),
            Instr::load_i16 => process.load_u16(),
            Instr::load_i32 => process.load_u32(),
            Instr::load_i64 => process.load_u64(),

            Instr::load_f32 => process.load_u32(),
            Instr::load_f64 => process.load_u64(),

            // push instructions
            Instr::push_u8 => {let v = process.next_u8(); process.push_u8(v)},
            Instr::push_u16 => {let v = process.next_u16(); process.push_u16(v)},
            Instr::push_u32 => {let v = process.next_u32(); process.push_u32(v)},
            Instr::push_u64 => {let v = process.next_u64(); process.push_u64(v)},

            Instr::push_i8 => {let v = process.next_u8(); process.push_u8(v)},
            Instr::push_i16 => {let v = process.next_u16(); process.push_u16(v)},
            Instr::push_i32 => {let v = process.next_u32(); process.push_u32(v)},
            Instr::push_i64 => {let v = process.next_u64(); process.push_u64(v)},

            Instr::push_f32 => {let v = process.next_u32(); process.push_u32(v)},
            Instr::push_f64 => {let v = process.next_u64(); process.push_u64(v)},

            // sub instructions
            Instr::sub_u8 => process.sub_u8(),


            _ => panic!("Invalid Instruction {:?}", instr),
        }
    }
}
