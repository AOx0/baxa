use num_traits::int::PrimInt;
use crate::error::Error;

pub struct Stack {
    last_addrs: i32,
    space: [u128; 10000]
}

impl Stack {
    pub fn new() -> Self {
        Stack {
            last_addrs: -1,
            space: [0x0; 10000]
        }
    }
}


impl Stack {
    pub fn push<T: PrimInt> (&mut self, value: T) -> Result<(), Error>{
        self.last_addrs += 1;
        self.space[self.last_addrs as usize] = 
                value.to_u128().ok_or(Error::from(&format!("Stack: Failed to convert number to u128")))?;
        Ok(())
    }

    pub fn pop(&mut self) -> Result<u128, Error> {
        let index = self.last_addrs as usize;
        let value = self.space[index];
        self.space[index] = 0x0;
        self.last_addrs -= 1;
        Ok(value)
    }
}
