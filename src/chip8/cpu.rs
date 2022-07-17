use super::{display::Display, instruction::Instruction, memory::Memory};

pub(super) struct Cpu {
    /// 16-bit index register called "I" which is used to point at locations in memory
    i: u16,
    /// A program counter, often called just "PC", which points to the current instruction in memory
    program_counter: u16,
    /// A stack for 16-bit addresses which is used to call subroutines/function and return from them
    stack: Vec<u16>,
    /// 16 8-bit general purpose variable registers numbered `0` through `F`, called `V0` through `VF`
    v: [u8; 16],
}

impl Cpu {
    pub(super) fn new() -> Cpu {
        Cpu {
            i: 0,
            program_counter: 0x200,
            stack: Vec::new(),
            v: [0; 16],
        }
    }

    pub(super) fn execute(&mut self, display: &mut Display, memory: &mut Memory) {
        // TODO: add conditional program counter increment based on instruction
        let program_counter = self.program_counter;
        self.program_counter += 2;
        match self.fetch_instruction(&memory, program_counter) {
            Instruction::ClearScreen => display.clear_screen(),
            Instruction::Jump { nnn } => self.set_program_counter(nnn),
            Instruction::SetRegister { x, nn } => self.set_v_register(x, nn),
            Instruction::AddValueToRegister { x, nn } => {
                self.set_v_register(x, self.get_v_register(x) + nn)
            }
            Instruction::SetIndexRegister { nnn } => self.set_index_register(nnn),
            Instruction::DisplayDraw { x, y, n } => {
                // Set the X coordinate to the value in VX modulo 64
                let x = self.get_v_register(x) % 64;
                // Set the Y coordinate to the value in VY modulo 32
                let y = self.get_v_register(y) % 32;

                // Set VF to 0
                self.set_v_register(0xF, 0);

                // For N rows
                for row in 0..n {
                    // Get the Nth byte of sprite data, counting from the memory address in the I register
                    let sprite_data = memory.get_byte(self.get_index_register() + u16::from(row));

                    // For each of the 8 pixels/bits in this sprite row
                    for pixel in 0..8 {
                        let sprite_row_pixel = match (sprite_data >> (7 - pixel)) & 0x1 {
                            1 => true,
                            0 => false,
                            v => panic!("{}", v),
                        };
                        let display_pixel = display.is_pixel_on(x + pixel, y + n - row);

                        // If the current pixel in the sprite row is on and the pixel at coordinates X,Y on the screen is also on
                        if sprite_row_pixel && display_pixel {
                            // turn off the pixel
                            display.set_pixel(x + pixel, y + n - row, false);
                            // set VF to 1
                            self.set_v_register(0xF, 1);
                        }
                        // Or if the current pixel in the sprite row is on and the screen pixel is not
                        else if sprite_row_pixel && !display_pixel {
                            // draw the pixel at the X and Y coordinates
                            display.set_pixel(x + pixel, y + n - row, sprite_row_pixel);
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

    fn fetch_instruction(&self, memory: &Memory, program_counter: u16) -> Instruction {
        let byte1 = memory.get_byte(program_counter);
        let byte2 = memory.get_byte(program_counter + 0x1);
        Instruction::parse([byte1, byte2])
    }

    fn set_program_counter(&mut self, program_counter: u16) {
        self.program_counter = program_counter;
    }

    fn get_v_register(&self, x: u8) -> u8 {
        self.v[x as usize]
    }

    fn set_v_register(&mut self, x: u8, nn: u8) {
        self.v[x as usize] = nn;
    }

    fn get_index_register(&self) -> u16 {
        self.i
    }

    fn set_index_register(&mut self, nnn: u16) {
        self.i = nnn;
    }
}
