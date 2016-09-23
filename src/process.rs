use core::mem;
use collections::vec::Vec;

use instrs::Instr;
use regs::Regs;
use state::State;


#[derive(Debug)]
pub struct Process<'a> {
    state: State,

    program_counter: usize,
    program: &'a [u8],

    regs: Regs,
    stack: Vec<u8>,
}

impl<'a> Process<'a> {

    #[inline]
    pub fn new(program: &'a [u8]) -> Self {
        Process {
            state: State::New,

            program_counter: 0,
            program: program,

            regs: Regs::new(),
            stack: Vec::new(),
        }
    }

    #[inline]
    pub fn get_state(&self) -> State {self.state}
    #[inline]
    pub fn set_state(&mut self, state: State) {
        self.state = state;
    }

    #[inline]
    pub fn is_running(&self) -> bool {self.state == State::Running}
    #[inline]
    pub fn to_instr(instruction: u8) -> Instr {unsafe {mem::transmute(instruction)}}

    #[inline]
    pub fn peek(&mut self, offset: usize) -> Option<u8> {
        let pc = self.program_counter + offset;

        if pc < self.program.len() {
            let data = self.program[pc];
            Some(data)
        } else {
            None
        }
    }
    #[inline]
    pub fn skip(&mut self, count: usize) {
        let pc = self.program_counter + count;

        if pc < self.program.len() {
            self.program_counter = pc;
        } else {
            panic!("Cannot skip past end of program!");
        }
    }
    #[inline]
    pub fn next(&mut self) -> Option<u8> {
        if self.program_counter < self.program.len() {
            let data = self.program[self.program_counter];
            self.program_counter += 1;
            Some(data)
        } else {
            None
        }
    }
    #[inline(always)]
    pub fn next_expect(&mut self) -> u8 {
        self.next().expect("Unexpected end of program")
    }

    #[inline(always)]
    pub fn to_u16(b0: u8, b1: u8) -> u16 {
        (b0 as u16) << 8 |
        (b1 as u16)
    }
    #[inline(always)]
    pub fn to_u32(b0: u8, b1: u8, b2: u8, b3: u8) -> u32 {
        (b0 as u32) << 24 |
        (b1 as u32) << 16 |
        (b2 as u32) << 8 |
        (b3 as u32)
    }
    #[inline(always)]
    pub fn to_u64(b0: u8, b1: u8, b2: u8, b3: u8, b4: u8, b5: u8, b6: u8, b7: u8) -> u64 {
        (b0 as u64) << 56 |
        (b1 as u64) << 48 |
        (b2 as u64) << 40 |
        (b3 as u64) << 32 |
        (b4 as u64) << 24 |
        (b5 as u64) << 16 |
        (b6 as u64) << 8 |
        (b7 as u64)
    }

    #[inline]
    pub fn next_u8(&mut self) -> u8 {
        self.next_expect()
    }
    #[inline]
    pub fn next_u16(&mut self) -> u16 {
        Self::to_u16(
            self.next_expect(),
            self.next_expect()
        )
    }
    #[inline]
    pub fn next_u32(&mut self) -> u32 {
        Self::to_u32(
            self.next_expect(),
            self.next_expect(),
            self.next_expect(),
            self.next_expect()
        )
    }
    #[inline]
    pub fn next_u64(&mut self) -> u64 {
        Self::to_u64(
            self.next_expect(),
            self.next_expect(),
            self.next_expect(),
            self.next_expect(),
            self.next_expect(),
            self.next_expect(),
            self.next_expect(),
            self.next_expect()
        )
    }

    #[cfg(target_pointer_width = "32")]
    #[inline]
    pub fn next_usize(&mut self) -> usize {self.next_u32() as usize}
    #[cfg(target_pointer_width = "64")]
    #[inline]
    pub fn next_usize(&mut self) -> usize {self.next_u64() as usize}

    #[inline]
    pub fn next_i8(&mut self) -> i8 {self.next_u8() as i8}
    #[inline]
    pub fn next_i16(&mut self) -> i16 {self.next_u16() as i16}
    #[inline]
    pub fn next_i32(&mut self) -> i32 {self.next_u32() as i32}
    #[inline]
    pub fn next_i64(&mut self) -> i64 {self.next_u64() as i64}

