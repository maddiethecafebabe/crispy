extern crate sdl2;

use std::{env, fs};

pub mod vm;
pub mod display;
pub mod context;
pub mod bell;
use bell::{Bell, PlayingStatus::*};

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;

pub struct Chip8Emulator {
    ctx: context::Context,
    vm: vm::Vm,
}

impl Chip8Emulator {

}

pub fn main() {
    let rom = env::args().nth(1).map(fs::read).unwrap().unwrap();

    let mut ctx = context::Context::new(rom);

    ctx.canvas().set_draw_color(Color::RGB(0, 255, 255));
    ctx.canvas().clear();
    ctx.canvas().present();
    let mut event_pump = ctx.sdl_ctx().event_pump().unwrap();
    let mut i = 0;
    'running: loop {


        i = (i + 1) % 255;
        ctx.canvas().set_draw_color(Color::RGB(i, 64, 255 - i));
        ctx.canvas().clear();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                Event::KeyDown { keycode: Some(Keycode::Space), .. } => {
                    ctx.bell().set_status(Playing);
                },
                Event::KeyUp { keycode: Some(Keycode::Space), .. } => {
                    ctx.bell().set_status(Stopped);
                },
                _ => {}
            }
        }

        ctx.canvas().present();
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
