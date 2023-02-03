use core::panic;
use std::{fs::File, io::{Read, Cursor}, any::type_name};
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
    VecBool(Vec<bool>),
    VecU16(Vec<u16>),
    VecU32(Vec<u32>),
    VecU64(Vec<u64>),
    VecF32(Vec<f32>),
    VecF64(Vec<f64>),
}

pub struct FbxProperty<'a> {
    reader: &'a mut FbxReader<File>,
    pub value: Value,
}

impl<'a> FbxProperty<'a> {
    pub fn new(reader: &'a mut FbxReader<File>) -> FbxProperty<'a> {
        
        FbxProperty {
            reader,
            value: Value::U8(0),
        }
    }

    pub fn read(&mut self) {
        println!("started reading");

        let type_char = char::from(self.reader.read_u8());
        match type_char {
            'S' | 'R' => {
                self.value = self.read_special_type_value();
                println!("    type char: S or R")
            }
            _ if type_char < 'Z' => {
                self.value = self.read_primitive_to_var(type_char);
                println!("    type char: Z")
            }
            _ => {
                let array_length = self.reader.read_u32();
                let encoding = self.reader.read_u32();
                let compressed_length = self.reader.read_u32() as usize; // convert to usize

                match encoding {
                    1 => {
                        let decompressed_length = (self.read_array_type_size(type_char) * array_length) as usize;

                        let mut compressed_buffer: Vec<u8> = vec![0; compressed_length];
                        self.reader.read_to_heap(&mut compressed_buffer);
                        
                        let mut decompressed_buffer: Vec<u8> = vec![0; decompressed_length];
                        let mut compressed_slice = compressed_buffer.as_slice();
                        GzDecoder::new(&mut compressed_slice).read_to_end(&mut decompressed_buffer).unwrap();

                        if decompressed_length != decompressed_buffer.len() {
                            println!("Decompression failed as decompressed slice is {} long, instead of expected {}", decompressed_length, decompressed_buffer.len())
                        }

                        let decompressed_cursor = Cursor::new(decompressed_buffer);
                        let mut decompressed_reader = FbxReader::new(decompressed_cursor);

                        self.value = Self::read_primitive_array_to_vec(&mut decompressed_reader, type_char, array_length);

                    }
                    0 => {
                        self.value = Self::read_primitive_array_to_vec(self.reader, type_char, array_length);
                    }
                    _ => {
                        println!("Unsupported encoding type: {}", encoding);
                    }
                }
                println!("    type char: {}", type_char);
                // match encoding
            }
        }
        println!("finished reading");
    }

    fn read_primitive_to_var(&mut self, type_char: char) -> Value{
        match type_char {
            'B' | 'C' => {
                return Value::Bool(self.reader.read_u8() != 0);
            }, // B: 1 bit boolean (1: true, 0: false) encoded as the LSB of a 1 Byte value.
            'Y' => {
                return Value::U16(self.reader.read_u16());
            },  // Y: 2 byte signed Integer
            'I' => {
                return Value::U32(self.reader.read_u32());
            },  // I: 4 byte signed Integer
            'L' => {
                return Value::U64(self.reader.read_u64());
            },  // L: 8 byte signed Integer
            'F' => {
                return Value::F32(self.reader.read_f32());
            }, // F: 4 byte single-precision IEEE 754 number
            'D' => {
                return Value::F64(self.reader.read_f64());
            },  // D: 8 byte double-precision IEEE 754 number
            _ => {
                return Value::U8(0);
            }
        };
    }

    fn read_primitive_array_to_vec<R: Read>(fbx_reader: &mut FbxReader<R>, type_char: char, array_length: u32) -> Value {
        match type_char {
            'B' | 'C' => {
                let mut temporary_vector: Vec<bool> = vec![];
                for _ in 0..array_length {
                    temporary_vector.push(fbx_reader.read_u8() != 0);
                }
                return Value::VecBool(temporary_vector);
            },
            'Y' => {
                let mut temporary_vector: Vec<u16> = vec![];
                for _ in 0..array_length {
                    temporary_vector.push(fbx_reader.read_u16());
                }
                return Value::VecU16(temporary_vector);
            },
            'I' => {
                let mut temporary_vector: Vec<u32> = vec![];
                for _ in 0..array_length {
                    temporary_vector.push(fbx_reader.read_u32());
                }
                return Value::VecU32(temporary_vector);
            },
            'L' => {
                let mut temporary_vector: Vec<u64> = vec![];
                for _ in 0..array_length {
                    temporary_vector.push(fbx_reader.read_u64());
                }
                return Value::VecU64(temporary_vector);
            },
            'F' => {
                let mut temporary_vector: Vec<f32> = vec![];
                for _ in 0..array_length {
                    temporary_vector.push(fbx_reader.read_f32());
                }
                return Value::VecF32(temporary_vector);
            },
            'D' => {
                let mut temporary_vector: Vec<f64> = vec![];
                for _ in 0..array_length {
                    temporary_vector.push(fbx_reader.read_f64());
                }
                return Value::VecF64(temporary_vector);
            },
            _ => {
                return Value::VecU8(vec![0]);
            }
        };
    }

    fn read_special_type_value(&mut self) -> Value { // S-string, R-raw binary data
        let length: u32 = self.reader.read_u32();
        let mut value: Vec<u8> = Vec::new();

        for _ in 0..length {
            value.push(self.reader.read_u8());
        }
        return Value::VecU8(value)
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