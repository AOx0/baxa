use num_traits::PrimInt;
use crate::error::Error;
use std::mem::{transmute};

pub struct Heap {
    pub space: [u8; 0xFFFF]
}

impl Heap {
    /// New instance of Heap, 65535 addresses of 8 bits available
    pub fn new() -> Self {
        Heap {
            space: [0x00; 0xFFFF]
        }
    }

    /// Print all memory contents 
    pub fn view_memory(&mut self) {
        const PIECE_SIZE: usize = 17;
        let mut index = 0;
        while index  < self.space.len() {
            for i in 0..PIECE_SIZE {
                print!("{:01$x} ", self.space[index+i], 2)
            }
            print!("\n");
            index += PIECE_SIZE;
        }
    }
}

impl Heap {
    pub fn save<T: PrimInt>(&mut self, data: T, addr: usize) -> Result<usize, Error>{
        let data = data.to_u128().ok_or(Error::from("HEAP: Failed to convert to u128"))?;
        self.space[addr] = 16;
        let data = unsafe { transmute::<u128, [u8; 16]>(data.to_be()) };
        for (n, value) in data.iter().enumerate() {
            self.space[addr + n + 1] = *value;
        }
        Ok(addr)
    }
}
