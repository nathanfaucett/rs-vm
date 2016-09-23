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

            // pop instructions
            Instr::pop_u8 => {let v = process.pop_u8(); process.write_u8(v)},
            Instr::pop_u16 => {let v = process.pop_u16(); process.write_u16(v)},
            Instr::pop_u32 => {let v = process.pop_u32(); process.write_u32(v)},
            Instr::pop_u64 => {let v = process.pop_u64(); process.write_u64(v)},

            Instr::pop_i8 => {let v = process.pop_u8(); process.write_u8(v)},
            Instr::pop_i16 => {let v = process.pop_u16(); process.write_u16(v)},
            Instr::pop_i32 => {let v = process.pop_u32(); process.write_u32(v)},
            Instr::pop_i64 => {let v = process.pop_u64(); process.write_u64(v)},

            Instr::pop_f32 => {let v = process.pop_u32(); process.write_u32(v)},
            Instr::pop_f64 => {let v = process.pop_u64(); process.write_u64(v)},

            // add instructions
            Instr::add_u8 => process.add_u8(),
            Instr::add_u16 => process.add_u16(),
            Instr::add_u32 => process.add_u32(),
            Instr::add_u64 => process.add_u64(),

            Instr::add_i8 => process.add_i8(),
            Instr::add_i16 => process.add_i16(),
            Instr::add_i32 => process.add_i32(),
            Instr::add_i64 => process.add_i64(),

            Instr::add_f32 => process.add_f32(),
            Instr::add_f64 => process.add_f64(),

            // sub instructions
            Instr::sub_u8 => process.sub_u8(),
            Instr::sub_u16 => process.sub_u16(),
            Instr::sub_u32 => process.sub_u32(),
            Instr::sub_u64 => process.sub_u64(),

            Instr::sub_i8 => process.sub_i8(),
            Instr::sub_i16 => process.sub_i16(),
            Instr::sub_i32 => process.sub_i32(),
            Instr::sub_i64 => process.sub_i64(),

            Instr::sub_f32 => process.sub_f32(),
            Instr::sub_f64 => process.sub_f64(),

            // mul instructions
            Instr::mul_u8 => process.mul_u8(),
            Instr::mul_u16 => process.mul_u16(),
            Instr::mul_u32 => process.mul_u32(),
            Instr::mul_u64 => process.mul_u64(),

            Instr::mul_i8 => process.mul_i8(),
            Instr::mul_i16 => process.mul_i16(),
            Instr::mul_i32 => process.mul_i32(),
            Instr::mul_i64 => process.mul_i64(),

            Instr::mul_f32 => process.mul_f32(),
            Instr::mul_f64 => process.mul_f64(),

            // div instructions
            Instr::div_u8 => process.div_u8(),
            Instr::div_u16 => process.div_u16(),
            Instr::div_u32 => process.div_u32(),
            Instr::div_u64 => process.div_u64(),

            Instr::div_i8 => process.div_i8(),
            Instr::div_i16 => process.div_i16(),
            Instr::div_i32 => process.div_i32(),
            Instr::div_i64 => process.div_i64(),

            Instr::div_f32 => process.div_f32(),
            Instr::div_f64 => process.div_f64(),

            // rem instructions
            Instr::rem_u8 => process.rem_u8(),
            Instr::rem_u16 => process.rem_u16(),
            Instr::rem_u32 => process.rem_u32(),
            Instr::rem_u64 => process.rem_u64(),

            Instr::rem_i8 => process.rem_i8(),
            Instr::rem_i16 => process.rem_i16(),
            Instr::rem_i32 => process.rem_i32(),
            Instr::rem_i64 => process.rem_i64(),

            Instr::rem_f32 => process.rem_f32(),
            Instr::rem_f64 => process.rem_f64(),


            _ => panic!("Invalid Instruction {:?}", instr),
        }
    }
}
