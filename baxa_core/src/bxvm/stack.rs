pub struct Stack {
    last_addrs: i32,
    space: [u8; 0xFFFF]
}

impl Stack {
    pub fn new() -> Self {
        Stack {
            last_addrs: -1,
            space: [0x0; 0xFFFF]
        }
    }
}


impl Stack {
    
}
