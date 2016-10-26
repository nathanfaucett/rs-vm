#![no_std]


extern crate vector;
extern crate collection;
extern crate stack;
extern crate remove;


mod instrs;
mod process;
mod state;
mod vm;


pub use instrs::Instr;
pub use process::Process;
pub use state::State;
pub use vm::run;
