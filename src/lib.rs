#![feature(collections)]
#![no_std]


extern crate collections;

extern crate vector;
extern crate stack;


mod instrs;
mod process;
mod state;
mod virtual_machine;


pub use instrs::Instr;
pub use process::Process;
pub use state::State;
pub use virtual_machine::VirtualMachine;
