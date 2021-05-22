use std::mem::{transmute};

pub const ADD:   u8 = 0x01;
pub const SUB:   u8 = 0x02;
pub const MUL:   u8 = 0x03;
pub const DIV:   u8 = 0x04;
pub const STOP:  u8 = 0x05;
pub const PRINT: u8 = 0x06;
pub const GOTO:  u8 = 0x07;
pub const GOIF:  u8 = 0x08;
pub const GOIF_ELSE: u8 = 0x09;
pub const MOV:   u8 = 0xA;
pub const SET: u8 = 0xB;

pub const REG1: u8 = 0x11;
pub const REG2: u8 = 0x12;
pub const REG3: u8 = 0x13;
pub const REG4: u8 = 0x14;
pub const REG5: u8 = 0x15;
pub const REG6: u8 = 0x16;
pub const REG7: u8 = 0x17;
pub const REG8: u8 = 0x18;

pub const NUMBER: u8 = 0x01;
pub const FLOAT: u8 = 0x02;
pub const STRING: u8 = 0x03;
pub const BOOL: u8 = 0x04;

pub enum Value {
    BxStr(String),
    BxInt(i128),
    BxFloat(f64)
}

pub enum Bytecode {
    Instruction(u8),
    Data(Value)
}

impl Value {
    pub fn make_binary(&self) -> Vec<u8> {
        match self {
            Value::BxStr(value) => {
                let mut result = vec![];
                result.push(0x00);      // Size byte
                result.push(STRING);    // Data type byte
                let mut converted_value = value.as_bytes().to_vec();
                result[0] = converted_value.len() as u8;
                result.append(&mut converted_value);

                return result
            },
            Value::BxInt(value) => {
                unsafe {
                    let mut result = vec![];
                    result.push(0x00);      // Size byte
                    result.push(NUMBER);    // Data type byte
                    let mut converted_value = transmute::<i128, [u8; 16]>(*value).to_vec();
                    result[0] = converted_value.len() as u8;
                    result.append(&mut converted_value);
                    
                    return result
                }
            },
            Value::BxFloat(value) => {
                unsafe {
                    let mut result = vec![];
                    result.push(0x00);      // Size byte
                    result.push(FLOAT);    // Data type byte
                    let mut converted_value = transmute::<f64, [u8; 8]>(*value).to_vec();
                    result[0] = converted_value.len() as u8;
                    result.append(&mut converted_value);
                    
                    return result
                }
            }
        };
    }
}

pub fn compile(bytecode: &[Bytecode]) -> Vec<u8> {
    let mut result = Vec::with_capacity(100000);
    for byte in bytecode {
        let mut new_contents = match byte {
            Bytecode::Data(value) => (*value.make_binary()).to_vec(),
            Bytecode::Instruction(value) => vec![*value]
        };

        result.append(
            &mut new_contents
        )
    };  
    result
}