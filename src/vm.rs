use instrs::Instr;
use state::State;
use process::Process;


#[inline]
pub fn run<'a>(process: &mut Process<'a>) {

    process.set_state(State::Running);

    while process.get_state() != State::Terminated {
        if let Some(instr) = process.next() {
            evaluate_instr(process, Process::to_instr(instr));
        } else {
            process.set_state(State::Terminated);
        }
    }
}

#[inline]
fn evaluate_instr<'a>(process: &mut Process<'a>, instr: Instr) {
    match instr {
        Instr::nop => (),
        Instr::halt => process.halt(),

        // branching
        Instr::jmp => process.jmp(),
        Instr::if_jmp => process.if_jmp(),

        // function calls
        Instr::call => process.call(),
        Instr::ret => process.ret(),

        // push instructions
        Instr::push_u8 => {let v = process.read_u8(); process.push_u8(v)},
        Instr::push_u16 => {let v = process.read_u16(); process.push_u16(v)},
        Instr::push_u32 => {let v = process.read_u32(); process.push_u32(v)},
        Instr::push_u64 => {let v = process.read_u64(); process.push_u64(v)},

        Instr::push_i8 => {let v = process.read_u8(); process.push_u8(v)},
        Instr::push_i16 => {let v = process.read_u16(); process.push_u16(v)},
        Instr::push_i32 => {let v = process.read_u32(); process.push_u32(v)},
        Instr::push_i64 => {let v = process.read_u64(); process.push_u64(v)},

        Instr::push_f32 => {let v = process.read_u32(); process.push_u32(v)},
        Instr::push_f64 => {let v = process.read_u64(); process.push_u64(v)},

        // pop instructions
        Instr::pop_u8 => {process.pop_u8();},
        Instr::pop_u16 => {process.pop_u16();},
        Instr::pop_u32 => {process.pop_u32();},
        Instr::pop_u64 => {process.pop_u64();},

        Instr::pop_i8 => {process.pop_u8();},
        Instr::pop_i16 => {process.pop_u16();},
        Instr::pop_i32 => {process.pop_u32();},
        Instr::pop_i64 => {process.pop_u64();},

        Instr::pop_f32 => {process.pop_u32();},
        Instr::pop_f64 => {process.pop_u64();},

        // push instructions
        Instr::copy_u8 => process.copy_u8(),
        Instr::copy_u16 => process.copy_u16(),
        Instr::copy_u32 => process.copy_u32(),
        Instr::copy_u64 => process.copy_u64(),

        Instr::copy_i8 => process.copy_u8(),
        Instr::copy_i16 => process.copy_u16(),
        Instr::copy_i32 => process.copy_u32(),
        Instr::copy_i64 => process.copy_u64(),

        Instr::copy_f32 => process.copy_u32(),
        Instr::copy_f64 => process.copy_u64(),

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

        // save instructions
        Instr::save_u8 => process.save_u8(),
        Instr::save_u16 => process.save_u16(),
        Instr::save_u32 => process.save_u32(),
        Instr::save_u64 => process.save_u64(),

        Instr::save_i8 => process.save_u8(),
        Instr::save_i16 => process.save_u16(),
        Instr::save_i32 => process.save_u32(),
        Instr::save_i64 => process.save_u64(),

        Instr::save_f32 => process.save_u32(),
        Instr::save_f64 => process.save_u64(),

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

        // and instructions
        Instr::and_u8 => process.and_u8(),
        Instr::and_u16 => process.and_u16(),
        Instr::and_u32 => process.and_u32(),
        Instr::and_u64 => process.and_u64(),

        Instr::and_i8 => process.and_i8(),
        Instr::and_i16 => process.and_i16(),
        Instr::and_i32 => process.and_i32(),
        Instr::and_i64 => process.and_i64(),

        // or instructions
        Instr::or_u8 => process.or_u8(),
        Instr::or_u16 => process.or_u16(),
        Instr::or_u32 => process.or_u32(),
        Instr::or_u64 => process.or_u64(),

        Instr::or_i8 => process.or_i8(),
        Instr::or_i16 => process.or_i16(),
        Instr::or_i32 => process.or_i32(),
        Instr::or_i64 => process.or_i64(),

        // xor instructions
        Instr::xor_u8 => process.xor_u8(),
        Instr::xor_u16 => process.xor_u16(),
        Instr::xor_u32 => process.xor_u32(),
        Instr::xor_u64 => process.xor_u64(),

        Instr::xor_i8 => process.xor_i8(),
        Instr::xor_i16 => process.xor_i16(),
        Instr::xor_i32 => process.xor_i32(),
        Instr::xor_i64 => process.xor_i64(),

        // shl instructions
        Instr::shl_u8 => process.shl_u8(),
        Instr::shl_u16 => process.shl_u16(),
        Instr::shl_u32 => process.shl_u32(),
        Instr::shl_u64 => process.shl_u64(),

        Instr::shl_i8 => process.shl_i8(),
        Instr::shl_i16 => process.shl_i16(),
        Instr::shl_i32 => process.shl_i32(),
        Instr::shl_i64 => process.shl_i64(),

        // shr instructions
        Instr::shr_u8 => process.shr_u8(),
        Instr::shr_u16 => process.shr_u16(),
        Instr::shr_u32 => process.shr_u32(),
        Instr::shr_u64 => process.shr_u64(),

        Instr::shr_i8 => process.shr_i8(),
        Instr::shr_i16 => process.shr_i16(),
        Instr::shr_i32 => process.shr_i32(),
        Instr::shr_i64 => process.shr_i64(),

        // not instructions
        Instr::not_u8 => process.not_u8(),
        Instr::not_u16 => process.not_u16(),
        Instr::not_u32 => process.not_u32(),
        Instr::not_u64 => process.not_u64(),

        Instr::not_i8 => process.not_i8(),
        Instr::not_i16 => process.not_i16(),
        Instr::not_i32 => process.not_i32(),
        Instr::not_i64 => process.not_i64(),

        // neg instructions
        Instr::neg_u8 => process.neg_u8(),
        Instr::neg_u16 => process.neg_u16(),
        Instr::neg_u32 => process.neg_u32(),
        Instr::neg_u64 => process.neg_u64(),

        Instr::neg_i8 => process.neg_i8(),
        Instr::neg_i16 => process.neg_i16(),
        Instr::neg_i32 => process.neg_i32(),
        Instr::neg_i64 => process.neg_i64(),

        Instr::neg_f32 => process.neg_f32(),
        Instr::neg_f64 => process.neg_f64(),

        // lt instructions
        Instr::lt_u8 => process.lt_u8(),
        Instr::lt_u16 => process.lt_u16(),
        Instr::lt_u32 => process.lt_u32(),
        Instr::lt_u64 => process.lt_u64(),

        Instr::lt_i8 => process.lt_i8(),
        Instr::lt_i16 => process.lt_i16(),
        Instr::lt_i32 => process.lt_i32(),
        Instr::lt_i64 => process.lt_i64(),

        Instr::lt_f32 => process.lt_f32(),
        Instr::lt_f64 => process.lt_f64(),

        // gt instructions
        Instr::gt_u8 => process.gt_u8(),
        Instr::gt_u16 => process.gt_u16(),
        Instr::gt_u32 => process.gt_u32(),
        Instr::gt_u64 => process.gt_u64(),

        Instr::gt_i8 => process.gt_i8(),
        Instr::gt_i16 => process.gt_i16(),
        Instr::gt_i32 => process.gt_i32(),
        Instr::gt_i64 => process.gt_i64(),

        Instr::gt_f32 => process.gt_f32(),
        Instr::gt_f64 => process.gt_f64(),

        // lte instructions
        Instr::lte_u8 => process.lte_u8(),
        Instr::lte_u16 => process.lte_u16(),
        Instr::lte_u32 => process.lte_u32(),
        Instr::lte_u64 => process.lte_u64(),

        Instr::lte_i8 => process.lte_i8(),
        Instr::lte_i16 => process.lte_i16(),
        Instr::lte_i32 => process.lte_i32(),
        Instr::lte_i64 => process.lte_i64(),

        Instr::lte_f32 => process.lte_f32(),
        Instr::lte_f64 => process.lte_f64(),

        // gte instructions
        Instr::gte_u8 => process.gte_u8(),
        Instr::gte_u16 => process.gte_u16(),
        Instr::gte_u32 => process.gte_u32(),
        Instr::gte_u64 => process.gte_u64(),

        Instr::gte_i8 => process.gte_i8(),
        Instr::gte_i16 => process.gte_i16(),
        Instr::gte_i32 => process.gte_i32(),
        Instr::gte_i64 => process.gte_i64(),

        Instr::gte_f32 => process.gte_f32(),
        Instr::gte_f64 => process.gte_f64(),

        // eq instructions
        Instr::eq_u8 => process.eq_u8(),
        Instr::eq_u16 => process.eq_u16(),
        Instr::eq_u32 => process.eq_u32(),
        Instr::eq_u64 => process.eq_u64(),

        Instr::eq_i8 => process.eq_i8(),
        Instr::eq_i16 => process.eq_i16(),
        Instr::eq_i32 => process.eq_i32(),
        Instr::eq_i64 => process.eq_i64(),

        Instr::eq_f32 => process.eq_f32(),
        Instr::eq_f64 => process.eq_f64(),

        // not eq instructions
        Instr::neq_u8 => process.neq_u8(),
        Instr::neq_u16 => process.neq_u16(),
        Instr::neq_u32 => process.neq_u32(),
        Instr::neq_u64 => process.neq_u64(),

        Instr::neq_i8 => process.neq_i8(),
        Instr::neq_i16 => process.neq_i16(),
        Instr::neq_i32 => process.neq_i32(),
        Instr::neq_i64 => process.neq_i64(),

        Instr::neq_f32 => process.neq_f32(),
        Instr::neq_f64 => process.neq_f64(),

        _ => panic!("Invalid Instruction {:?}", instr),
    }
}
