use crate::MEMORY_SIZE;

pub struct Memory {
    ram: [u8; MEMORY_SIZE],
}

impl Memory {
    pub fn new() -> Self {
        Memory {
            ram: [0u8; MEMORY_SIZE],
        }
    }

    pub fn read<A>(&self, addr: A) -> u8
    where
        A: Into<usize>,
    {
        let addr = addr.into();
        self.ram[addr]
    }

    pub fn write<A>(&mut self, addr: A, value: u8)
    where
        A: Into<usize>,
    {
        let addr = addr.into();
        self.ram[addr] = value;
    }

    pub fn read_u16<A>(&self, addr: A) -> u16
    where
        A: Into<usize>,
    {
        let addr = addr.into();
        ((self.ram[addr] as u16) << 8) | self.ram[addr + 1] as u16
    }

    pub fn slice<A>(&mut self, start: A, end: A) -> &[u8]
    where
        A: Into<usize>,
    {
        let start = start.into();
        let end = end.into();
        &self.ram[start..end]
    }

    pub fn load<A>(&mut self, start: A, bytes: &[u8])
    where
        A: Into<usize>,
    {
        let start = start.into();
        self.ram[start..start + bytes.len()].copy_from_slice(bytes);
    }
    // pub fn print(&self, columns: usize) {
    //     info!("DUMPED RAM:");
    //     for i in 0..MEMORY_SIZE {
    //         print!("{:03X} {:02X?} |", i, self.ram[i]);
    //         if i != 0 && i % columns == 0 {
    //             println!();
    //         }
    //     }
    //     println!();
    // }
}
