use crate::{Result, RuntimeError};

const SPRITE_DATA: &[u8] = &[
    0xf0, 0x90, 0x90, 0x90, 0xf0, // '0'
    0x20, 0x60, 0x20, 0x20, 0x70, // '1'
    0xf0, 0x10, 0xf0, 0x80, 0xf0, // '2'
    0xf0, 0x10, 0xf0, 0x10, 0xf0, // '3'
    0x90, 0x90, 0xf0, 0x10, 0x10, // '4'
    0xf0, 0x80, 0xf0, 0x10, 0xf0, // '5'
    0xf0, 0x80, 0xf0, 0x90, 0xf0, // '6'
    0xf0, 0x10, 0x20, 0x40, 0x40, // '7'
    0xf0, 0x90, 0xf0, 0x90, 0xf0, // '8'
    0xf0, 0x90, 0xf0, 0x10, 0xf0, // '9'
    0xf0, 0x90, 0xf0, 0x90, 0x90, // 'A'
    0xe0, 0x90, 0xe0, 0x90, 0xe0, // 'B'
    0xf0, 0x80, 0x80, 0x80, 0xf0, // 'C'
    0xe0, 0x90, 0x90, 0x90, 0xe0, // 'D'
    0xf0, 0x80, 0xf0, 0x80, 0xf0, // 'E'
    0xf0, 0x80, 0xf0, 0x80, 0x80, // 'F'
];

const SPRITE_START_ADDR: u16 = 0x100;

pub struct Memory([u8; 0x1000], Stack);

impl Memory {
    pub fn empty() -> Self {
        Self([0xcc; 0x1000], Stack::init())
    }

    pub fn load_u8(&self, addr: u16) -> Result<u8> {
        self.0
            .get(addr as usize)
            .map(|b| *b)
            .ok_or(RuntimeError::IllegalMemoryAccess(addr))
    }

    pub fn load_u16(&self, addr: u16) -> Result<u16> {
        Ok(u16::from_be_bytes([
            self.load_u8(addr)?,
            self.load_u8(addr + 1)?,
        ]))
    }

    pub fn store_u8(&mut self, addr: u16, value: u8) -> Result<()> {
        self.0
            .get_mut(addr as usize)
            .map(|b| *b = value)
            .ok_or(RuntimeError::IllegalMemoryAccess(addr))
    }

    pub fn raw(&self) -> &[u8; 0x1000] {
        &self.0
    }

    pub fn raw_mut(&mut self) -> &mut [u8; 0x1000] {
        &mut self.0
    }

    pub fn stack(&self) -> &Stack {
        &self.1
    }

    pub fn get_sprite(&self, character: u8) -> u16 {
        let character = character & 0xf;
        SPRITE_START_ADDR + (0x5 * character) as u16
    }

    pub fn stack_mut(&mut self) -> &mut Stack {
        &mut self.1
    }

    pub fn init_interpreter_data(&mut self) {
        for (offset, &c) in SPRITE_DATA.iter().enumerate() {
            self.0[SPRITE_START_ADDR as usize + offset] = c;
        }
    }

    pub fn load_rom(&mut self, rom: &[u8]) -> Result<()> {
        assert!(rom.len() < 0x1000 - 0x200);
        for (idx, c) in rom.iter().enumerate() {
            *self
                .0
                .get_mut(idx + 0x200)
                .ok_or(RuntimeError::IllegalMemoryAccess(idx as u16 + 0x200))? = *c;
        }
        Ok(())
    }
}

#[derive(Debug)]
pub struct Stack {
    sp: usize,
    raw: [u16; Stack::STACK_FRAME_SIZE as usize],
}

impl Stack {
    const STACK_FRAME_SIZE: u16 = 0x10;

    pub fn init() -> Self {
        Self {
            sp: 0,
            raw: [0; Stack::STACK_FRAME_SIZE as usize],
        }
    }

    pub fn push(&mut self, val: u16) -> Result<()> {
        *self
            .raw
            .get_mut(self.sp)
            .ok_or(RuntimeError::Stackoverflow)? = val;

        self.sp += 1;
        Ok(())
    }

    pub fn pop(&mut self) -> Result<u16> {
        self.sp -= 1;
        self.raw
            .get(self.sp)
            .map(|b| *b)
            .ok_or(RuntimeError::Stackunderflow)
    }

    pub fn peek(&self) -> Result<u16> {
        self.raw
            .get(self.sp)
            .map(|b| *b)
            .ok_or(RuntimeError::Stackunderflow)
    }

    pub fn sp(&self) -> u16 {
        self.sp as u16
    }
}
