use crate::bxvm::{heap::Heap, stack::Stack};
use crate::error;

pub struct Vm {
    pub instruction_pointer: usize,
    pub heap: Heap,
    stack: Stack,
    registers: [u128; 8],
    code: [u8; 10000]
}

impl Vm {
    pub fn new() -> Self {
        Vm {
            stack: Stack::new(),
            heap: Heap::new(),
            registers: [0x0; 8],
            instruction_pointer: 0,
            code: [0; 10000]
        }
    }
}

/// Bytecode logic implementation
impl Vm {
    pub fn match_bytecode(&mut self, bytecode: u8) -> bool {
        
        false
    }
}

/// Interpret bytecode implementation
impl Vm {
    pub fn interpret_bytecode(&mut self, bytecode: &[u8]) -> Result<(), error::Error> {
        for (position, value) in bytecode.iter().enumerate() {
            self.code[position] = *value;
        }

        /*
        while self.instruction_pointer < bytecode.len() {
            if self.match_bytecode(self.code[self.instruction_pointer]) {
                break;
            }
            self.instruction_pointer += 1
        }
        */

        self.heap.save(&[0xAA, 0xFB, 0x98, 0xFF, 0x00, 0xFD, 0x99], 0x1)?;
        self.heap.save(&[0xAA, 0xFB, 0x98, 0xFF], 0x1)?;

        self.heap.delete(0x0000);
        self.heap.delete(0x000A);

        self.heap.view_memory();
        for value in self.registers.iter() {
            println!("{}", value)
         }
        Ok(())
    }
}