    #[cfg(target_pointer_width = "32")]
    #[inline]
    pub fn next_isize(&mut self) -> usize {self.next_i32() as usize}
    #[cfg(target_pointer_width = "64")]
    #[inline]
    pub fn next_isize(&mut self) -> usize {self.next_i64() as usize}

    #[inline]
    pub fn next_f32(&mut self) -> f32 {self.next_u32() as f32}
    #[inline]
    pub fn next_f64(&mut self) -> f64 {self.next_u64() as f64}

    #[inline]
    pub fn read_u8(&mut self) -> u8 {
        let typ = Self::to_instr(self.next_u8());

        match typ {
            Instr::type_int => self.next_u8(),
            Instr::type_ptr => unsafe {*(self.next_usize() as *const u8)},
            Instr::type_reg => {
                let reg_index = self.next_u8();
                self.regs.get_u8(reg_index as usize)
            },
            instr => panic!("Invalid type {:?} given, trying to read 8 bits ", instr),
        }
    }
    #[inline]
    pub fn read_u16(&mut self) -> u16 {
        let typ = Self::to_instr(self.next_u8());

        match typ {
            Instr::type_int => self.next_u16(),
            Instr::type_ptr => unsafe {*(self.next_usize() as *const u16)},
            Instr::type_reg => {
                let reg_index = self.next_u8();
                self.regs.get_u16(reg_index as usize)
            },
            instr => panic!("Invalid type {:?} given, trying to read 16 bits", instr),
        }
    }
    #[inline]
    pub fn read_u32(&mut self) -> u32 {
        let typ = Self::to_instr(self.next_u8());

        match typ {
            Instr::type_int => self.next_u32(),
            Instr::type_ptr => unsafe {*(self.next_usize() as *const u32)},
            Instr::type_reg => {
                let reg_index = self.next_u8();
                self.regs.get_u32(reg_index as usize)
            },
            instr => panic!("Invalid type {:?} given, trying to read 32 bits", instr),
        }
    }
    #[inline]
    pub fn read_u64(&mut self) -> u64 {
        let typ = Self::to_instr(self.next_u8());

        match typ {
            Instr::type_int => self.next_u64(),
            Instr::type_ptr => unsafe {*(self.next_usize() as *const u64)},
            Instr::type_reg => {
                let reg_index = self.next_u8();
                self.regs.get_u64(reg_index as usize)
            },
            instr => panic!("Invalid type {:?} given, trying to read 64 bits", instr),
        }
    }
    #[cfg(target_pointer_width = "32")]
    #[inline]
    pub fn read_usize(&mut self) -> usize {self.read_u32() as usize}
    #[cfg(target_pointer_width = "64")]
    #[inline]
    pub fn read_usize(&mut self) -> usize {self.read_u64() as usize}

    #[inline]
    pub fn write_u8(&mut self, value: u8) {
        let typ = Self::to_instr(self.next_u8());

        match typ {
            Instr::type_ptr => unsafe {
                *(self.next_usize() as *mut u8) = value;
            },
            Instr::type_reg => {
                let reg_index = self.next_u8();
                self.regs.set_u8(reg_index as usize, value);
            },
            instr => panic!("Invalid write type {:?} given, trying to write 8 bits ", instr),
        }
    }
    #[inline]
    pub fn write_u16(&mut self, value: u16) {
        let typ = Self::to_instr(self.next_u8());

        match typ {
            Instr::type_ptr => unsafe {
                *(self.next_usize() as *mut u16) = value;
            },
            Instr::type_reg => {
                let reg_index = self.next_u16();
                self.regs.set_u16(reg_index as usize, value);
            },
            instr => panic!("Invalid write type {:?} given, trying to write 16 bits ", instr),
        }
    }
    #[inline]
    pub fn write_u32(&mut self, value: u32) {
        let typ = Self::to_instr(self.next_u8());

        match typ {
            Instr::type_ptr => unsafe {
                *(self.next_usize() as *mut u32) = value;
            },
            Instr::type_reg => {
                let reg_index = self.next_u32();
                self.regs.set_u32(reg_index as usize, value);
            },
            instr => panic!("Invalid write type {:?} given, trying to write 32 bits ", instr),
        }
    }
    #[inline]
    pub fn write_u64(&mut self, value: u64) {
        let typ = Self::to_instr(self.next_u8());

        match typ {
            Instr::type_ptr => unsafe {
                *(self.next_usize() as *mut u64) = value;
            },
            Instr::type_reg => {
                let reg_index = self.next_u64();
                self.regs.set_u64(reg_index as usize, value);
            },
            instr => panic!("Invalid write type {:?} given, trying to write 64 bits ", instr),
        }
    }

