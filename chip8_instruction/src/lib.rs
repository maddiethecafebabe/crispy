#![no_std]

mod instr;
pub use instr::Instruction;

mod decode;
pub use decode::decode;
