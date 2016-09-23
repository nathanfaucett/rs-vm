

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Regs {
    pub u8: [u8; 8],
    pub u16: [u16; 8],
    pub u32: [u32; 8],
    pub u64: [u64; 8],
}

macro_rules! create_get_size {
    ($n: ident, $s: ident) => (
        pub fn $n(&self, i: usize) -> $s {
            if i < 8 {
                self.u8[i] as $s
            } else if i < 16 {
                self.u16[i - 8] as $s
            } else if i < 24 {
                self.u32[i - 16] as $s
            } else {
                self.u64[i - 24] as $s
            }
        }
    )
}

macro_rules! create_set_size {
    ($n: ident, $s: ident) => (
        pub fn $n(&mut self, i: usize, v: $s) {
            if i < 8 {
                self.u8[i] = v as u8;
            } else if i < 16 {
                self.u16[i - 8] = v as u16;
            } else if i < 24 {
                self.u32[i - 16] = v as u32;
            } else {
                self.u64[i - 24] = v as u64;
            }
        }
    )
}

impl Regs {
    pub fn new() -> Self {
        Regs {
            u8: [0; 8],
            u16: [0; 8],
            u32: [0; 8],
            u64: [0; 8],
        }
    }

    create_get_size!(get_u8, u8);
    create_get_size!(get_u16, u16);
    create_get_size!(get_u32, u32);
    create_get_size!(get_u64, u64);

    create_set_size!(set_u8, u8);
    create_set_size!(set_u16, u16);
    create_set_size!(set_u32, u32);
    create_set_size!(set_u64, u64);
}
