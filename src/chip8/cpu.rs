use super::{
    display::Display, instruction::Instruction, keypad::Keypad, memory::Memory, register::Register,
    timer::Timer,
};

pub(super) struct Cpu;

impl Cpu {
    pub(super) fn new() -> Cpu {
        Cpu
    }

    pub(super) fn execute(
        &mut self,
        register: &mut Register,
        display: &mut Display,
        keypad: &Keypad,
        memory: &mut Memory,
        delay_timer: &mut Timer,
        sound_timer: &mut Timer,
    ) {
        let instruction = Instruction::parse([
            memory.get_byte(register.get_program_counter()),
            memory.get_byte(register.get_program_counter() + 0x1),
        ]);

        match instruction {
            Instruction::CLS(i) => i.execute(register, display),
            Instruction::RET(i) => i.execute(register),
            Instruction::SYS(i) => i.execute(),
            Instruction::JP(i) => i.execute(register),
            Instruction::CALL(i) => i.execute(register),
            Instruction::SE(i) => i.execute(register),
            Instruction::SkipIfNotEqual1 { x, nn } => {
                self.execute_skip_if_not_equal_1(x, nn, register)
            }
            Instruction::SkipIfEqual2 { x, y } => self.execute_skip_if_equal_2(x, y, register),
            Instruction::SetRegister { x, nn } => self.execute_set_register(x, nn, register),
            Instruction::AddValueToRegister { x, nn } => {
                self.execute_add_value_to_register(x, nn, register)
            }
            Instruction::Set { x, y } => self.execute_set(x, y, register),
            Instruction::BinaryAnd { x, y } => self.execute_binary_add(x, y, register),
            Instruction::BinaryOr { x, y } => self.execute_binary_or(x, y, register),
            Instruction::LogicalXor { x, y } => self.execute_logical_xor(x, y, register),
            Instruction::Add { x, y } => self.execute_and(x, y, register),
            Instruction::SubtractRightFromLeft { x, y } => {
                self.execute_subtract_right_from_left(x, y, register)
            }
            Instruction::ShiftRight { x, y } => self.execute_shift_right(x, y, register),
            Instruction::ShiftLeft { x, y } => self.execute_shift_left(x, y, register),
            Instruction::SkipIfNotEqual2 { x, y } => {
                self.execute_skip_if_not_equal_2(x, y, register)
            }
            Instruction::SetIndexRegister { nnn } => self.execute_set_index_register(nnn, register),
            Instruction::JumpWithOffset { nnn } => self.execute_jump_with_offset(nnn, register),
            Instruction::Random { x, nn } => self.execute_random(x, nn, register),
            Instruction::DisplayDraw { x, y, n } => {
                self.execute_display_draw(x, y, n, display, memory, register)
            }
            Instruction::SkipIfKeyPressed { x } => {
                self.execute_skip_if_key_pressed(x, keypad, register)
            }
            Instruction::SkipIfKeyNotPressed { x } => {
                self.execute_skip_if_key_not_pressed(x, keypad, register)
            }
            Instruction::SetCurrentDelayTimerValueToRegister { x } => {
                self.execute_self_current_delay_timer_value_to_register(x, delay_timer, register)
            }
            Instruction::GetKey { x } => self.execute_get_key(x, keypad, register),
            Instruction::SetDelayTimer { x } => {
                self.execute_set_delay_timer(x, delay_timer, register)
            }
            Instruction::SetSoundTimer { x } => {
                self.execute_set_sound_timer(x, sound_timer, register)
            }
            Instruction::AddToIndex { x } => self.execute_add_to_index(x, register),
            Instruction::LoadFont { x } => self.execute_load_font(x, register),
            Instruction::BinaryCodedDecimalConversion { x } => {
                self.execute_binary_coded_decimal_conversion(x, memory, register)
            }
            Instruction::StoreMemory { x } => self.execute_store_memory(x, memory, register),
            Instruction::LoadMemory { x } => self.execute_load_memory(x, memory, register),
        }
    }

