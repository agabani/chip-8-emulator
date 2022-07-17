mod display;
mod font;
mod instruction;
mod memory;

use display::Display;
use font::Font;
use instruction::Instruction;
use memory::Memory;

pub(crate) struct Emulator {
    display: Display,
    memory: Memory,
    paused: bool,
}

impl Emulator {
    pub(crate) fn new() -> Emulator {
        let mut emulator = Emulator {
            paused: true,
            display: Display::new(),
            memory: Memory::new(),
        };

        emulator
            .memory
            .load_font(Font::new().data())
            .expect("failed to load font");

        emulator
    }

    pub(crate) fn is_pixel_on(&self, x: u8, y: u8) -> bool {
        self.display.is_pixel_on(x, y)
    }

    pub(crate) fn load_rom(&mut self, rom: &[u8]) -> crate::Result<()> {
        self.paused = false;
        self.memory.load_rom(rom)
    }

    pub(crate) fn fetch_decode_execute(&mut self) {
        if self.paused {
            return;
        }

        match self.memory.get_instruction() {
            Instruction::ClearScreen => self.display.clear_screen(),
            Instruction::Jump { nnn } => self.memory.set_program_counter(nnn),
            Instruction::SetRegister { x, nn } => self.memory.set_v_register(x, nn),
            Instruction::AddValueToRegister { x, nn } => self
                .memory
                .set_v_register(x, self.memory.get_v_register(x) + nn),
            Instruction::SetIndexRegister { nnn } => self.memory.set_index_register(nnn),
            Instruction::DisplayDraw { x, y, n } => {
                // Set the X coordinate to the value in VX modulo 64
                let x = self.memory.get_v_register(x) % 64;
                // Set the Y coordinate to the value in VY modulo 32
                let y = self.memory.get_v_register(y) % 32;

                // Set VF to 0
                self.memory.set_v_register(0xF, 0);

                // For N rows
                for row in 0..n {
                    // Get the Nth byte of sprite data, counting from the memory address in the I register
                    let sprite_data = self
                        .memory
                        .get_ram_byte(self.memory.get_index_register() as usize + row as usize);

                    // For each of the 8 pixels/bits in this sprite row
                    for pixel in 0..8 {
                        let sprite_row_pixel = match (sprite_data >> (7 - pixel)) & 0x1 {
                            1 => true,
                            0 => false,
                            v => panic!("{}", v),
                        };
                        let display_pixel = self.display.is_pixel_on(x + pixel, y + n - row);

                        // If the current pixel in the sprite row is on and the pixel at coordinates X,Y on the screen is also on
                        if sprite_row_pixel && display_pixel {
                            // turn off the pixel
                            self.display.set_pixel(x + pixel, y + n - row, false);
                            // set VF to 1
                            self.memory.set_v_register(0xF, 1);
                        }
                        // Or if the current pixel in the sprite row is on and the screen pixel is not
                        else if sprite_row_pixel && !display_pixel {
                            // draw the pixel at the X and Y coordinates
                            self.display
                                .set_pixel(x + pixel, y + n - row, sprite_row_pixel);
                        }

                        // If you reach the right edge of the screen, stop drawing this row
                        if x + pixel == 63 {
                            break;
                        };
                    }

                    if y + row == 31 {
                        break;
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use std::io::Read;

    use super::*;

    #[test]
    fn get_pixel() {
        let emulator = Emulator::new();

        assert_eq!(emulator.is_pixel_on(0, 0), false);
        assert_eq!(emulator.is_pixel_on(63, 0), false);
        assert_eq!(emulator.is_pixel_on(0, 0), false);
        assert_eq!(emulator.is_pixel_on(0, 31), false);
    }

    #[test]
    fn loads_rom() {
        let mut rom = Vec::new();
        let mut file = std::fs::File::open("./roms/IBM Logo.ch8").unwrap();
        file.read_to_end(&mut rom).unwrap();

        let mut emulator = Emulator::new();
        emulator.load_rom(&rom).unwrap();
    }
}
