use crate::{DISPLAY_HEIGHT, DISPLAY_WIDTH};

pub trait Ch8Display {
    /// Implementor must provide mutable access to the display buffer
    fn buffer(&mut self) -> &mut [[bool; DISPLAY_WIDTH]; DISPLAY_HEIGHT];

    fn clear(&mut self);

    fn render(&self);

    /// Default CHIP-8 sprite drawing (XOR + collision)
    fn draw_sprite(&mut self, x: u8, y: u8, sprite: &[u8]) -> bool {
        let buffer = self.buffer();
        let mut collision = false;

        for (row, byte) in sprite.iter().enumerate() {
            let py = (y as usize + row) % DISPLAY_HEIGHT;

            for bit in 0..8 {
                let px = (x as usize + bit) % DISPLAY_WIDTH;

                if (byte & (0x80 >> bit)) != 0 {
                    let pixel = &mut buffer[py][px];

                    if *pixel {
                        collision = true;
                    }

                    *pixel ^= true;
                }
            }
        }

        collision
    }
}