    #[inline]
    pub fn halt(&mut self) {
        self.state = State::Terminated;
    }

    #[inline]
    pub fn jmp(&mut self) {
        let index = self.read_usize();
        self.program_counter = index;
    }
    #[inline]
    pub fn if_jmp(&mut self) {
        let value = self.read_u8();

        if value == 0 {
            // skip type instruction and 8 bits for jump to value
            self.skip(9);
        } else {
            let index = self.read_usize();
            self.program_counter = index;
        }
    }

    #[inline]
    pub fn load_u8(&mut self) {
        let index = self.next_u8() as usize;
        let value = self.read_u8();
        self.regs.set_u8(index, value);
    }
    #[inline]
    pub fn load_u16(&mut self) {
        let index = self.next_u8() as usize;
        let value = self.read_u16();
        self.regs.set_u16(index, value);
    }
    #[inline]
    pub fn load_u32(&mut self) {
        let index = self.next_u8() as usize;
        let value = self.read_u32();
        self.regs.set_u32(index, value);
    }
    #[inline]
    pub fn load_u64(&mut self) {
        let index = self.next_u8() as usize;
        let value = self.read_u64();
        self.regs.set_u64(index, value);
    }

    #[inline]
    pub fn pop_u8(&mut self) -> u8 {
        self.stack.pop().expect("Unexpected end of stack")
    }
    #[inline]
    pub fn pop_u16(&mut self) -> u16 {
        let b0 = self.pop_u8();
        let b1 = self.pop_u8();
        Self::to_u16(b1, b0)
    }
    #[inline]
    pub fn pop_u32(&mut self) -> u32 {
        let b0 = self.pop_u8();
        let b1 = self.pop_u8();
        let b2 = self.pop_u8();
        let b3 = self.pop_u8();
        Self::to_u32(b3, b2, b1, b0)
    }
    #[inline]
    pub fn pop_u64(&mut self) -> u64 {
        let b0 = self.pop_u8();
        let b1 = self.pop_u8();
        let b2 = self.pop_u8();
        let b3 = self.pop_u8();
        let b4 = self.pop_u8();
        let b5 = self.pop_u8();
        let b6 = self.pop_u8();
        let b7 = self.pop_u8();
        Self::to_u64(b7, b6, b5, b4, b3, b2, b1, b0)
    }

    #[inline]
    pub fn push_u8(&mut self, value: u8) {
        let ref mut stack = self.stack;
        stack.push(value);
    }
    #[inline]
    pub fn push_u16(&mut self, value: u16) {
        let ref mut stack = self.stack;
        stack.push((value >> 8) as u8);
        stack.push(value as u8);
    }
    #[inline]
    pub fn push_u32(&mut self, value: u32) {
        let ref mut stack = self.stack;
        stack.push((value >> 24) as u8);
        stack.push((value >> 16) as u8);
        stack.push((value >> 8) as u8);
        stack.push(value as u8);
    }
    #[inline]
    pub fn push_u64(&mut self, value: u64) {
        let ref mut stack = self.stack;
        stack.push((value >> 56) as u8);
        stack.push((value >> 48) as u8);
        stack.push((value >> 40) as u8);
        stack.push((value >> 32) as u8);
        stack.push((value >> 24) as u8);
        stack.push((value >> 16) as u8);
        stack.push((value >> 8) as u8);
        stack.push(value as u8);
    }

    #[inline]
    pub fn add_u8(&mut self) {
        let a = self.read_u8();
        let b = self.read_u8();
        self.write_u8(a + b);
    }
    #[inline]
    pub fn add_u16(&mut self) {
        let a = self.read_u16();
        let b = self.read_u16();
        self.write_u16(a + b);
    }
    #[inline]
    pub fn add_u32(&mut self) {
        let a = self.read_u32();
        let b = self.read_u32();
        self.write_u32(a + b);
    }
    #[inline]
    pub fn add_u64(&mut self) {
        let a = self.read_u64();
        let b = self.read_u64();
        self.write_u64(a + b);
    }

