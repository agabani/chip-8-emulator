pub(super) struct Timer {
    remaining: std::time::Duration,
}

impl Timer {
    pub(super) fn new() -> Timer {
        Timer {
            remaining: std::time::Duration::ZERO,
        }
    }

    #[allow(clippy::cast_possible_truncation)]
    pub(super) fn get(&self) -> u8 {
        let seconds = self.remaining.as_millis() / 1_000;
        let remainder = self.remaining.as_millis() % 1_000;

        if remainder == 0 {
            seconds as u8
        } else {
            seconds as u8 + 1
        }
    }

    pub(super) fn set(&mut self, seconds: u8) {
        self.remaining = std::time::Duration::from_secs(u64::from(seconds));
    }

    pub(super) fn tick(&mut self, duration: &std::time::Duration) {
        self.remaining = self.remaining.saturating_sub(*duration);
    }
}
