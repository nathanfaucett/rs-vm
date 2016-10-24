use core::mem;

use vector::Vector;
use stack::Stack;

use instrs::Instr;
use state::State;


#[derive(Debug)]
pub struct Process<'a> {
    state: State,

    program_counter: usize,
    program: &'a [u8],

    pub stack: Vector<u8>,
}

impl<'a> Process<'a> {

    #[inline]
    pub fn new(program: &'a [u8]) -> Self {
        Process {
            state: State::New,

            program_counter: 0,
            program: program,

            stack: Vector::new(),
        }
    }

    #[inline]
    pub fn get_state(&self) -> State {self.state}
    #[inline]
    pub fn set_state(&mut self, state: State) {
        self.state = state;
    }

    #[inline]
    pub fn restart(&mut self) {
        self.state = State::New;
        self.program_counter = 0;
        self.stack.clear();
    }

    #[inline]
    pub fn is_running(&self) -> bool {self.state == State::Running}
    #[inline]
    pub fn to_instr(instruction: u8) -> Instr {unsafe {mem::transmute(instruction)}}

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
            instr => panic!("Invalid type {:?} given, trying to read 8 bits ", instr),
        }
    }
    #[inline]
    pub fn read_u16(&mut self) -> u16 {
        let typ = Self::to_instr(self.next_u8());

        match typ {
            Instr::type_int => self.next_u16(),
            Instr::type_ptr => unsafe {*(self.next_usize() as *const u16)},
            instr => panic!("Invalid type {:?} given, trying to read 16 bits", instr),
        }
    }
    #[inline]
    pub fn read_u32(&mut self) -> u32 {
        let typ = Self::to_instr(self.next_u8());

        match typ {
            Instr::type_int => self.next_u32(),
            Instr::type_ptr => unsafe {*(self.next_usize() as *const u32)},
            instr => panic!("Invalid type {:?} given, trying to read 32 bits", instr),
        }
    }
    #[inline]
    pub fn read_u64(&mut self) -> u64 {
        let typ = Self::to_instr(self.next_u8());

        match typ {
            Instr::type_int => self.next_u64(),
            Instr::type_ptr => unsafe {*(self.next_usize() as *const u64)},
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
    pub fn halt(&mut self) {
        self.state = State::Terminated;
    }

    #[inline]
    pub fn jmp(&mut self) {
        let index = self.next_usize();
        self.program_counter = index;
    }
    #[inline]
    pub fn if_jmp(&mut self) {
        let value = self.pop_u8();

        if value == 0 {
            self.skip(8); // skip location instruction
        } else {
            let index = self.next_usize();
            self.program_counter = index;
        }
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
    pub fn peek_stack(&self, offset: usize) -> u8 {
        let len = self.stack.len();
        self.stack[len - offset - 1]
    }
    #[inline]
    pub fn peek_u8(&self, offset: usize) -> u8 {
        self.peek_stack(offset)
    }
    #[inline]
    pub fn peek_u16(&self, offset: usize) -> u16 {
        let b0 = self.peek_stack(offset + 0);
        let b1 = self.peek_stack(offset + 1);
        Self::to_u16(b1, b0)
    }
    #[inline]
    pub fn peek_u32(&self, offset: usize) -> u32 {
        let b0 = self.peek_stack(offset + 0);
        let b1 = self.peek_stack(offset + 1);
        let b2 = self.peek_stack(offset + 2);
        let b3 = self.peek_stack(offset + 3);
        Self::to_u32(b3, b2, b1, b0)
    }
    #[inline]
    pub fn peek_u64(&self, offset: usize) -> u64 {
        let b0 = self.peek_stack(offset + 0);
        let b1 = self.peek_stack(offset + 1);
        let b2 = self.peek_stack(offset + 2);
        let b3 = self.peek_stack(offset + 3);
        let b4 = self.peek_stack(offset + 4);
        let b5 = self.peek_stack(offset + 5);
        let b6 = self.peek_stack(offset + 6);
        let b7 = self.peek_stack(offset + 7);
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
    pub fn copy_u8(&mut self) {
        let value = self.peek_u8(0);
        let ref mut stack = self.stack;
        stack.push(value);
    }
    #[inline]
    pub fn copy_u16(&mut self) {
        let value = self.peek_u16(0);
        let ref mut stack = self.stack;
        stack.push((value >> 8) as u8);
        stack.push(value as u8);
    }
    #[inline]
    pub fn copy_u32(&mut self) {
        let value = self.peek_u32(0);
        let ref mut stack = self.stack;
        stack.push((value >> 24) as u8);
        stack.push((value >> 16) as u8);
        stack.push((value >> 8) as u8);
        stack.push(value as u8);
    }
    #[inline]
    pub fn copy_u64(&mut self) {
        let value = self.peek_u64(0);
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

    /*
        TODO use concat_idents when working
        https://github.com/rust-lang/rust/issues/29599
    */
    #[inline]
    pub fn add_u8(&mut self) {
        let b = self.pop_u8();
        let a = self.pop_u8();
        self.push_u8(a + b);
    }
    #[inline]
    pub fn add_u16(&mut self) {
        let b = self.pop_u16();
        let a = self.pop_u16();
        self.push_u16(a + b);
    }
    #[inline]
    pub fn add_u32(&mut self) {
        let b = self.pop_u32();
        let a = self.pop_u32();
        self.push_u32(a + b);
    }
    #[inline]
    pub fn add_u64(&mut self) {
        let b = self.pop_u64();
        let a = self.pop_u64();
        self.push_u64(a + b);
    }

    #[inline]
    pub fn add_i8(&mut self) {
        let b = self.pop_u8() as i8;
        let a = self.pop_u8() as i8;
        self.push_u8((a + b) as u8);
    }
    #[inline]
    pub fn add_i16(&mut self) {
        let b = self.pop_u16() as i16;
        let a = self.pop_u16() as i16;
        self.push_u16((a + b) as u16);
    }
    #[inline]
    pub fn add_i32(&mut self) {
        let b = self.pop_u32() as i32;
        let a = self.pop_u32() as i32;
        self.push_u32((a + b) as u32);
    }
    #[inline]
    pub fn add_i64(&mut self) {
        let b = self.pop_u64() as i64;
        let a = self.pop_u64() as i64;
        self.push_u64((a + b) as u64);
    }

    #[inline]
    pub fn add_f32(&mut self) {
        let b = self.pop_u32() as f32;
        let a = self.pop_u32() as f32;
        self.push_u32((a + b) as u32);
    }
    #[inline]
    pub fn add_f64(&mut self) {
        let b = self.pop_u64() as f64;
        let a = self.pop_u64() as f64;
        self.push_u64((a + b) as u64);
    }


    #[inline]
    pub fn sub_u8(&mut self) {
        let b = self.pop_u8();
        let a = self.pop_u8();
        self.push_u8(a - b);
    }
    #[inline]
    pub fn sub_u16(&mut self) {
        let b = self.pop_u16();
        let a = self.pop_u16();
        self.push_u16(a - b);
    }
    #[inline]
    pub fn sub_u32(&mut self) {
        let b = self.pop_u32();
        let a = self.pop_u32();
        self.push_u32(a - b);
    }
    #[inline]
    pub fn sub_u64(&mut self) {
        let b = self.pop_u64();
        let a = self.pop_u64();
        self.push_u64(a - b);
    }

    #[inline]
    pub fn sub_i8(&mut self) {
        let b = self.pop_u8() as i8;
        let a = self.pop_u8() as i8;
        self.push_u8((a - b) as u8);
    }
    #[inline]
    pub fn sub_i16(&mut self) {
        let b = self.pop_u16() as i16;
        let a = self.pop_u16() as i16;
        self.push_u16((a - b) as u16);
    }
    #[inline]
    pub fn sub_i32(&mut self) {
        let b = self.pop_u32() as i32;
        let a = self.pop_u32() as i32;
        self.push_u32((a - b) as u32);
    }
    #[inline]
    pub fn sub_i64(&mut self) {
        let b = self.pop_u64() as i64;
        let a = self.pop_u64() as i64;
        self.push_u64((a - b) as u64);
    }

    #[inline]
    pub fn sub_f32(&mut self) {
        let b = self.pop_u32() as f32;
        let a = self.pop_u32() as f32;
        self.push_u32((a - b) as u32);
    }
    #[inline]
    pub fn sub_f64(&mut self) {
        let b = self.pop_u64() as f64;
        let a = self.pop_u64() as f64;
        self.push_u64((a - b) as u64);
    }


    #[inline]
    pub fn mul_u8(&mut self) {
        let b = self.pop_u8();
        let a = self.pop_u8();
        self.push_u8(a * b);
    }
    #[inline]
    pub fn mul_u16(&mut self) {
        let b = self.pop_u16();
        let a = self.pop_u16();
        self.push_u16(a * b);
    }
    #[inline]
    pub fn mul_u32(&mut self) {
        let b = self.pop_u32();
        let a = self.pop_u32();
        self.push_u32(a * b);
    }
    #[inline]
    pub fn mul_u64(&mut self) {
        let b = self.pop_u64();
        let a = self.pop_u64();
        self.push_u64(a * b);
    }

    #[inline]
    pub fn mul_i8(&mut self) {
        let b = self.pop_u8() as i8;
        let a = self.pop_u8() as i8;
        self.push_u8((a * b) as u8);
    }
    #[inline]
    pub fn mul_i16(&mut self) {
        let b = self.pop_u16() as i16;
        let a = self.pop_u16() as i16;
        self.push_u16((a * b) as u16);
    }
    #[inline]
    pub fn mul_i32(&mut self) {
        let b = self.pop_u32() as i32;
        let a = self.pop_u32() as i32;
        self.push_u32((a * b) as u32);
    }
    #[inline]
    pub fn mul_i64(&mut self) {
        let b = self.pop_u64() as i64;
        let a = self.pop_u64() as i64;
        self.push_u64((a * b) as u64);
    }

    #[inline]
    pub fn mul_f32(&mut self) {
        let b = self.pop_u32() as f32;
        let a = self.pop_u32() as f32;
        self.push_u32((a * b) as u32);
    }
    #[inline]
    pub fn mul_f64(&mut self) {
        let b = self.pop_u64() as f64;
        let a = self.pop_u64() as f64;
        self.push_u64((a * b) as u64);
    }


    #[inline]
    pub fn div_u8(&mut self) {
        let b = self.pop_u8();
        let a = self.pop_u8();
        self.push_u8(a / b);
    }
    #[inline]
    pub fn div_u16(&mut self) {
        let b = self.pop_u16();
        let a = self.pop_u16();
        self.push_u16(a / b);
    }
    #[inline]
    pub fn div_u32(&mut self) {
        let b = self.pop_u32();
        let a = self.pop_u32();
        self.push_u32(a / b);
    }
    #[inline]
    pub fn div_u64(&mut self) {
        let b = self.pop_u64();
        let a = self.pop_u64();
        self.push_u64(a / b);
    }

    #[inline]
    pub fn div_i8(&mut self) {
        let b = self.pop_u8() as i8;
        let a = self.pop_u8() as i8;
        self.push_u8((a / b) as u8);
    }
    #[inline]
    pub fn div_i16(&mut self) {
        let b = self.pop_u16() as i16;
        let a = self.pop_u16() as i16;
        self.push_u16((a / b) as u16);
    }
    #[inline]
    pub fn div_i32(&mut self) {
        let b = self.pop_u32() as i32;
        let a = self.pop_u32() as i32;
        self.push_u32((a / b) as u32);
    }
    #[inline]
    pub fn div_i64(&mut self) {
        let b = self.pop_u64() as i64;
        let a = self.pop_u64() as i64;
        self.push_u64((a / b) as u64);
    }

    #[inline]
    pub fn div_f32(&mut self) {
        let b = self.pop_u32() as f32;
        let a = self.pop_u32() as f32;
        self.push_u32((a / b) as u32);
    }
    #[inline]
    pub fn div_f64(&mut self) {
        let b = self.pop_u64() as f64;
        let a = self.pop_u64() as f64;
        self.push_u64((a / b) as u64);
    }


    #[inline]
    pub fn rem_u8(&mut self) {
        let b = self.pop_u8();
        let a = self.pop_u8();
        self.push_u8(a % b);
    }
    #[inline]
    pub fn rem_u16(&mut self) {
        let b = self.pop_u16();
        let a = self.pop_u16();
        self.push_u16(a % b);
    }
    #[inline]
    pub fn rem_u32(&mut self) {
        let b = self.pop_u32();
        let a = self.pop_u32();
        self.push_u32(a % b);
    }
    #[inline]
    pub fn rem_u64(&mut self) {
        let b = self.pop_u64();
        let a = self.pop_u64();
        self.push_u64(a % b);
    }

    #[inline]
    pub fn rem_i8(&mut self) {
        let b = self.pop_u8() as i8;
        let a = self.pop_u8() as i8;
        self.push_u8((a % b) as u8);
    }
    #[inline]
    pub fn rem_i16(&mut self) {
        let b = self.pop_u16() as i16;
        let a = self.pop_u16() as i16;
        self.push_u16((a % b) as u16);
    }
    #[inline]
    pub fn rem_i32(&mut self) {
        let b = self.pop_u32() as i32;
        let a = self.pop_u32() as i32;
        self.push_u32((a % b) as u32);
    }
    #[inline]
    pub fn rem_i64(&mut self) {
        let b = self.pop_u64() as i64;
        let a = self.pop_u64() as i64;
        self.push_u64((a % b) as u64);
    }

    #[inline]
    pub fn rem_f32(&mut self) {
        let b = self.pop_u32() as f32;
        let a = self.pop_u32() as f32;
        self.push_u32((a % b) as u32);
    }
    #[inline]
    pub fn rem_f64(&mut self) {
        let b = self.pop_u64() as f64;
        let a = self.pop_u64() as f64;
        self.push_u64((a % b) as u64);
    }


    #[inline]
    pub fn eq_u8(&mut self) {
        let b = self.pop_u8();
        let a = self.pop_u8();
        self.push_u8((a == b) as u8);
    }
    #[inline]
    pub fn eq_u16(&mut self) {
        let b = self.pop_u16();
        let a = self.pop_u16();
        self.push_u8((a == b) as u8);
    }
    #[inline]
    pub fn eq_u32(&mut self) {
        let b = self.pop_u32();
        let a = self.pop_u32();
        self.push_u8((a == b) as u8);
    }
    #[inline]
    pub fn eq_u64(&mut self) {
        let b = self.pop_u64();
        let a = self.pop_u64();
        self.push_u8((a == b) as u8);
    }


    #[inline]
    pub fn eq_i8(&mut self) {
        let b = self.pop_u8() as i8;
        let a = self.pop_u8() as i8;
        self.push_u8((a == b) as u8);
    }
    #[inline]
    pub fn eq_i16(&mut self) {
        let b = self.pop_u16() as i16;
        let a = self.pop_u16() as i16;
        self.push_u8((a == b) as u8);
    }
    #[inline]
    pub fn eq_i32(&mut self) {
        let b = self.pop_u32() as i32;
        let a = self.pop_u32() as i32;
        self.push_u8((a == b) as u8);
    }
    #[inline]
    pub fn eq_i64(&mut self) {
        let b = self.pop_u64() as i64;
        let a = self.pop_u64() as i64;
        self.push_u8((a == b) as u8);
    }

    #[inline]
    pub fn eq_f32(&mut self) {
        let b = self.pop_u32() as f32;
        let a = self.pop_u32() as f32;
        self.push_u8((a == b) as u8);
    }
    #[inline]
    pub fn eq_f64(&mut self) {
        let b = self.pop_u64() as f64;
        let a = self.pop_u64() as f64;
        self.push_u8((a == b) as u8);
    }


    #[inline]
    pub fn neq_u8(&mut self) {
        let b = self.pop_u8();
        let a = self.pop_u8();
        self.push_u8((a != b) as u8);
    }
    #[inline]
    pub fn neq_u16(&mut self) {
        let b = self.pop_u16();
        let a = self.pop_u16();
        self.push_u8((a != b) as u8);
    }
    #[inline]
    pub fn neq_u32(&mut self) {
        let b = self.pop_u32();
        let a = self.pop_u32();
        self.push_u8((a != b) as u8);
    }
    #[inline]
    pub fn neq_u64(&mut self) {
        let b = self.pop_u64();
        let a = self.pop_u64();
        self.push_u8((a != b) as u8);
    }

    #[inline]
    pub fn neq_i8(&mut self) {
        let b = self.pop_u8() as i8;
        let a = self.pop_u8() as i8;
        self.push_u8((a != b) as u8);
    }
    #[inline]
    pub fn neq_i16(&mut self) {
        let b = self.pop_u16() as i16;
        let a = self.pop_u16() as i16;
        self.push_u8((a != b) as u8);
    }
    #[inline]
    pub fn neq_i32(&mut self) {
        let b = self.pop_u32() as i32;
        let a = self.pop_u32() as i32;
        self.push_u8((a != b) as u8);
    }
    #[inline]
    pub fn neq_i64(&mut self) {
        let b = self.pop_u64() as i64;
        let a = self.pop_u64() as i64;
        self.push_u8((a != b) as u8);
    }

    #[inline]
    pub fn neq_f32(&mut self) {
        let b = self.pop_u32() as f32;
        let a = self.pop_u32() as f32;
        self.push_u8((a != b) as u8);
    }
    #[inline]
    pub fn neq_f64(&mut self) {
        let b = self.pop_u64() as f64;
        let a = self.pop_u64() as f64;
        self.push_u8((a != b) as u8);
    }
}