    #[inline]
    pub fn add_i8(&mut self) {
        let a = self.read_u8() as i8;
        let b = self.read_u8() as i8;
        self.write_u8((a + b) as u8);
    }
    #[inline]
    pub fn add_i16(&mut self) {
        let a = self.read_u16() as i16;
        let b = self.read_u16() as i16;
        self.write_u16((a + b) as u16);
    }
    #[inline]
    pub fn add_i32(&mut self) {
        let a = self.read_u32() as i32;
        let b = self.read_u32() as i32;
        self.write_u32((a + b) as u32);
    }
    #[inline]
    pub fn add_i64(&mut self) {
        let a = self.read_u64() as i64;
        let b = self.read_u64() as i64;
        self.write_u64((a + b) as u64);
    }

    #[inline]
    pub fn add_f32(&mut self) {
        let a = self.read_u32() as f32;
        let b = self.read_u32() as f32;
        self.write_u32((a + b) as u32);
    }
    #[inline]
    pub fn add_f64(&mut self) {
        let a = self.read_u64() as f64;
        let b = self.read_u64() as f64;
        self.write_u64((a + b) as u64);
    }

    #[inline]
    pub fn sub_u8(&mut self) {
        let a = self.read_u8();
        let b = self.read_u8();
        self.write_u8(a - b);
    }
    #[inline]
    pub fn sub_u16(&mut self) {
        let a = self.read_u16();
        let b = self.read_u16();
        self.write_u16(a - b);
    }
    #[inline]
    pub fn sub_u32(&mut self) {
        let a = self.read_u32();
        let b = self.read_u32();
        self.write_u32(a - b);
    }
    #[inline]
    pub fn sub_u64(&mut self) {
        let a = self.read_u64();
        let b = self.read_u64();
        self.write_u64(a - b);
    }

    #[inline]
    pub fn sub_i8(&mut self) {
        let a = self.read_u8() as i8;
        let b = self.read_u8() as i8;
        self.write_u8((a - b) as u8);
    }
    #[inline]
    pub fn sub_i16(&mut self) {
        let a = self.read_u16() as i16;
        let b = self.read_u16() as i16;
        self.write_u16((a - b) as u16);
    }
    #[inline]
    pub fn sub_i32(&mut self) {
        let a = self.read_u32() as i32;
        let b = self.read_u32() as i32;
        self.write_u32((a - b) as u32);
    }
    #[inline]
    pub fn sub_i64(&mut self) {
        let a = self.read_u64() as i64;
        let b = self.read_u64() as i64;
        self.write_u64((a - b) as u64);
    }

    #[inline]
    pub fn sub_f32(&mut self) {
        let a = self.read_u32() as f32;
        let b = self.read_u32() as f32;
        self.write_u32((a - b) as u32);
    }
    #[inline]
    pub fn sub_f64(&mut self) {
        let a = self.read_u64() as f64;
        let b = self.read_u64() as f64;
        self.write_u64((a - b) as u64);
    }

    #[inline]
    pub fn mul_u8(&mut self) {
        let a = self.read_u8();
        let b = self.read_u8();
        self.write_u8(a - b);
    }
    #[inline]
    pub fn mul_u16(&mut self) {
        let a = self.read_u16();
        let b = self.read_u16();
        self.write_u16(a - b);
    }
    #[inline]
    pub fn mul_u32(&mut self) {
        let a = self.read_u32();
        let b = self.read_u32();
        self.write_u32(a - b);
    }
    #[inline]
    pub fn mul_u64(&mut self) {
        let a = self.read_u64();
        let b = self.read_u64();
        self.write_u64(a - b);
    }

