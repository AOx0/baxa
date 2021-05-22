use crate::bxvm::{heap::Heap, stack::Stack, bytecodes::*};
use crate::error;

pub struct Vm {
    pub instruction_pointer: usize,
    pub heap: Heap,
    stack: Stack,
    registers: [u128; 8]
}

impl Vm {
    pub fn new() -> Self {
        Vm {
            stack: Stack::new(),
            heap: Heap::new(),
            registers: [0x0; 8],
            instruction_pointer: 0
        }
    }
}

impl Vm {
    pub fn get_reg_heap_value(&mut self, addr: usize, bytecodes: &[u128]) -> u128 {
        match bytecodes[addr] {
            REG1 => self.registers[0],
            REG2 => self.registers[1],
            REG3 => self.registers[2],
            REG4 => self.registers[3],
            REG5 => self.registers[4],
            REG6 => self.registers[5],
            REG7 => self.registers[6],
            REG8 => self.registers[7],
            _ => bytecodes[addr]
        }
    }

    pub fn set_reg_heap_value(&mut self, addr: usize, bytecodes: &[u128], value: u128) {
        match bytecodes[addr] {
            REG1 => self.registers[0] = value,
            REG2 => self.registers[1] = value,
            REG3 => self.registers[2]= value,
            REG4 => self.registers[3] = value,
            REG5 => self.registers[4]= value,
            REG6 => self.registers[5]= value,
            REG7 => self.registers[6]= value,
            REG8 => self.registers[7]= value,
            _ => self.heap.space[addr] = value as u8,
        } 
    }

    pub fn match_bytecode(&mut self, bytecode: u128, bytecodes: &[u128]) -> bool {
        match bytecode {
            ADD => {
                self.registers[0] = bytecodes[self.instruction_pointer+1] + bytecodes[self.instruction_pointer+2];
                self.instruction_pointer += 2;
            },
            PRINT => {
                println!("{}", 
                    self.get_reg_heap_value(self.instruction_pointer+1, &bytecodes)
                );
                self.instruction_pointer += 2;
            }
            SET => {
                self.set_reg_heap_value(self.instruction_pointer+1,&bytecodes,  bytecodes[self.instruction_pointer+2])
            },
            GOTO => {
                self.instruction_pointer = bytecodes[self.instruction_pointer+1] as usize -1 ;
            },
            GOIF => {
                if self.get_reg_heap_value(self.instruction_pointer+2, &bytecodes) == self.get_reg_heap_value(self.instruction_pointer+3, &bytecodes) {
                    self.instruction_pointer = bytecodes[self.instruction_pointer+1] as usize -1 ;
                } else {
                    self.instruction_pointer += 3
                }
            },
            GOIF_ELSE => {
                if self.get_reg_heap_value(self.instruction_pointer+2, &bytecodes) == self.get_reg_heap_value(self.instruction_pointer+3, &bytecodes) {
                    self.instruction_pointer = bytecodes[self.instruction_pointer+1] as usize;
                } else {
                    self.instruction_pointer = bytecodes[self.instruction_pointer+4] as usize;
                }
            },
            MOV => {
                let value =  self.get_reg_heap_value(self.instruction_pointer+2, &bytecodes);
                println!("Value: {}", value);
                self.set_reg_heap_value(self.instruction_pointer+1, &bytecodes, value);
                self.instruction_pointer += 2;
            }, 
            STOP => {
                return true
            }
            _ => ()
        }
        return false
    }

    pub fn interpret_bytecode(&mut self, bytecode: &[u128]) -> Result<(), error::Error> {
        let mut vm = Vm::new();
        while self.instruction_pointer < bytecode.len() {
            if self.match_bytecode(bytecode[self.instruction_pointer], bytecode) {
                break;
            }
            self.instruction_pointer += 1
        }
        // vm.heap.view_memory();
        // for value in self.registers.iter() {
        //     println!("{}", value)
        // }
        Ok(())
    }
}
