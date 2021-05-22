use crate::error::Error;

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

    /// Delete the value and its metadata from heap.
    pub fn delete(&mut self, address: usize)  {
        const DATA_TYPE_OFFSET: usize = 1;
        let size = self.space[address];
        for index in address..=address + (size as usize) + DATA_TYPE_OFFSET {
            self.space[index] = 0;
        }
    }

    /// Saves the given data &[u8] to any available space in the heap.
    pub fn save(&mut self, value: &[u8], data_type: u8) -> Result<(),Error> {
        let size = value.len();
        let addr = self.search_space(size)?;

        if size > 0xFF {
            return Err(
                Error::from(&format!("Data value size '{:?}' exceeds 8bit space bits storage", value))
            )
        };

        self.space[addr] = size as u8;
        self.space[addr+1] = data_type;
        for index in 0..size {
            self.space[addr+index+2] = value[index];
            println!("{:x}", value[index]);
        };
        Ok(())
    }

    /// Looks for available space in the heap to store.
    /// 
    /// [u8] -> value byte size |
    /// [u8] -> value data type | 
    /// [u8; ...] -> value
    /// 
    /// Returns a pointer to heap's available space within a usize. 
    pub fn search_space(&mut self, len: usize) -> Result<usize, Error> {
        let mut available_space: i32;
        for (mut index, _) in self.space.iter().enumerate() {
            available_space = 0;
            for offset in 0..len+2 {
                if self.space[index+offset] == 0x00 {
                    available_space += 1;
                } else {
                    index += len;
                    break;
                }
            }

            if available_space-2 == len as i32 {
                if index == 0 { return Ok(index); }
                else if self.space[index-1] != 0 {
                    return Ok(index+1)
                }
            }
        }
        Err(Error::from("Heap: No available memory in heap"))
    }
}
