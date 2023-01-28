use core::panic;
use std::{fs::File, io::Read};
use flate2::read::GzDecoder;

use crate::fbx_reader::*;

pub enum Value {
    Bool(bool),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    F32(f32),
    F64(f64),
    VecU8(Vec<u8>),
}

pub struct FbxProperty<'a> {
    reader: &'a mut FbxReader<File>,
    pub value: Value,
    array_size: u32
}

impl<'a> FbxProperty<'a> {
    pub fn new(reader: &'a mut FbxReader<File>) -> FbxProperty<'a> {
        
        FbxProperty {
            reader,
            value: Value::U8(0),
            array_size: 0
        }
    }

    pub fn read(&mut self) {

        let type_char = char::from(self.reader.read_u8());
        match type_char {
            'S' | 'R' => {
                self.read_special_type_value(type_char)
            }
            _ if type_char < 'Z' => {
                self.read_primitive_type_value(type_char)
            }
            _ => {
                let array_length = self.reader.read_u32();
                let encoding = self.reader.read_u32();
                let compressed_length = self.reader.read_u32() as usize; // convert to usize

                match encoding {
                    1 => {
                        let uncompressed_length = (self.read_array_type_size(type_char) * array_length) as usize;

                        let mut decompressed_buffer: Vec<u8> = vec![0; uncompressed_length];
                        let mut compressed_buffer: Vec<u8> = vec![0; compressed_length];
                        
                        self.reader.read_to_heap(&mut compressed_buffer);
                        
                        let mut compressed_slice = compressed_buffer.as_slice();
                        let mut decompressed_reader = GzDecoder::new(&mut compressed_slice);
                        decompressed_reader.read_to_end(&mut decompressed_buffer).unwrap();

                        // TODO: implement - check if decompression was successful

                        // TODO: create new reader to go through decompressed data
                        // TODO: for i in array_length -> read primitive value from decompressed using new reader and add it to vector

                    }
                    0 => {
                        panic!("not yet implemented");
                        // TODO: for i in array_length -> read primitive value and add to vector
                    }
                    _ => {
                        println!("Unsupported encoding type: {}", encoding);
                    }
                }
                // match encoding
            }
        }
    }

    fn read_primitive_type_value(&mut self, type_char: char) {
        match type_char {
            'B' | 'C' => {
                self.value = Value::Bool(self.reader.read_u8() != 0);
                println!("B or C");
            }, // B: 1 bit boolean (1: true, 0: false) encoded as the LSB of a 1 Byte value.
            'Y' => {
                self.value = Value::U16(self.reader.read_u16());
                println!("Y");
            },  // Y: 2 byte signed Integer
            'I' => {
                self.value = Value::U32(self.reader.read_u32());
                println!("I");
            },  // I: 4 byte signed Integer
            'L' => {
                self.value = Value::U64(self.reader.read_u64());
                println!("L");
            },  // L: 8 byte signed Integer
            'F' => {
                self.value = Value::F32(self.reader.read_f32());
                println!("F");
            }, // F: 4 byte single-precision IEEE 754 number
            'D' => {
                self.value = Value::F64(self.reader.read_f64());
                println!("D");
            },  // D: 8 byte double-precision IEEE 754 number
            _ => {
                self.value = Value::U8(0);
                println!("nothing");
            }
        };
    }

    fn read_special_type_value(&mut self, type_char: char) { // S-string, R-raw binary data
        let length = self.reader.read_u32();
        let mut value: Vec<u8> = Vec::new();

        for _ in 0..length {
            value.push(self.reader.read_u8());
        }
        self.value = Value::VecU8(value)
    }

    fn read_array_type_size(&mut self, type_char: char) -> u32{
        match type_char {
            'f' => 4, // f: Array of 4 byte single-precision IEEE 754 number
            'd' => 8, // d: Array of 8 byte double-precision IEEE 754 number
            'l' => 8, // l: Array of 8 byte signed Integer
            'i' => 4, // i: Array of 4 byte signed Integer
            'b' => 1, // D: 8 byte double-precision IEEE 754 number
            _ => panic!("Unsupported array type char")
        }
    }

}
/*
impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Value::I8(val) => write!(f, "I8: {}", val),
            Value::I16(val) => write!(f, "I16: {}", val),
            Value::I32(val) => write!(f, "I32: {}", val),
            Value::I64(val) => write!(f, "I64: {}", val),
            Value::F32(val) => write!(f, "F32: {}", val),
            Value::F64(val) => write!(f, "F64: {}", val),
            Value::VecF64(val) => write!(f, "VecF64: {:?}", val),
        }
    }
}
*/
/* 

fn read_primitive_type_value(type_char: char) {
    let value: Value;

    match type_char {
        'Y' => value = reader.readInt16(),  // Y: 2 byte signed Integer
        'C' => value = reader.readUint8() != 0,  // C: 1 bit boolean (1: true, 0: false) encoded as the LSB of a 1 Byte value.
        'I' => value = reader.readInt32(),  // I: 4 byte signed Integer
        'F' => value = reader.readFloat(), // F: 4 byte single-precision IEEE 754 number
        'D' => value = reader.readDouble(),  // D: 8 byte double-precision IEEE 754 number
        'L' => value = reader.readInt64(),  // L: 8 byte signed Integer
    }

    return value;
}

fn read_array_type_value(Reader: &reader, char: type_char) {

    match type_char {
        'f' => value.i16 = reader.readInt16(),  // f: Array of 4 byte single-precision IEEE 754 number
        'd' => value.bool = reader.readUint8() != 0,  // d: Array of 8 byte double-precision IEEE 754 number
        'l' => value.i32 = reader.readInt32(),  // l: Array of 8 byte signed Integer
        'i' => value.f32 = reader.readFloat(), // i: Array of 4 byte signed Integer
        'b' => value.f64 = reader.readDouble(),  // b: Array of 1 byte Booleans (always 0 or 1)
    }

    return value;
}
*/