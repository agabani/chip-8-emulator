use super::{display::Display, instruction::Instruction, memory::Memory};

pub(super) struct Cpu {
    delay_timer: u8,
    /// 16-bit index register called "I" which is used to point at locations in memory
    i: u16,
    /// A program counter, often called just "PC", which points to the current instruction in memory
    program_counter: u16,
    sound_timer: u8,
    /// A stack for 16-bit addresses which is used to call subroutines/function and return from them
    stack: Vec<u16>,
    /// 16 8-bit general purpose variable registers numbered `0` through `F`, called `V0` through `VF`
    v: [u8; 16],
}

impl Cpu {
    pub(super) fn new() -> Cpu {
        Cpu {
            delay_timer: 0,
            i: 0,
            program_counter: 0x200,
            sound_timer: 0,
            stack: Vec::new(),
            v: [0; 16],
        }
    }

    pub(super) fn execute(&mut self, display: &mut Display, memory: &mut Memory) {
        let instruction = Instruction::parse([
            memory.get_byte(self.get_program_counter()),
            memory.get_byte(self.get_program_counter() + 0x1),
        ]);

        match instruction {
            Instruction::ClearScreen => display.clear_screen(),
            Instruction::Return => {
                let program_counter = self.pop_stack();
                self.set_program_counter(program_counter)
            }
            Instruction::Jump { nnn } => self.set_program_counter(nnn),
            Instruction::Call { nnn } => {
                self.push_stack(self.get_program_counter());
                self.set_program_counter(nnn);
            }
            Instruction::SkipIfEqual1 { x, nn } => {
                if self.get_v_register(x) == nn {
                    self.increment_program_counter();
                }
            }
            Instruction::SkipIfNotEqual1 { x, nn } => {
                if self.get_v_register(x) != nn {
                    self.increment_program_counter();
                }
            }
            Instruction::SkipIfEqual2 { x, y } => {
                if self.get_v_register(x) == self.get_v_register(y) {
                    self.increment_program_counter();
                }
            }
            Instruction::SetRegister { x, nn } => self.set_v_register(x, nn),
            Instruction::AddValueToRegister { x, nn } => {
                let (result, _) = self.get_v_register(x).overflowing_add(nn);
                self.set_v_register(x, result)
            }
            Instruction::Set { x, y } => self.set_v_register(x, self.get_v_register(y)),
            Instruction::BinaryAnd { x, y } => {
                self.set_v_register(x, self.get_v_register(x) & self.get_v_register(y))
            }
            Instruction::BinaryOr { x, y } => {
                self.set_v_register(x, self.get_v_register(x) | self.get_v_register(y))
            }
            Instruction::LogicalXor { x, y } => {
                self.set_v_register(x, self.get_v_register(x) ^ self.get_v_register(y))
            }
            Instruction::Add { x, y } => {
                let (nn, overflow) = self
                    .get_v_register(x)
                    .overflowing_add(self.get_v_register(y));

                self.set_v_register(x, nn);

                if overflow {
                    self.set_v_register(0xF, 1);
                } else {
                    self.set_v_register(0xF, 0);
                }
            }
            Instruction::SubtractRightFromLeft { x, y } => {
                let (nn, overflow) = self
                    .get_v_register(x)
                    .overflowing_sub(self.get_v_register(y));
                if overflow && self.get_v_register(x) > self.get_v_register(y) {
                    self.set_v_register(0xF, 1);
                } else {
                    self.set_v_register(0xF, 0);
                }
                self.set_v_register(x, nn);
            }
            Instruction::ShiftRight { x, y } => {
                // (Optional, or configurable) Set VX to the value of VY
                self.set_v_register(x, self.get_v_register(y));
                // Shift the value of VX one bit to the right
                let (nn, overflow) = self.get_v_register(x).overflowing_shr(1);
                self.set_v_register(x, nn);
                // Set VF to 1 if the bit that was shifted out was 1, or 0 if it was 0
                if overflow {
                    self.set_v_register(0xF, 1);
                } else {
                    self.set_v_register(0xF, 0);
                }
            }
            Instruction::ShiftLeft { x, y } => {
                // (Optional, or configurable) Set VX to the value of VY
                self.set_v_register(x, self.get_v_register(y));
                // Shift the value of VX one bit to the left
                let (nn, overflow) = self.get_v_register(x).overflowing_shl(1);
                self.set_v_register(x, nn);
                // Set VF to 1 if the bit that was shifted out was 1, or 0 if it was 0
                if overflow {
                    self.set_v_register(0xF, 1);
                } else {
                    self.set_v_register(0xF, 0);
                }
            }
            Instruction::SkipIfNotEqual2 { x, y } => {
                if self.get_v_register(x) != self.get_v_register(y) {
                    self.increment_program_counter();
                }
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
                        let display_pixel = display.is_pixel_on(x + pixel, y + row);

                        // If the current pixel in the sprite row is on and the pixel at coordinates X,Y on the screen is also on
                        if sprite_row_pixel && display_pixel {
                            // turn off the pixel
                            display.set_pixel(x + pixel, y + row, false);
                            // set VF to 1
                            self.set_v_register(0xF, 1);
                        }
                        // Or if the current pixel in the sprite row is on and the screen pixel is not
                        else if sprite_row_pixel && !display_pixel {
                            // draw the pixel at the X and Y coordinates
                            display.set_pixel(x + pixel, y + row, sprite_row_pixel);
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
            Instruction::SetCurrentDelayTimerValueToRegister { x } => {
                self.set_v_register(x, self.get_delay_timer())
            }
            Instruction::GetKey { x: _ } => {
                // TODO: read key
            }
            Instruction::SetDelayTimer { x } => {
                self.set_delay_timer(self.get_v_register(x));
                // TODO: build timer decrementing functionality then remove the code below
                self.set_delay_timer(0);
            }
            Instruction::SetSoundTimer { x } => {
                self.set_sound_timer(self.get_v_register(x));
                // TODO: build timer decrementing functionality then remove the code below
                self.set_sound_timer(0);
            }
            Instruction::AddToIndex { x } => self
                .set_index_register(self.get_index_register() + u16::from(self.get_v_register(x))),
            Instruction::BinaryCodedDecimalConversion { x } => {
                let n = self.get_v_register(x);
                let d3 = n % 10;
                let n = n / 10;
                let d2 = n % 10;
                let n = n / 10;
                let d1 = n;
                memory.set_byte(self.get_index_register() + 0x0, d1);
                memory.set_byte(self.get_index_register() + 0x1, d2);
                memory.set_byte(self.get_index_register() + 0x2, d3);
            }
            Instruction::StoreMemory { x } => {
                for i in 0..=x {
                    memory.set_byte(
                        self.get_index_register() + u16::from(i),
                        self.get_v_register(i),
                    )
                }
            }
            Instruction::LoadMemory { x } => {
                for i in 0..=x {
                    let byte = memory.get_byte(self.get_index_register() + u16::from(i));
                    self.set_v_register(x, byte);
                }
            }
        }

        match instruction {
            Instruction::Jump { nnn: _ } => {}
            Instruction::Call { nnn: _ } => {}
            Instruction::GetKey { x: _ } => {}
            _ => self.increment_program_counter(),
        }
    }

    fn get_delay_timer(&self) -> u8 {
        self.delay_timer
    }

    fn set_delay_timer(&mut self, n: u8) {
        self.delay_timer = n;
    }

    fn set_sound_timer(&mut self, n: u8) {
        self.sound_timer = n;
    }

    fn get_program_counter(&self) -> u16 {
        self.program_counter
    }

    fn set_program_counter(&mut self, program_counter: u16) {
        self.program_counter = program_counter;
    }

    fn increment_program_counter(&mut self) {
        self.program_counter += 2;
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

    fn push_stack(&mut self, nnn: u16) {
        self.stack.push(nnn);
    }

    fn pop_stack(&mut self) -> u16 {
        self.stack.pop().expect("failed to pop stack")
    }
}
