mod cpu;
mod display;
mod font;
mod instruction;
mod memory;

use cpu::Cpu;
use display::Display;
use font::Font;
use memory::Memory;

pub(crate) struct Emulator {
    cpu: Cpu,
    display: Display,
    memory: Memory,
    paused: bool,
}

impl Emulator {
    pub(crate) fn new() -> Emulator {
        let mut emulator = Emulator {
            cpu: Cpu::new(),
            display: Display::new(),
            memory: Memory::new(),
            paused: true,
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

        self.cpu.execute(&mut self.display, &mut self.memory);
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