    #[inline]
    pub fn mul_i8(&mut self) {
        let a = self.read_u8() as i8;
        let b = self.read_u8() as i8;
        self.write_u8((a * b) as u8);
    }
    #[inline]
    pub fn mul_i16(&mut self) {
        let a = self.read_u16() as i16;
        let b = self.read_u16() as i16;
        self.write_u16((a * b) as u16);
    }
    #[inline]
    pub fn mul_i32(&mut self) {
        let a = self.read_u32() as i32;
        let b = self.read_u32() as i32;
        self.write_u32((a * b) as u32);
    }
    #[inline]
    pub fn mul_i64(&mut self) {
        let a = self.read_u64() as i64;
        let b = self.read_u64() as i64;
        self.write_u64((a * b) as u64);
    }

    #[inline]
    pub fn mul_f32(&mut self) {
        let a = self.read_u32() as f32;
        let b = self.read_u32() as f32;
        self.write_u32((a * b) as u32);
    }
    #[inline]
    pub fn mul_f64(&mut self) {
        let a = self.read_u64() as f64;
        let b = self.read_u64() as f64;
        self.write_u64((a * b) as u64);
    }

    #[inline]
    pub fn div_u8(&mut self) {
        let a = self.read_u8();
        let b = self.read_u8();
        self.write_u8(a - b);
    }
    #[inline]
    pub fn div_u16(&mut self) {
        let a = self.read_u16();
        let b = self.read_u16();
        self.write_u16(a - b);
    }
    #[inline]
    pub fn div_u32(&mut self) {
        let a = self.read_u32();
        let b = self.read_u32();
        self.write_u32(a - b);
    }
    #[inline]
    pub fn div_u64(&mut self) {
        let a = self.read_u64();
        let b = self.read_u64();
        self.write_u64(a - b);
    }

    #[inline]
    pub fn div_i8(&mut self) {
        let a = self.read_u8() as i8;
        let b = self.read_u8() as i8;
        self.write_u8((a / b) as u8);
    }
    #[inline]
    pub fn div_i16(&mut self) {
        let a = self.read_u16() as i16;
        let b = self.read_u16() as i16;
        self.write_u16((a / b) as u16);
    }
    #[inline]
    pub fn div_i32(&mut self) {
        let a = self.read_u32() as i32;
        let b = self.read_u32() as i32;
        self.write_u32((a / b) as u32);
    }
    #[inline]
    pub fn div_i64(&mut self) {
        let a = self.read_u64() as i64;
        let b = self.read_u64() as i64;
        self.write_u64((a / b) as u64);
    }

    #[inline]
    pub fn div_f32(&mut self) {
        let a = self.read_u32() as f32;
        let b = self.read_u32() as f32;
        self.write_u32((a / b) as u32);
    }
    #[inline]
    pub fn div_f64(&mut self) {
        let a = self.read_u64() as f64;
        let b = self.read_u64() as f64;
        self.write_u64((a / b) as u64);
    }

    #[inline]
    pub fn rem_i8(&mut self) {
        let a = self.read_u8() as i8;
        let b = self.read_u8() as i8;
        self.write_u8((a % b) as u8);
    }
    #[inline]
    pub fn rem_i16(&mut self) {
        let a = self.read_u16() as i16;
        let b = self.read_u16() as i16;
        self.write_u16((a % b) as u16);
    }
    #[inline]
    pub fn rem_i32(&mut self) {
        let a = self.read_u32() as i32;
        let b = self.read_u32() as i32;
        self.write_u32((a % b) as u32);
    }
    #[inline]
    pub fn rem_i64(&mut self) {
        let a = self.read_u64() as i64;
        let b = self.read_u64() as i64;
        self.write_u64((a % b) as u64);
    }

    #[inline]
    pub fn rem_f32(&mut self) {
        let a = self.read_u32() as f32;
        let b = self.read_u32() as f32;
        self.write_u32((a % b) as u32);
    }
    #[inline]
    pub fn rem_f64(&mut self) {
        let a = self.read_u64() as f64;
        let b = self.read_u64() as f64;
        self.write_u64((a % b) as u64);
    }
}

#[cfg(test)]
mod test {
    use self::super::*;


    #[test]
    fn test_next() {
        let program = [0, 0, 0, 0, 0, 0, 0, 1];
        let mut process = Process::new(&program);
        assert_eq!(process.next_usize(), 1usize);
    }

    #[test]
    fn test_push_pop() {
        let program = [];
        let mut process = Process::new(&program);
        process.push_u64(1);
        assert_eq!(process.pop_u64(), 1);
    }
}
