

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum RegsId {
    // 8 bit registers
    r8_0 = 0, r8_1, r8_2, r8_3, r8_4, r8_5, r8_6, r8_7,

    // 16 bit registers
    r16_0, r16_1, r16_2, r16_3, r16_4, r16_5, r16_6, r16_7,

    // 32 bit registers
    r32_0, r32_1, r32_2, r32_3, r32_4, r32_5, r32_6, r32_7,

    // 64 bit registers
    r64_0, r64_1, r64_2, r64_3, r64_4, r64_5, r64_6, r64_7,
}
