#![feature(collections)]
//#![no_std]


extern crate core;
extern crate collections;


mod instrs;
mod process;
mod state;
mod regs_id;
mod regs;
mod virtual_machine;


pub use instrs::Instr;
pub use process::Process;
pub use regs_id::RegsId;
pub use regs::Regs;
pub use state::State;
pub use virtual_machine::VirtualMachine;
