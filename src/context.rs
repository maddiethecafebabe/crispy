use sdl2::{render::Canvas, video::Window, Sdl};

use crate::{display::Display, Bell};

pub struct Context {
    sdl_ctx: Sdl,
    canvas: Canvas<Window>,
    bell: Bell,
    image: Vec<u8>,
    pub display: Option<Display>,
}

impl Context {
    pub fn new(image: Vec<u8>) -> Self {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        let audio_subsystem = sdl_context.audio().unwrap();

        let canvas = video_subsystem
            .window("crispi", 800, 600)
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
            display: Some(display),
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
}
