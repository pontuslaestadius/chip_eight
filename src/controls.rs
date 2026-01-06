use std::collections::HashMap;

//
// CHIP-8 key type
//
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Chip8Key(u8);

impl Chip8Key {
    pub fn new(n: u8) -> Option<Self> {
        (n < 16).then_some(Self(n))
    }

    pub fn as_u8(self) -> u8 {
        self.0
    }

    pub fn as_usize(self) -> usize {
        self.0 as usize
    }
}

//
// Key mapping (char -> CHIP-8 key)
//
pub struct KeyMap {
    bindings: HashMap<char, Chip8Key>,
}

impl KeyMap {
    pub fn default() -> Self {
        use Chip8Key as K;

        let mut bindings = HashMap::new();

        // 1 2 3 C
        bindings.insert('1', K::new(0x1).unwrap());
        bindings.insert('2', K::new(0x2).unwrap());
        bindings.insert('3', K::new(0x3).unwrap());
        bindings.insert('4', K::new(0xC).unwrap());

        // 4 5 6 D
        bindings.insert('q', K::new(0x4).unwrap());
        bindings.insert('w', K::new(0x5).unwrap());
        bindings.insert('e', K::new(0x6).unwrap());
        bindings.insert('r', K::new(0xD).unwrap());

        // 7 8 9 E
        bindings.insert('a', K::new(0x7).unwrap());
        bindings.insert('s', K::new(0x8).unwrap());
        bindings.insert('d', K::new(0x9).unwrap());
        bindings.insert('f', K::new(0xE).unwrap());

        // A 0 B F
        bindings.insert('z', K::new(0xA).unwrap());
        bindings.insert('x', K::new(0x0).unwrap());
        bindings.insert('c', K::new(0xB).unwrap());
        bindings.insert('v', K::new(0xF).unwrap());

        Self { bindings }
    }

    pub fn lookup(&self, key: char) -> Option<Chip8Key> {
        self.bindings.get(&key).copied()
    }
}

//
// Keypad state (what the CPU sees)
//
pub struct Keypad {
    keys: [bool; 16],
    last_pressed: Option<Chip8Key>,
    pub keymap: KeyMap,
}

impl Keypad {
    pub fn new() -> Self {
        Self {
            keys: [false; 16],
            last_pressed: None,
            keymap: KeyMap::default(),
        }
    }

    pub fn clear(&mut self) {
        self.keys = [false; 16];
        self.last_pressed = None;
    }

    pub fn press(&mut self, key: Chip8Key) {
        self.keys[key.as_usize()] = true;
        self.last_pressed = Some(key);
    }

    pub fn release(&mut self, key: Chip8Key) {
        self.keys[key.as_usize()] = false;
    }

    pub fn is_pressed(&self, key: Chip8Key) -> bool {
        self.keys[key.as_usize()]
    }

    // For Fx0A
    pub fn take_last_pressed(&mut self) -> Option<Chip8Key> {
        self.last_pressed.take()
    }

    pub fn lookup(&self, key: char) -> Option<Chip8Key> {
        self.keymap.bindings.get(&key).copied()
    }
}
