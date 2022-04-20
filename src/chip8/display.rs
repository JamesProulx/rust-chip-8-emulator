use pixels::Pixels;

pub const DISPLAY_WIDTH:usize = 64;
pub const DISPLAY_HEIGHT:usize = 32;

pub struct Display {
    pixels: [[bool; DISPLAY_WIDTH]; DISPLAY_HEIGHT]
}

impl Display {
    pub fn draw(&self, pixels: &mut Pixels) {
        for (i, pixel) in pixels.get_frame().chunks_exact_mut(4).enumerate() {
            let x = (i % DISPLAY_WIDTH as usize) as i16;
            let y = (i / DISPLAY_WIDTH as usize) as i16;

            let rgba = if self.pixels[y as usize][x as usize] {
                [0xFF, 0xFF, 0xFF, 0xFF]
            } else {
                [0x00, 0x00, 0x00, 0x00]
            };

            pixel.copy_from_slice(&rgba);
        }
    }

    pub fn clear_screen(&mut self) {
        for y in self.pixels.iter_mut() {
            for x in y.iter_mut() {
                *x = false;
            }
        };
    }

    pub fn get_value_at_coord(&self, x: usize, y: usize) -> u8 {

        match self.pixels[y % DISPLAY_HEIGHT][x % DISPLAY_WIDTH] {
            true => 1,
            false => 0
        }
    }

    pub fn set_value_at_coord(&mut self, x: usize, y: usize, value: u8) {

        let x = x % DISPLAY_WIDTH;
        let y = y % DISPLAY_HEIGHT;

        let xor_true = self.pixels[y][x] ^ true;
        let xor_false = self.pixels[y][x] ^ false;

        match value {
            0 => self.pixels[y][x] = xor_false,
            1 => self.pixels[y][x] = xor_true,
            _ => panic!("Tried to set invalid pixel!")
        }
    }

    pub fn new() -> Self {
        Self {
            pixels: [[false; 64]; 32]
        }
    }
}