use std::io::{Cursor, Write};

pub(crate) struct Emulator {
    display: Display,
    memory: Memory,
}

struct Display {
    pixels: [[bool; 64]; 32],
}

struct Memory {
    ram: Vec<u8>,
}

impl Emulator {
    pub(crate) fn new() -> Emulator {
        Emulator {
            display: Display::new(),
            memory: Memory::new(),
        }
    }

    pub(crate) fn is_pixel_on(&self, x: u16, y: u16) -> bool {
        self.display.is_pixel_on(x, y)
    }

    pub(crate) fn load_rom(&mut self, rom: &[u8]) -> crate::Result<()> {
        self.memory.load_rom(rom)
    }
}

impl Display {
    fn new() -> Display {
        Display {
            pixels: [[false; 64]; 32],
        }
    }

    fn is_pixel_on(&self, x: u16, y: u16) -> bool {
        self.pixels[y as usize][x as usize]
    }
}

impl Memory {
    fn new() -> Memory {
        Memory { ram: vec![0; 4096] }
    }

    fn load_rom(&mut self, rom: &[u8]) -> crate::Result<()> {
        let mut cursor = Cursor::new(&mut self.ram);
        cursor.set_position(0x200);
        cursor.write_all(rom)?;
        Ok(())
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
