use sdl2::{
    Sdl, render::Canvas,
    video::Window
};

use crate::{Bell, display::Display};

pub struct Context {
    sdl_ctx: Sdl,
    canvas: Canvas<Window>,
    bell: Bell,
    image: Vec<u8>,
    display: Display,
}

impl Context {
    pub fn new(image: Vec<u8>) -> Self {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        let audio_subsystem = sdl_context.audio().unwrap();

        let canvas = video_subsystem.window("rust-sdl2 demo", 800, 600)
            .position_centered()
            .opengl()
            .build()
            .unwrap()
            .into_canvas()
            .build()
            .unwrap();

        let bell = Bell::new(&audio_subsystem);
        let display = Display::new();

        Self { 
            sdl_ctx: sdl_context,
            canvas,
            bell,
            image,
            display
        }
    }

    pub fn bell(&mut self) -> &mut Bell {
        &mut self.bell
    }

    pub fn canvas(&mut self) -> &mut Canvas<Window> {
        &mut self.canvas
    }

    pub fn sdl_ctx(&self) -> &Sdl {
        &self.sdl_ctx
    }

    pub fn rom(&self) -> &[u8] {
        &self.image
    }

    pub fn display(&self) -> &Display {
        &self.display
    }

    pub fn display_mut(&mut self) -> &mut Display {
        &mut self.display
    }

    pub fn run(self, tick: impl Fn()) {
        
    }
}
