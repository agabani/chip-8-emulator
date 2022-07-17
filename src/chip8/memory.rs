use std::io::{Cursor, Read, Write};

use super::instruction::Instruction;

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

    pub(super) fn get_instruction(&mut self, program_counter: u16) -> Instruction {
        let mut cursor = Cursor::new(&self.ram);
        cursor.set_position(u64::from(program_counter));

        let mut buf = [0; 2];
        cursor.read_exact(&mut buf).unwrap();

        let nibbles = [
            (buf[0] & 0xF0) >> 4,
            buf[0] & 0x0F,
            (buf[1] & 0xF0) >> 4,
            buf[1] & 0x0F,
        ];

        match nibbles {
            [0x0, 0x0, 0xE, 0x0] => Instruction::ClearScreen,
            [0x1, n2, n3, n4] => Instruction::Jump {
                nnn: (u16::from(n2) << 8) + (u16::from(n3) << 4) + (u16::from(n4)),
            },
            [0x6, n2, n3, n4] => Instruction::SetRegister {
                x: n2,
                nn: (n3 << 4) + n4,
            },
            [0x7, n2, n3, n4] => Instruction::AddValueToRegister {
                x: n2,
                nn: (n3 << 4) + n4,
            },
            [0xA, n2, n3, n4] => Instruction::SetIndexRegister {
                nnn: (u16::from(n2) << 8) + (u16::from(n3) << 4) + (u16::from(n4)),
            },
            [0xD, n2, n3, n4] => Instruction::DisplayDraw {
                x: n2,
                y: n3,
                n: n4,
            },
            [n1, n2, n3, n4] => todo!("{:1X} {:1X} {:1X} {:1X}", n1, n2, n3, n4),
        }
    }

    pub(super) fn get_ram_byte(&self, address: usize) -> u8 {
        self.ram[address]
    }
}

#[cfg(test)]
mod tests {

    use std::io::Read;

    use super::*;

    #[test]
    fn memory_get_instruction() {
        let mut rom = Vec::new();
        let mut file = std::fs::File::open("./roms/IBM Logo.ch8").unwrap();
        file.read_to_end(&mut rom).unwrap();

        let mut memory = Memory::new();
        memory.load_rom(&rom).unwrap();

        let instructions = vec![
            // 00000000
            Instruction::ClearScreen,
            Instruction::SetIndexRegister { nnn: 0x22A },
            Instruction::SetRegister { x: 0x0, nn: 0x0C },
            Instruction::SetRegister { x: 0x1, nn: 0x08 },
            Instruction::DisplayDraw {
                x: 0x0,
                y: 0x1,
                n: 0xF,
            },
            Instruction::AddValueToRegister { x: 0x0, nn: 0x09 },
            Instruction::SetIndexRegister { nnn: 0x239 },
            Instruction::DisplayDraw {
                x: 0x0,
                y: 0x1,
                n: 0xF,
            },
            // 00000010
            Instruction::SetIndexRegister { nnn: 0x248 },
            Instruction::AddValueToRegister { x: 0x0, nn: 0x08 },
            Instruction::DisplayDraw {
                x: 0x0,
                y: 0x1,
                n: 0xF,
            },
            Instruction::AddValueToRegister { x: 0x0, nn: 0x04 },
            Instruction::SetIndexRegister { nnn: 0x257 },
            Instruction::DisplayDraw {
                x: 0x0,
                y: 0x1,
                n: 0xF,
            },
            Instruction::AddValueToRegister { x: 0x0, nn: 0x08 },
            Instruction::SetIndexRegister { nnn: 0x266 },
            // 00000020
            Instruction::DisplayDraw {
                x: 0x0,
                y: 0x1,
                n: 0xF,
            },
            Instruction::AddValueToRegister { x: 0x0, nn: 0x08 },
            Instruction::SetIndexRegister { nnn: 0x275 },
            Instruction::DisplayDraw {
                x: 0x0,
                y: 0x1,
                n: 0xF,
            },
            Instruction::Jump { nnn: 0x228 },
        ];

        for (index, instruction) in instructions.into_iter().enumerate() {
            assert_eq!(
                memory.get_instruction((0x200 + index * 2) as u16),
                instruction,
                "instruction {}",
                index
            );
        }
    }
}