    fn execute_skip_if_not_equal_1(&mut self, x: u8, nn: u8, register: &mut Register) {
        if register.get_v_register(x) != nn {
            register.increment_program_counter();
        }
        register.increment_program_counter();
    }

    fn execute_skip_if_equal_2(&mut self, x: u8, y: u8, register: &mut Register) {
        if register.get_v_register(x) == register.get_v_register(y) {
            register.increment_program_counter();
        }
        register.increment_program_counter();
    }

    fn execute_set_register(&mut self, x: u8, nn: u8, register: &mut Register) {
        register.set_v_register(x, nn);
        register.increment_program_counter();
    }

    fn execute_add_value_to_register(&mut self, x: u8, nn: u8, register: &mut Register) {
        let (result, _) = register.get_v_register(x).overflowing_add(nn);
        register.set_v_register(x, result);
        register.increment_program_counter();
    }

    fn execute_set(&mut self, x: u8, y: u8, register: &mut Register) {
        register.set_v_register(x, register.get_v_register(y));
        register.increment_program_counter();
    }

    fn execute_binary_add(&mut self, x: u8, y: u8, register: &mut Register) {
        register.set_v_register(x, register.get_v_register(x) & register.get_v_register(y));
        register.increment_program_counter();
    }

    fn execute_binary_or(&mut self, x: u8, y: u8, register: &mut Register) {
        register.set_v_register(x, register.get_v_register(x) | register.get_v_register(y));
        register.increment_program_counter();
    }

    fn execute_logical_xor(&mut self, x: u8, y: u8, register: &mut Register) {
        register.set_v_register(x, register.get_v_register(x) ^ register.get_v_register(y));
        register.increment_program_counter();
    }

    fn execute_and(&mut self, x: u8, y: u8, register: &mut Register) {
        let (nn, overflow) = register
            .get_v_register(x)
            .overflowing_add(register.get_v_register(y));

        register.set_v_register(x, nn);

        if overflow {
            register.set_v_register(0xF, 1);
        } else {
            register.set_v_register(0xF, 0);
        }

        register.increment_program_counter();
    }

    fn execute_subtract_right_from_left(&mut self, x: u8, y: u8, register: &mut Register) {
        if register.get_v_register(x) > register.get_v_register(y) {
            register.set_v_register(0xF, 1);
        } else {
            register.set_v_register(0xF, 0);
        }

        let (nn, _) = register
            .get_v_register(x)
            .overflowing_sub(register.get_v_register(y));

        register.set_v_register(x, nn);

        register.increment_program_counter();
    }

    fn execute_shift_right(&mut self, x: u8, y: u8, register: &mut Register) {
        // (Optional, or configurable) Set VX to the value of VY
        register.set_v_register(x, register.get_v_register(y));
        // Shift the value of VX one bit to the right
        let (nn, overflow) = register.get_v_register(x).overflowing_shr(1);
        register.set_v_register(x, nn);
        // Set VF to 1 if the bit that was shifted out was 1, or 0 if it was 0
        if overflow {
            register.set_v_register(0xF, 1);
        } else {
            register.set_v_register(0xF, 0);
        }
        register.increment_program_counter();
    }

    fn execute_shift_left(&mut self, x: u8, y: u8, register: &mut Register) {
        // (Optional, or configurable) Set VX to the value of VY
        register.set_v_register(x, register.get_v_register(y));
        // Shift the value of VX one bit to the left
        let (nn, overflow) = register.get_v_register(x).overflowing_shl(1);
        register.set_v_register(x, nn);
        // Set VF to 1 if the bit that was shifted out was 1, or 0 if it was 0
        if overflow {
            register.set_v_register(0xF, 1);
        } else {
            register.set_v_register(0xF, 0);
        }
        register.increment_program_counter();
    }

    fn execute_skip_if_not_equal_2(&mut self, x: u8, y: u8, register: &mut Register) {
        if register.get_v_register(x) != register.get_v_register(y) {
            register.increment_program_counter();
        }
        register.increment_program_counter();
    }

    fn execute_set_index_register(&mut self, nnn: u16, register: &mut Register) {
        register.set_index_register(nnn);
        register.increment_program_counter();
    }

