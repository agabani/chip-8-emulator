use super::{
    display::Display, keypad::Keypad, memory::Memory, operation::Operation, register::Register,
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
        let instruction = Operation::parse([
            memory.get_byte(register.get_program_counter()),
            memory.get_byte(register.get_program_counter() + 0x1),
        ]);

        match instruction {
            Operation::CLS(o) => o.execute(register, display),
            Operation::RET(o) => o.execute(register),
            Operation::SYS(o) => o.execute(),
            Operation::JP(o) => o.execute(register),
            Operation::CALL(o) => o.execute(register),
            Operation::SE1(o) => o.execute(register),
            Operation::SNE1(o) => o.execute(register),
            Operation::SE2(o) => o.execute(register),
            Operation::LD1(o) => o.execute(register),
            Operation::ADD1(o) => o.execute(register),
            Operation::LD2(o) => o.execute(register),
            Operation::OR(o) => o.execute(register),
            Operation::AND2(o) => o.execute(register),
            Operation::XOR(o) => o.execute(register),
            Operation::ADD2(o) => o.execute(register),
            Operation::SUB(o) => o.execute(register),
            Operation::SHR(o) => o.execute(register),
            Operation::SUBN(o) => o.execute(register),
            Operation::SHL(o) => o.execute(register),
            Operation::SNE2(o) => o.execute(register),
            Operation::LDI(o) => o.execute(register),
            Operation::JPV0(o) => o.execute(register),
            Operation::Random { x, nn } => self.execute_random(x, nn, register),
            Operation::DisplayDraw { x, y, n } => {
                self.execute_display_draw(x, y, n, display, memory, register)
            }
            Operation::SkipIfKeyPressed { x } => {
                self.execute_skip_if_key_pressed(x, keypad, register)
            }
            Operation::SkipIfKeyNotPressed { x } => {
                self.execute_skip_if_key_not_pressed(x, keypad, register)
            }
            Operation::SetCurrentDelayTimerValueToRegister { x } => {
                self.execute_self_current_delay_timer_value_to_register(x, delay_timer, register)
            }
            Operation::GetKey { x } => self.execute_get_key(x, keypad, register),
            Operation::SetDelayTimer { x } => {
                self.execute_set_delay_timer(x, delay_timer, register)
            }
            Operation::SetSoundTimer { x } => {
                self.execute_set_sound_timer(x, sound_timer, register)
            }
            Operation::AddToIndex { x } => self.execute_add_to_index(x, register),
            Operation::LoadFont { x } => self.execute_load_font(x, register),
            Operation::BinaryCodedDecimalConversion { x } => {
                self.execute_binary_coded_decimal_conversion(x, memory, register)
            }
            Operation::StoreMemory { x } => self.execute_store_memory(x, memory, register),
            Operation::LoadMemory { x } => self.execute_load_memory(x, memory, register),
        }
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
