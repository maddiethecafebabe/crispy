use sdl2::{pixels::Color, rect::Rect, render::Canvas, video::Window};

pub struct Display([[bool; 64]; 32]);

impl Display {
    pub fn new() -> Self {
        let this = Self([[false; 64]; 32]);

        this
    }

    pub fn set(&mut self, x: usize, y: usize, value: bool) {
        self.0[y][x] = value;
    }

    pub fn get(&self, x: usize, y: usize) -> bool {
        self.0[y][x]
    }

    pub fn inner(&self) -> &[[bool; 64]; 32] {
        &self.0
    }

    pub fn render(&mut self) {}

    pub fn inner_mut(&mut self) -> &mut [[bool; 64]; 32] {
        &mut self.0
    }

    pub const fn width(&self) -> u8 {
        self.0[0].len() as u8
    }

    pub const fn height(&self) -> u8 {
        self.0.len() as u8
    }

    pub fn clear(&mut self) {
        for line in self.0.iter_mut() {
            for c in line.iter_mut() {
                *c = false;
            }
        }
    }

    pub fn write_display_debug(&self) {
        for line in self.0 {
            for c in line {
                print!("{}", if c { 'X' } else { 'O' })
            }
            print!("\n")
        }
        println!()
    }

    pub fn render_sprite_byte_at(&mut self, x: u8, y: u8, sprite_data: u8, collision_reg: &mut u8) {
        debug!("Rendering {sprite_data:08b}");

        let mut collision = false;
        for bit in 0..8 {
            let new_bit = sprite_data & (0b1 << bit) != 1;

            let x_off = ((x + bit) % self.width()) as usize;
            let current_bit = self.get(x_off, y as usize);

            self.set(x_off, y as usize, new_bit ^ current_bit);

            collision |= !current_bit && new_bit;
        }
        *collision_reg = collision as u8;
    }

    pub fn render_canvas(&mut self, canvas: &mut Canvas<Window>) {
        debug!("rendering canvas...");
        let (width, height) = canvas.logical_size();
        let pixels_per_cell =
            core::cmp::min(width / self.width() as u32, height / self.height() as u32);

        for (rect, pixel) in self.0.iter().enumerate().flat_map(|(y_pos, line)| {
            line.iter().enumerate().map(move |(x_pos, c)| {
                (
                    Rect::new(
                        (x_pos as u32 * pixels_per_cell) as i32,
                        (y_pos as u32 * pixels_per_cell) as i32,
                        pixels_per_cell,
                        pixels_per_cell,
                    ),
                    *c,
                )
            })
        }) {
            canvas.set_draw_color(if pixel {
                Color::RGB(255, 255, 255)
            } else {
                Color::RGB(0, 0, 255)
            });
            trace!("canvas.fill_rect(rect: {:?}, pixel: {:?})", rect, pixel);
            canvas.fill_rect(rect).unwrap();
        }
    }
}
