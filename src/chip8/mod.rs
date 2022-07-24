mod cpu;
mod display;
mod font;
pub(crate) mod keypad;
mod memory;
mod operation;
mod register;
mod timer;

use self::{
    cpu::Cpu, display::Display, font::Font, keypad::Keypad, memory::Memory, register::Register,
    timer::Timer,
};

pub(crate) struct Emulator {
    cpu: Cpu,
    delay_timer: Timer,
    display: Display,
    execute_interval: std::time::Duration,
    keypad: Keypad,
    memory: Memory,
    paused: bool,
    register: Register,
    sound_timer: Timer,
    time: std::time::Duration,
}

impl Emulator {
    pub(crate) fn new() -> Emulator {
        let mut emulator = Emulator {
            cpu: Cpu::new(),
            delay_timer: Timer::new(),
            display: Display::new(),
            execute_interval: std::time::Duration::from_secs(1) / 700,
            keypad: Keypad::new(),
            memory: Memory::new(),
            paused: true,
            register: Register::new(),
            sound_timer: Timer::new(),
            time: std::time::Duration::ZERO,
        };

        emulator
            .memory
            .load_font(Font::new().data())
            .expect("failed to load font");

        emulator
    }

    pub(crate) fn emulate(&mut self, delta: &std::time::Duration) {
        if self.paused {
            return;
        }

        self.delay_timer.tick(delta);
        self.sound_timer.tick(delta);

        let current_time = self.time;
        let target_time = self.time.saturating_add(*delta);

        let current_executions = current_time.as_micros() / self.execute_interval.as_micros();
        let target_executions = target_time.as_micros() / self.execute_interval.as_micros();
        let delta_executions = target_executions - current_executions;

        for _ in 0..delta_executions {
            self.cpu.execute(
                &mut self.register,
                &mut self.display,
                &self.keypad,
                &mut self.memory,
                &mut self.delay_timer,
                &mut self.sound_timer,
            );
        }

        self.time = target_time;
    }

    pub(crate) fn is_pixel_on(&self, x: u8, y: u8) -> bool {
        self.display.is_pixel_on(x, y)
    }

    pub(crate) fn key_pressed(&mut self, key: keypad::Key) {
        self.keypad.pressed(key);
    }

    pub(crate) fn key_released(&mut self, key: keypad::Key) {
        self.keypad.released(key);
    }

    pub(crate) fn load_rom(&mut self, rom: &[u8]) -> crate::Result<()> {
        self.paused = false;
        self.memory.load_rom(rom)
    }
}

#[cfg(test)]
mod tests {

    use std::io::Read;

    use super::{
        operation::{Operation, ADD1, CLS, JP, LD1, LDI},
        *,
    };

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
    fn parse_instruction() {
        let mut rom = Vec::new();
        let mut file = std::fs::File::open("./roms/IBM Logo.ch8").unwrap();
        file.read_to_end(&mut rom).unwrap();

        let mut memory = Memory::new();
        memory.load_rom(&rom).unwrap();

        let instructions = vec![
            // 00000000
            Operation::CLS(CLS::new()),
            Operation::LDI(LDI::new(0x22A)),
            Operation::LD1(LD1::new(0x0, 0x0C)),
            Operation::LD1(LD1::new(0x1, 0x08)),
            Operation::DisplayDraw {
                x: 0x0,
                y: 0x1,
                n: 0xF,
            },
            Operation::ADD1(ADD1::new(0x0, 0x09)),
            Operation::LDI(LDI::new(0x239)),
            Operation::DisplayDraw {
                x: 0x0,
                y: 0x1,
                n: 0xF,
            },
            // 00000010
            Operation::LDI(LDI::new(0x248)),
            Operation::ADD1(ADD1::new(0x0, 0x08)),
            Operation::DisplayDraw {
                x: 0x0,
                y: 0x1,
                n: 0xF,
            },
            Operation::ADD1(ADD1::new(0x0, 0x04)),
            Operation::LDI(LDI::new(0x257)),
            Operation::DisplayDraw {
                x: 0x0,
                y: 0x1,
                n: 0xF,
            },
            Operation::ADD1(ADD1::new(0x0, 0x08)),
            Operation::LDI(LDI::new(0x266)),
            // 00000020
            Operation::DisplayDraw {
                x: 0x0,
                y: 0x1,
                n: 0xF,
            },
            Operation::ADD1(ADD1::new(0x0, 0x08)),
            Operation::LDI(LDI::new(0x275)),
            Operation::DisplayDraw {
                x: 0x0,
                y: 0x1,
                n: 0xF,
            },
            Operation::JP(JP::new(0x228)),
        ];

        for (index, instruction) in instructions.into_iter().enumerate() {
            let byte1 = memory.get_byte((0x200 + index * 2) as u16);
            let byte2 = memory.get_byte((0x200 + index * 2 + 1) as u16);

            assert_eq!(
                Operation::parse([byte1, byte2]),
                instruction,
                "instruction {}",
                index
            );
        }
    }
}
