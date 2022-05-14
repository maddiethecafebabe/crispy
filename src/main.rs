#[macro_use]
extern crate tracing;

extern crate sdl2;

use std::{env, fs};

pub mod bell;
pub mod context;
pub mod display;
pub mod memory;
pub mod vm;
use bell::{Bell, PlayingStatus::*};
pub use vm::{Result, RuntimeError, Vm};

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use std::time::Duration;

use std::thread;

pub struct Chip8Emulator {
    ctx: context::Context,
    vm: vm::Vm,
}

impl Chip8Emulator {}

pub fn main() {
    tracing_subscriber::fmt::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let rom = env::args().nth(1).map(fs::read).unwrap().unwrap();
    let mut ctx = context::Context::new(rom);

    let vm = Vm::init(&mut ctx).unwrap();

    let mut emu = Chip8Emulator { ctx, vm };

    emu.ctx.canvas().set_draw_color(Color::RGB(0, 255, 255));
    emu.ctx.canvas().clear();
    emu.ctx.canvas().present();
    let mut event_pump = emu.ctx.sdl_ctx().event_pump().unwrap();

    'running: loop {
        emu.vm.step().unwrap();
        emu.vm.display.write_display_debug();

        info!("{:?}", emu.vm.regs);
        info!("{:?}", emu.vm.memory.stack());

        if emu.vm.regs.sound == 0 {
            emu.ctx.bell().set_status(Stopped);
        }

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        //emu.vm.display.render_canvas(&mut emu.ctx.canvas());


        emu.ctx.canvas().present();
        thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
