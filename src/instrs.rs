/*
instruction general format

instr type ... args
*/

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Instr {

    nop = 0, halt,

    // branching
    // (value: u8, location: usize)
    if_jmp,
    // (location: usize)
    jmp,

    // types
    // instructs the vm to interpret the next bytes as register, pointer, or intermediate value
    type_reg, type_ptr, type_int,

    // registers
    // (reg: ident, value)
    load_u8, load_u16, load_u32, load_u64,
    load_i8, load_i16, load_i32, load_i64, load_f32, load_f64,

    // stack
    // (value)
    push_u8, push_u16, push_u32, push_u64,
    push_i8, push_i16, push_i32, push_i64, push_f32, push_f64,

    pop_u8, pop_u16, pop_u32, pop_u64,
    pop_i8, pop_i16, pop_i32, pop_i64, pop_f32, pop_f64,

    //binary operations (a, b, out)
    add_u8, add_u16, add_u32, add_u64,
    add_i8, add_i16, add_i32, add_i64, add_f32, add_f64,

    sub_u8, sub_u16, sub_u32, sub_u64,
    sub_i8, sub_i16, sub_i32, sub_i64, sub_f32, sub_f64,

    mul_u8, mul_u16, mul_u32, mul_u64,
    mul_i8, mul_i16, mul_i32, mul_i64, mul_f32, mul_f64,

    div_u8, div_u16, div_u32, div_u64,
    div_i8, div_i16, div_i32, div_i64, div_f32, div_f64,

    rem_u8, rem_u16, rem_u32, rem_u64,
    rem_i8, rem_i16, rem_i32, rem_i64, rem_f32, rem_f64,

    // bitwise operations (a, b, out)
    and_u8, and_u16, and_u32, and_u64,
    and_i8, and_i16, and_i32, and_i64,

    or_u8, or_u16, or_u32, or_u64,
    or_i8, or_i16, or_i32, or_i64,

    xor_u8, xor_u16, xor_u32, xor_u64,
    xor_i8, xor_i16, xor_i32, xor_i64,

    shl_u8, shl_u16, shl_u32, shl_u64,
    shl_i8, shl_i16, shl_i32, shl_i64,

    shr_u8, shr_u16, shr_u32, shr_u64,
    shr_i8, shr_i16, shr_i32, shr_i64,

    // single operators (value, out)
    neg_u8, neg_u16, neg_u32, neg_u64,
    neg_i8, neg_i16, neg_i32, neg_i64,

    not_u8, not_u16, not_u32, not_u64,
    not_i8, not_i16, not_i32, not_i64,

    // comparison operators (a, b, out: u8)
    lt_u8, lt_u16, lt_u32, lt_u64,
    lt_i8, lt_i16, lt_i32, lt_i64, lt_f32, lt_f64,

    lte_u8, lte_u16, lte_u32, lte_u64,
    lte_i8, lte_i16, lte_i32, lte_i64, lte_f32, lte_f64,

    gt_u8, gt_u16, gt_u32, gt_u64,
    gt_i8, gt_i16, gt_i32, gt_i64, gt_f32, gt_f64,

    gte_u8, gte_u16, gte_u32, gte_u64,
    gte_i8, gte_i16, gte_i32, gte_i64, gte_f32, gte_f64,

    eq_u8, eq_u16, eq_u32, eq_u64,
    eq_i8, eq_i16, eq_i32, eq_i64, eq_f32, eq_f64,
}