    fn execute_jump_with_offset(&mut self, nnn: u16, register: &mut Register) {
        register.set_program_counter(nnn + u16::from(register.get_v_register(0x0)));
    }

    fn execute_random(&mut self, x: u8, nn: u8, register: &mut Register) {
        register.set_v_register(x, rand::random::<u8>() & nn);
        register.increment_program_counter();
    }

    fn execute_display_draw(
        &mut self,
        x: u8,
        y: u8,
        n: u8,
        display: &mut Display,
        memory: &mut Memory,
        register: &mut Register,
    ) {
        // Set the X coordinate to the value in VX modulo 64
        let x = register.get_v_register(x) % 64;
        // Set the Y coordinate to the value in VY modulo 32
        let y = register.get_v_register(y) % 32;

        // Set VF to 0
        register.set_v_register(0xF, 0);

        // For N rows
        for row in 0..n {
            // Get the Nth byte of sprite data, counting from the memory address in the I register
            let sprite_data = memory.get_byte(register.get_index_register() + u16::from(row));

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
                    register.set_v_register(0xF, 1);
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
        register.increment_program_counter();
    }

    fn execute_skip_if_key_pressed(&mut self, x: u8, keypad: &Keypad, register: &mut Register) {
        match keypad.read() {
            Some(key) if key == register.get_v_register(x) => register.increment_program_counter(),
            _ => {}
        }
        register.increment_program_counter();
    }

    fn execute_skip_if_key_not_pressed(&mut self, x: u8, keypad: &Keypad, register: &mut Register) {
        match keypad.read() {
            Some(key) if key == register.get_v_register(x) => {}
            _ => register.increment_program_counter(),
        }
        register.increment_program_counter();
    }

    fn execute_self_current_delay_timer_value_to_register(
        &mut self,
        x: u8,
        delay_timer: &Timer,
        register: &mut Register,
    ) {
        register.set_v_register(x, delay_timer.get());
        register.increment_program_counter();
    }

    fn execute_get_key(&mut self, x: u8, keypad: &Keypad, register: &mut Register) {
        if let Some(n) = keypad.read() {
            register.set_v_register(x, n);
            register.increment_program_counter();
        }
    }

    fn execute_set_delay_timer(&mut self, x: u8, delay_timer: &mut Timer, register: &mut Register) {
        delay_timer.set(register.get_v_register(x));
        register.increment_program_counter();
    }

    fn execute_set_sound_timer(&mut self, x: u8, sound_timer: &mut Timer, register: &mut Register) {
        sound_timer.set(register.get_v_register(x));
        register.increment_program_counter();
    }

    fn execute_add_to_index(&mut self, x: u8, register: &mut Register) {
        register.set_index_register(
            register.get_index_register() + u16::from(register.get_v_register(x)),
        );
        register.increment_program_counter();
    }

    fn execute_load_font(&mut self, x: u8, register: &mut Register) {
        register.set_index_register(0x050 + u16::from(x) * 0x5);
        register.increment_program_counter();
    }

    fn execute_binary_coded_decimal_conversion(
        &mut self,
        x: u8,
        memory: &mut Memory,
        register: &mut Register,
    ) {
        // TODO: do not rely on string conversion
        let string = format!("{:03}", register.get_v_register(x));

        for (i, c) in string.chars().enumerate() {
            memory.set_byte(
                register.get_index_register() + i as u16,
                c.to_digit(10).unwrap() as u8,
            );
        }
        register.increment_program_counter();
    }

    fn execute_store_memory(&mut self, x: u8, memory: &mut Memory, register: &mut Register) {
        for i in 0..=x {
            memory.set_byte(
                register.get_index_register() + u16::from(i),
                register.get_v_register(i),
            );
        }
        register.increment_program_counter();
    }

    fn execute_load_memory(&mut self, x: u8, memory: &Memory, register: &mut Register) {
        for i in 0..=x {
            let byte = memory.get_byte(register.get_index_register() + u16::from(i));
            register.set_v_register(x, byte);
        }
        register.increment_program_counter();
    }
}
