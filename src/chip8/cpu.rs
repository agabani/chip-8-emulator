use super::{
    display::Display, instruction::Instruction, keypad::Keypad, memory::Memory, timer::Timer,
};

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

    pub(super) fn execute(
        &mut self,
        display: &mut Display,
        keypad: &Keypad,
        memory: &mut Memory,
        delay_timer: &mut Timer,
        sound_timer: &mut Timer,
    ) {
        let instruction = Instruction::parse([
            memory.get_byte(self.get_program_counter()),
            memory.get_byte(self.get_program_counter() + 0x1),
        ]);

        match instruction {
            Instruction::ClearScreen => self.execute_clear_screen(display),
            Instruction::Return => self.execute_return(),
            Instruction::SystemAddress { nnn } => self.execute_system_address(nnn),
            Instruction::Jump { nnn } => self.execute_jump(nnn),
            Instruction::Call { nnn } => self.execute_call(nnn),
            Instruction::SkipIfEqual1 { x, nn } => self.execute_skip_if_equal_1(x, nn),
            Instruction::SkipIfNotEqual1 { x, nn } => self.execute_skip_if_not_equal_1(x, nn),
            Instruction::SkipIfEqual2 { x, y } => self.execute_skip_if_equal_2(x, y),
            Instruction::SetRegister { x, nn } => self.set_v_register(x, nn),
            Instruction::AddValueToRegister { x, nn } => self.execute_add_value_to_register(x, nn),
            Instruction::Set { x, y } => self.execute_set(x, y),
            Instruction::BinaryAnd { x, y } => self.execute_binary_add(x, y),
            Instruction::BinaryOr { x, y } => self.execute_binary_or(x, y),
            Instruction::LogicalXor { x, y } => self.execute_logical_xor(x, y),
            Instruction::Add { x, y } => self.execute_and(x, y),
            Instruction::SubtractRightFromLeft { x, y } => {
                self.execute_subtract_right_from_left(x, y)
            }
            Instruction::ShiftRight { x, y } => self.execute_shift_right(x, y),
            Instruction::ShiftLeft { x, y } => self.execute_shift_left(x, y),
            Instruction::SkipIfNotEqual2 { x, y } => self.execute_skip_if_not_equal_2(x, y),
            Instruction::SetIndexRegister { nnn } => self.execute_set_index_register(nnn),
            Instruction::JumpWithOffset { nnn } => self.execute_jump_with_offset(nnn),
            Instruction::Random { x, nn } => self.execute_random(x, nn),
            Instruction::DisplayDraw { x, y, n } => {
                self.execute_display_draw(x, y, n, display, memory)
            }
            Instruction::SkipIfKeyPressed { x } => self.execute_skip_if_key_pressed(x, keypad),
            Instruction::SkipIfKeyNotPressed { x } => {
                self.execute_skip_if_key_not_pressed(x, keypad)
            }
            Instruction::SetCurrentDelayTimerValueToRegister { x } => {
                self.execute_self_current_delay_timer_value_to_register(x, delay_timer)
            }
            Instruction::GetKey { x } => self.execute_get_key(x, keypad),
            Instruction::SetDelayTimer { x } => self.execute_set_delay_timer(x, delay_timer),
            Instruction::SetSoundTimer { x } => self.execute_set_sound_timer(x, sound_timer),
            Instruction::AddToIndex { x } => self.execute_add_to_index(x),
            Instruction::LoadFont { x } => self.execute_load_font(x),
            Instruction::BinaryCodedDecimalConversion { x } => {
                self.execute_binary_coded_decimal_conversion(x, memory)
            }
            Instruction::StoreMemory { x } => self.execute_store_memory(x, memory),
            Instruction::LoadMemory { x } => self.execute_load_memory(x, memory),
        }

        match instruction {
            Instruction::Jump { nnn: _ }
            | Instruction::Call { nnn: _ }
            | Instruction::JumpWithOffset { nnn: _ }
            | Instruction::GetKey { x: _ } => {}
            _ => self.increment_program_counter(),
        }
    }

    fn execute_clear_screen(&mut self, display: &mut Display) {
        display.clear_screen();
    }

    fn execute_return(&mut self) {
        let program_counter = self.pop_stack();
        self.set_program_counter(program_counter);
    }

    fn execute_system_address(&self, _nnn: u16) {}

    fn execute_jump(&mut self, nnn: u16) {
        self.set_program_counter(nnn);
    }

    fn execute_call(&mut self, nnn: u16) {
        self.push_stack(self.get_program_counter());
        self.set_program_counter(nnn);
    }

    fn execute_skip_if_equal_1(&mut self, x: u8, nn: u8) {
        if self.get_v_register(x) == nn {
            self.increment_program_counter();
        }
    }

    fn execute_skip_if_not_equal_1(&mut self, x: u8, nn: u8) {
        if self.get_v_register(x) != nn {
            self.increment_program_counter();
        }
    }

    fn execute_skip_if_equal_2(&mut self, x: u8, y: u8) {
        if self.get_v_register(x) == self.get_v_register(y) {
            self.increment_program_counter();
        }
    }

    fn execute_add_value_to_register(&mut self, x: u8, nn: u8) {
        let (result, _) = self.get_v_register(x).overflowing_add(nn);
        self.set_v_register(x, result);
    }

    fn execute_set(&mut self, x: u8, y: u8) {
        self.set_v_register(x, self.get_v_register(y));
    }

    fn execute_binary_add(&mut self, x: u8, y: u8) {
        self.set_v_register(x, self.get_v_register(x) & self.get_v_register(y));
    }

    fn execute_binary_or(&mut self, x: u8, y: u8) {
        self.set_v_register(x, self.get_v_register(x) | self.get_v_register(y));
    }

    fn execute_logical_xor(&mut self, x: u8, y: u8) {
        self.set_v_register(x, self.get_v_register(x) ^ self.get_v_register(y));
    }

    fn execute_and(&mut self, x: u8, y: u8) {
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

    fn execute_subtract_right_from_left(&mut self, x: u8, y: u8) {
        if self.get_v_register(x) > self.get_v_register(y) {
            self.set_v_register(0xF, 1);
        } else {
            self.set_v_register(0xF, 0);
        }

        let (nn, _) = self
            .get_v_register(x)
            .overflowing_sub(self.get_v_register(y));

        self.set_v_register(x, nn);
    }

    fn execute_shift_right(&mut self, x: u8, y: u8) {
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

    fn execute_shift_left(&mut self, x: u8, y: u8) {
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

    fn execute_skip_if_not_equal_2(&mut self, x: u8, y: u8) {
        if self.get_v_register(x) != self.get_v_register(y) {
            self.increment_program_counter();
        }
    }

    fn execute_set_index_register(&mut self, nnn: u16) {
        self.set_index_register(nnn)
    }

    fn execute_jump_with_offset(&mut self, nnn: u16) {
        self.set_program_counter(nnn + u16::from(self.get_v_register(0x0)));
    }

    fn execute_random(&mut self, x: u8, nn: u8) {
        self.set_v_register(x, rand::random::<u8>() & nn);
    }

    fn execute_display_draw(
        &mut self,
        x: u8,
        y: u8,
        n: u8,
        display: &mut Display,
        memory: &mut Memory,
    ) {
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

    fn execute_skip_if_key_pressed(&mut self, x: u8, keypad: &Keypad) {
        match keypad.read() {
            Some(key) if key == self.get_v_register(x) => self.increment_program_counter(),
            _ => {}
        }
    }

    fn execute_skip_if_key_not_pressed(&mut self, x: u8, keypad: &Keypad) {
        match keypad.read() {
            Some(key) if key == self.get_v_register(x) => {}
            _ => self.increment_program_counter(),
        }
    }

    fn execute_self_current_delay_timer_value_to_register(&mut self, x: u8, delay_timer: &Timer) {
        self.set_v_register(x, delay_timer.get());
    }

    fn execute_get_key(&mut self, x: u8, keypad: &Keypad) {
        if let Some(n) = keypad.read() {
            self.set_v_register(x, n);
            self.increment_program_counter();
        }
    }

    fn execute_set_delay_timer(&mut self, x: u8, delay_timer: &mut Timer) {
        delay_timer.set(self.get_v_register(x));
    }

    fn execute_set_sound_timer(&mut self, x: u8, sound_timer: &mut Timer) {
        sound_timer.set(self.get_v_register(x));
    }

    fn execute_add_to_index(&mut self, x: u8) {
        self.set_index_register(self.get_index_register() + u16::from(self.get_v_register(x)));
    }

    fn execute_load_font(&mut self, x: u8) {
        self.set_index_register(0x050 + u16::from(x) * 0x5);
    }

    fn execute_binary_coded_decimal_conversion(&mut self, x: u8, memory: &mut Memory) {
        // TODO: do not rely on string conversion
        let string = format!("{:03}", self.get_v_register(x));

        for (i, c) in string.chars().enumerate() {
            memory.set_byte(
                self.get_index_register() + i as u16,
                c.to_digit(10).unwrap() as u8,
            );
        }
    }

    fn execute_store_memory(&mut self, x: u8, memory: &mut Memory) {
        for i in 0..=x {
            memory.set_byte(
                self.get_index_register() + u16::from(i),
                self.get_v_register(i),
            );
        }
    }

    fn execute_load_memory(&mut self, x: u8, memory: &Memory) {
        for i in 0..=x {
            let byte = memory.get_byte(self.get_index_register() + u16::from(i));
            self.set_v_register(x, byte);
        }
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
