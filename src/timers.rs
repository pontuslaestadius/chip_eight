pub struct Timers {
    delay: u8,
    sound: u8,
}

impl Timers {
    /// Create new timers (both zeroed)
    pub fn new() -> Self {
        Timers { delay: 0, sound: 0 }
    }

    /// Set delay timer
    pub fn set_delay(&mut self, value: u8) {
        self.delay = value;
    }

    /// Get delay timer
    pub fn get_delay(&self) -> u8 {
        self.delay
    }

    /// Set sound timer
    pub fn set_sound(&mut self, value: u8) {
        self.sound = value;
    }

    /// Get sound timer
    pub fn get_sound(&self) -> u8 {
        self.sound
    }

    /// Decrement both timers by 1 if they are greater than 0
    /// Should be called at ~60Hz
    pub fn tick(&mut self) {
        if self.delay > 0 {
            self.delay -= 1;
        }

        if self.sound > 0 {
            self.sound -= 1;
        }
    }

    /// Returns true if sound timer is still active (>0)
    pub fn is_sound_active(&self) -> bool {
        self.sound > 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initial_state() {
        let timers = Timers::new();
        assert_eq!(timers.get_delay(), 0);
        assert_eq!(timers.get_sound(), 0);
        assert!(!timers.is_sound_active());
    }

    #[test]
    fn test_set_get() {
        let mut timers = Timers::new();
        timers.set_delay(10);
        timers.set_sound(5);

        assert_eq!(timers.get_delay(), 10);
        assert_eq!(timers.get_sound(), 5);
        assert!(timers.is_sound_active());
    }

    #[test]
    fn test_tick_decrements() {
        let mut timers = Timers::new();
        timers.set_delay(3);
        timers.set_sound(2);

        timers.tick();
        assert_eq!(timers.get_delay(), 2);
        assert_eq!(timers.get_sound(), 1);

        timers.tick();
        assert_eq!(timers.get_delay(), 1);
        assert_eq!(timers.get_sound(), 0);
        assert!(!timers.is_sound_active());

        timers.tick();
        assert_eq!(timers.get_delay(), 0);
        assert_eq!(timers.get_sound(), 0);
    }

    #[test]
    fn test_tick_does_not_underflow() {
        let mut timers = Timers::new();
        timers.tick(); // delay = 0, sound = 0
        assert_eq!(timers.get_delay(), 0);
        assert_eq!(timers.get_sound(), 0);

        // Tick multiple times
        for _ in 0..10 {
            timers.tick();
        }
        assert_eq!(timers.get_delay(), 0);
        assert_eq!(timers.get_sound(), 0);
    }

    #[test]
    fn test_is_sound_active() {
        let mut timers = Timers::new();
        assert!(!timers.is_sound_active());

        timers.set_sound(1);
        assert!(timers.is_sound_active());

        timers.tick();
        assert!(!timers.is_sound_active());
    }
}
