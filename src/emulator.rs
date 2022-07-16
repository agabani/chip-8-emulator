use std::io::{Cursor, Write};

pub(crate) struct Emulator {
    memory: Memory,
}

pub(crate) struct Memory {
    ram: Vec<u8>,
}

impl Emulator {
    pub(crate) fn new() -> Emulator {
        Emulator {
            memory: Memory::new(),
        }
    }

    pub(crate) fn load_rom(&mut self, rom: &[u8]) -> crate::Result<()> {
        self.memory.load_rom(rom)
    }
}

impl Memory {
    pub(crate) fn new() -> Memory {
        Memory { ram: vec![0; 4096] }
    }

    pub(crate) fn load_rom(&mut self, rom: &[u8]) -> crate::Result<()> {
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
    fn loads_rom() {
        let mut rom = Vec::new();
        let mut file = std::fs::File::open("./roms/IBM Logo.ch8").unwrap();
        file.read_to_end(&mut rom).unwrap();

        let mut emulator = Emulator::new();
        emulator.load_rom(&rom).unwrap();
    }
}
