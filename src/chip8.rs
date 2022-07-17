use std::io::{Cursor, Read, Write};

pub(crate) struct Emulator {
    display: Display,
    memory: Memory,
    paused: bool,
}

struct Display {
    /// 64 x 32 pixels monochrome, ie. black or white
    pixels: [[bool; 64]; 32],
}

struct Font(Vec<u8>);

#[derive(Debug, PartialEq)]
enum Instruction {
    /// 00E0
    ClearScreen,
    /// 1NNN
    Jump { nnn: u16 },
    /// 6XNN
    SetRegister { x: u8, nn: u8 },
    /// 7XNN
    AddValueToRegister { x: u8, nn: u8 },
    /// ANNN
    SetIndexRegister { nnn: u16 },
    /// DXYN
    DisplayDraw { x: u8, y: u8, n: u8 },
}

struct Memory {
    /// 16-bit index register called "I" which is used to point at locations in memory
    i: u16,
    /// A program counter, often called just "PC", which points to the current instruction in memory
    program_counter: u16,
    /// Memory: CHIP-8 has direct access to up to 4 kilobytes of RAM
    ram: Vec<u8>,
    /// A stack for 16-bit addresses which is used to call subroutines/function and return from them
    stack: Vec<u16>,
    /// 16 8-bit general purpose variable registers numbered `0` through `F`, called `V0` through `VF`
    v: [u8; 16],
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
            .load_font(&Font::new().0)
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
            Instruction::Jump { nnn } => self.memory.program_counter = nnn,
            Instruction::SetRegister { x, nn } => self.memory.v[x as usize] = nn,
            Instruction::AddValueToRegister { x, nn } => self.memory.v[x as usize] += nn,
            Instruction::SetIndexRegister { nnn } => self.memory.i = nnn,
            Instruction::DisplayDraw { x, y, n } => {
                // Set the X coordinate to the value in VX modulo 64
                let x = self.memory.v[x as usize] % 64;
                // Set the Y coordinate to the value in VY modulo 32
                let y = self.memory.v[y as usize] % 32;

                // Set VF to 0
                self.memory.v[0xF] = 0;

                // For N rows
                for row in 0..n {
                    // Get the Nth byte of sprite data, counting from the memory address in the I register
                    let sprite_data = self.memory.ram[self.memory.i as usize + row as usize];

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
                            self.memory.v[0xF] = 1;
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

impl Display {
    fn new() -> Display {
        Display {
            pixels: [[false; 64]; 32],
        }
    }

    fn clear_screen(&mut self) {
        self.pixels = [[false; 64]; 32];
    }

    fn is_pixel_on(&self, x: u8, y: u8) -> bool {
        self.pixels[y as usize][x as usize]
    }

    fn set_pixel(&mut self, x: u8, y: u8, value: bool) {
        self.pixels[y as usize][x as usize] = value;
    }
}

impl Font {
    fn new() -> Font {
        Font(vec![
            0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
            0x20, 0x60, 0x20, 0x20, 0x70, // 1
            0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
            0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
            0x90, 0x90, 0xF0, 0x10, 0x10, // 4
            0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
            0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
            0xF0, 0x10, 0x20, 0x40, 0x40, // 7
            0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
            0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
            0xF0, 0x90, 0xF0, 0x90, 0x90, // A
            0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
            0xF0, 0x80, 0x80, 0x80, 0xF0, // C
            0xE0, 0x90, 0x90, 0x90, 0xE0, // D
            0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
            0xF0, 0x80, 0xF0, 0x80, 0x80, // F
        ])
    }
}

impl Memory {
    fn new() -> Memory {
        Memory {
            i: 0,
            program_counter: 0,
            ram: vec![0; 4096],
            stack: Vec::new(),
            v: [0; 16],
        }
    }

    fn load_font(&mut self, font: &[u8]) -> crate::Result<()> {
        let mut cursor = Cursor::new(&mut self.ram);
        cursor.set_position(0x050);
        cursor.write_all(font)?;
        Ok(())
    }

    fn load_rom(&mut self, rom: &[u8]) -> crate::Result<()> {
        let mut cursor = Cursor::new(&mut self.ram);
        cursor.set_position(0x200);
        self.program_counter = 0x200;
        cursor.write_all(rom)?;
        Ok(())
    }

    fn get_instruction(&mut self) -> Instruction {
        let mut cursor = Cursor::new(&self.ram);
        cursor.set_position(u64::from(self.program_counter));

        let mut buf = [0; 2];
        cursor.read_exact(&mut buf).unwrap();

        let nibbles = [
            (buf[0] & 0xF0) >> 4,
            buf[0] & 0x0F,
            (buf[1] & 0xF0) >> 4,
            buf[1] & 0x0F,
        ];

        self.program_counter = cursor.position() as u16;

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
                memory.get_instruction(),
                instruction,
                "instruction {}",
                index
            );
        }
    }
}
