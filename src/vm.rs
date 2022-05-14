use crate::context::Context;
use chip8_instruction as instruction;

struct Memory(Box<[u8; 0x1000]>);

impl Memory {
    pub fn empty() -> Self {
        Self(Box::new([0xcc; 0x1000]))
    }

    pub fn load_u8(&self, addr: u16) -> u8 {
        unsafe { *self.0.get_unchecked(addr as usize) }
    }

    pub fn load_u16(&self, addr: u16) -> u16 {
        ((self.0[addr as usize] as u16) << 8) | self.0[(addr + 1) as usize] as u16
    }

    pub fn store_u8(&mut self, addr: u16, value: u8) {
        unsafe { *self.0.get_unchecked_mut(addr as usize) = value; }
    }

    pub fn raw(&self) -> &[u8; 0x1000] {
        &self.0
    }

    pub fn raw_mut(&mut self) -> &mut [u8; 0x1000] {
        &mut self.0
    }

    pub fn load_rom(&mut self, rom: &[u8]) {
        assert!(rom.len() < 0x1000 - 0x200);
        for (idx, c) in rom.iter().enumerate() {
            *self.0.get_mut(idx).unwrap() = *c;
        }
    }
}

struct Registers {
    pub pc: u16,
    pub sp: u8,
}

impl Registers {
    pub fn new() -> Self {
        Self { pc: 0x200, sp: 0 }
    }
}

pub struct Vm {
    memory: Memory,
    regs: Registers,
}

impl Vm {
    pub fn init(ctx: &Context) -> Self {
        let mut this = Self { 
            memory: Memory::empty(),
            regs: Registers::new(),
        };

        this.memory.load_rom(ctx.rom());

        this
    }

    pub fn process_next_instruction(&mut self) {
        let next = self.memory.load_u16(self.regs.pc);
        self.regs.pc += 2;

        use instruction::Instruction::*;
        match instruction::decode(next).expect("invalid instruction") {
            _ => todo!()
        }
    }
}
