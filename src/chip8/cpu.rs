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
            Operation::RND(o) => o.execute(register),
            Operation::DRW(o) => o.execute(register, display, memory),
            Operation::SKP(o) => o.execute(register, keypad),
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
