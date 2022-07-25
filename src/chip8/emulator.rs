use super::{
    cpu::Cpu, display::Display, font::Font, keypad::Keypad, memory::Memory, register::Register,
    timer::Timer,
};

pub(crate) struct Emulator {
    beeping: bool,
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

#[cfg(feature = "editor")]
pub(crate) struct Debug {
    pub(crate) delay_timer: u8,
    pub(crate) memory_ram: Vec<u8>,
    pub(crate) register_i: u16,
    pub(crate) register_program_counter: u16,
    pub(crate) register_stack: Vec<u16>,
    pub(crate) register_v: Vec<u8>,
    pub(crate) sound_timer: u8,
}

impl Emulator {
    pub(crate) fn new() -> Emulator {
        let mut emulator = Emulator {
            beeping: false,
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

        let b1 = self.sound_timer.get();
        self.delay_timer.tick(delta);
        self.sound_timer.tick(delta);
        let b2 = self.sound_timer.get();

        if b2 > 0 && b1 != b2 {
            self.beeping = true;
        } else {
            self.beeping = false;
        }

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

    pub(crate) fn is_beeping(&self) -> bool {
        self.beeping
    }

    pub(crate) fn is_pixel_on(&self, x: u8, y: u8) -> bool {
        self.display.is_pixel_on(x, y)
    }

    pub(crate) fn key_pressed(&mut self, key: super::keypad::Key) {
        self.keypad.pressed(key);
    }

    pub(crate) fn key_released(&mut self, key: super::keypad::Key) {
        self.keypad.released(key);
    }

    pub(crate) fn load_rom(&mut self, rom: &[u8]) -> crate::Result<()> {
        self.paused = false;
        self.memory.load_rom(rom)
    }

    #[cfg(feature = "editor")]
    pub(crate) fn get_debug(&self) -> Debug {
        Debug {
            delay_timer: self.delay_timer.get(),
            memory_ram: self.memory.get_ram().into(),
            register_i: self.register.get_i(),
            register_program_counter: self.register.get_program_counter(),
            register_stack: self.register.get_stack().into(),
            register_v: (0..=0xF)
                .into_iter()
                .map(|x| self.register.get_v(x))
                .collect(),
            sound_timer: self.sound_timer.get(),
        }
    }

    #[cfg(feature = "editor")]
    pub(crate) fn step_execute(&mut self) {
        self.cpu.execute(
            &mut self.register,
            &mut self.display,
            &self.keypad,
            &mut self.memory,
            &mut self.delay_timer,
            &mut self.sound_timer,
        );
    }

    #[cfg(feature = "editor")]
    pub(crate) fn zero_delay(&mut self) {
        self.delay_timer.set(0);
    }

    #[cfg(feature = "editor")]
    pub(crate) fn zero_sound(&mut self) {
        self.sound_timer.set(0);
    }
}
