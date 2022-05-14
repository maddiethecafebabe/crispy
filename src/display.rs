pub struct Display([[bool; 32]; 64]);

impl Display {
    pub fn new() -> Self {
        let this = Self([[false; 32]; 64]); 


        this
    }

    pub fn set(&mut self, x: usize, y: usize, value: bool) {
        self.0[x][y] = value;
    }

    pub fn get(&self, x: usize, y: usize) -> bool {
        self.0[x][y]
    }

    pub fn inner(&self) -> &[[bool; 32]; 64] {
        &self.0
    }

    pub fn render(&mut self) {
        
    }

    pub fn inner_mut(&mut self) -> &mut [[bool; 32]; 64] {
        &mut self.0
    }
}
