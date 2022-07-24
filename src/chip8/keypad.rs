#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Key {
    Key0,
    Key1,
    Key2,
    Key3,
    Key4,
    Key5,
    Key6,
    Key7,
    Key8,
    Key9,

    A,
    B,
    C,
    D,
    E,
    F,
}

pub(super) struct Keypad {
    last_key: Option<Key>,
    pressed: [bool; 0x10],
}

impl Keypad {
    pub(super) fn new() -> Keypad {
        Keypad {
            last_key: None,
            pressed: [false; 0x10],
        }
    }

    pub(crate) fn pressed(&mut self, key: Key) {
        self.last_key = Some(key);
        self.pressed[Self::map(key) as usize] = true;
    }

    pub(crate) fn released(&mut self, key: Key) {
        if let Some(current_key) = self.last_key {
            if current_key == key {
                self.last_key = None;
            }
        }
        self.pressed[Self::map(key) as usize] = false;
    }

    pub(super) fn read(&self) -> Option<u8> {
        self.last_key.map(Self::map)
    }

    pub(super) fn is_pressed(&self, key: u8) -> bool {
        self.pressed[key as usize]
    }

    fn map(key: Key) -> u8 {
        match key {
            Key::Key0 => 0x0,
            Key::Key1 => 0x1,
            Key::Key2 => 0x2,
            Key::Key3 => 0x3,
            Key::Key4 => 0x4,
            Key::Key5 => 0x5,
            Key::Key6 => 0x6,
            Key::Key7 => 0x7,
            Key::Key8 => 0x8,
            Key::Key9 => 0x9,
            Key::A => 0xA,
            Key::B => 0xB,
            Key::C => 0xC,
            Key::D => 0xD,
            Key::E => 0xE,
            Key::F => 0xF,
        }
    }
}
