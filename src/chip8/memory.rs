use std::io::{Cursor, Write};

pub(super) struct Memory {
    /// Memory: CHIP-8 has direct access to up to 4 kilobytes of RAM
    ram: Vec<u8>,
}

impl Memory {
    pub(super) fn new() -> Memory {
        Memory { ram: vec![0; 4096] }
    }

    pub(super) fn load_font(&mut self, font: &[u8]) -> crate::Result<()> {
        let mut cursor = Cursor::new(&mut self.ram);
        cursor.set_position(0x050);
        cursor.write_all(font)?;
        Ok(())
    }

    pub(super) fn load_rom(&mut self, rom: &[u8]) -> crate::Result<()> {
        let mut cursor = Cursor::new(&mut self.ram);
        cursor.set_position(0x200);
        cursor.write_all(rom)?;
        Ok(())
    }

    pub(super) fn get_ram(&self) -> &[u8] {
        &self.ram
    }

    pub(super) fn get_byte(&self, address: u16) -> u8 {
        self.ram[address as usize]
    }

    pub(super) fn set_byte(&mut self, address: u16, byte: u8) {
        self.ram[address as usize] = byte;
    }
}
