use chip8_instruction as instr;

use std::{env, fs, array::IntoIter};

struct U16Iter<I: Iterator<Item = u8>>(I);

impl<I: Iterator<Item = u8>> Iterator for U16Iter<I> {
    type Item = u16;

    fn next(&mut self) -> Option<Self::Item> {
        let hi = self.0.next()?;
        let lo = self.0.next()?;

        Some((hi as u16) << 8 | lo as u16)
    }
}

fn main() {
    let rom = env::args().nth(1).map(fs::read).unwrap().unwrap().into_iter();
    let opcodes = U16Iter(rom);

    for (idx, opcode) in opcodes.enumerate() {
        println!("0x{idx:04x} | {:?}", instr::decode(opcode));
    }
}
