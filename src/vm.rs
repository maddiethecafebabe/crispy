use crate::{context::Context, display::Display};
use chip8_instruction as instruction;

use crate::memory::Memory;

#[derive(Debug)]
#[allow(non_snake_case)]
pub struct Registers {
    pub pc: u16,
    pub v: [u8; 0x10],
    pub I: u16,
    pub delay: u8,
    pub sound: u8,
}

impl Registers {
    pub fn new() -> Self {
        Self {
            pc: 0x200,
            v: [0x00; 0x10],
            I: 0,
            delay: 0,
            sound: 0,
        }
    }
}

pub struct Vm {
    pub(crate) memory: Memory,
    pub(crate) regs: Registers,
    pub(crate) display: Display,
    do_variable_shifts: bool,
}

#[derive(Debug)]
pub enum RuntimeError {
    InvalidInstruction,
    IllegalMemoryAccess(u16),
    Stackoverflow,
    Stackunderflow,
}

impl std::fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        <Self as std::fmt::Debug>::fmt(self, f)
    }
}

pub type Result<T> = core::result::Result<T, RuntimeError>;

impl Vm {
    pub fn init(ctx: &mut Context) -> Result<Self> {
        let mut this = Self {
            memory: Memory::empty(),
            regs: Registers::new(),
            display: ctx.display.take().unwrap(),
            do_variable_shifts: false,
        };

        this.memory.load_rom(ctx.rom())?;

        Ok(this)
    }

    pub fn step(&mut self) -> Result<()> {
        let next = self.fetch_next_instruction()?;
        self.process_next_instruction(next)
    }

    pub fn fetch_next_instruction(&mut self) -> Result<instruction::Instruction> {
        let next = self.memory.load_u16(self.regs.pc)?;
        self.regs.pc += 2;
        let instr = instruction::decode(next);

        debug!("Fetched 0x{:x}, pc is now at 0x{:x}", next, self.regs.pc);
        debug!("Decoded to {:?}", instr);

        Ok(instr)
    }

    pub fn process_next_instruction(&mut self, next: instruction::Instruction) -> Result<()> {
        use instruction::Instruction::*;
        debug!("Processing {:?}", next);
        match next {
            SysJmp(_) => (), // ignored
            ClearScreen => self.display.clear(),
            Return => self.regs.pc = self.memory.stack_mut().pop()?,
            Jump(addr) => self.regs.pc = addr,
            Call(addr) => {
                self.memory.stack_mut().push(self.regs.pc)?;
                self.regs.pc = addr;
            }
            SkipIfEqualImmidiate(reg, val) => {
                if self.regs.v[reg as usize] == val {
                    self.regs.pc += 2;
                }
            }
            XorRegister(a, b) => self.regs.v[a as usize] ^= self.regs.v[b as usize],
            LoadI(r) => self.regs.I = r,
            RegDumpI(i) => {
                for idx in 0..i {
                    self.memory
                        .store_u8(self.regs.I + idx as u16, self.regs.v[idx as usize])?
                }
            }
            RegLoadI(i) => {
                for idx in 0..i {
                    let val = self.memory.load_u8(self.regs.I + idx as u16)?;
                    self.regs.v[idx as usize] = val;
                }
            }
            LoadImmidiate(reg, val) => self.regs.v[reg as usize] = val,
            LoadRegister(reg_dst, reg_src) => {
                self.regs.v[reg_dst as usize] = self.regs.v[reg_src as usize]
            }
            AddI(val) => self.regs.I += val as u16,
            AddImmidiate(reg, r_val) => {
                let mut val = self.regs.v[reg as usize] as u16;
                val += r_val as u16;
                if val > u8::MAX as u16 {
                    // on overflow, set carry
                    self.regs.v[0xf] = 1;
                } else {
                    // clear existing if present
                    self.regs.v[0xf] = 0;
                }
                self.regs.v[reg as usize] = (val & 0xff) as u8;
            }
            ShrRegister(regx, regy) => {
                let by = if self.do_variable_shifts { regy } else { 1 };

                let val = self.regs.v[regx as usize];

                // set carry
                self.regs.v[0xf] = val & 0b0000_0001;

                self.regs.v[regx as usize] = val >> by;
            }
            ShlRegister(regx, regy) => {
                let by = if self.do_variable_shifts { regy } else { 1 };

                let val = self.regs.v[regx as usize];

                // set carry
                self.regs.v[0xf] = val & 0b1000_0000;

                self.regs.v[regx as usize] = val << by;
            }
            AndRegister(regx, regy) => self.regs.v[regx as usize] &= self.regs.v[regy as usize],
            DisplaySprite(regx, regy, sprite_len) => {
                let (x, y) = (self.regs.v[regx as usize], self.regs.v[regy as usize]);

                for sp_i in 0..sprite_len {
                    let cur_sprite_byte = self.memory.load_u8(self.regs.I + sp_i as u16)?;
                    self.display.render_sprite_byte_at(
                        x,
                        y + sp_i,
                        cur_sprite_byte,
                        &mut self.regs.v[0xf],
                    );
                }
            }
            i => todo!("{:?}", i),
        };

        Ok(())
    }
}
