pub(super) struct Display {
    /// 64 x 32 pixels monochrome, ie. black or white
    pixels: [[bool; 64]; 32],
}

impl Display {
    pub(super) fn new() -> Display {
        Display {
            pixels: [[false; 64]; 32],
        }
    }

    pub(super) fn clear_screen(&mut self) {
        self.pixels = [[false; 64]; 32];
    }

    pub(super) fn is_pixel_on(&self, x: u8, y: u8) -> bool {
        self.pixels[y as usize][x as usize]
    }

    pub(super) fn set_pixel(&mut self, x: u8, y: u8, value: bool) {
        self.pixels[y as usize][x as usize] = value;
    }
}
