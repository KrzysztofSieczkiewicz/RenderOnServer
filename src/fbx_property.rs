use core::panic;
use std::{fs::File, io::{Read, Cursor}};
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
    pub type_char: char,
    pub last_property: bool,
}


impl<'a> FbxProperty<'a> {
    pub fn new(reader: &'a mut FbxReader<File>) -> FbxProperty<'a> {

        FbxProperty {
            reader,
            type_char: 'A',
            value: Value::U8(0),
            last_property: false,
        }
    }


    pub fn read(&mut self) {
        self.type_char = char::from(self.reader.read_u8());
        match self.type_char {
            'S' | 'R' => {
                self.value = self.read_special_type_value();
            }
            _ if self.type_char < 'Z' => {
                self.value = self.read_primitive_to_var(self.type_char);
            }
            _ => {
                let array_length = self.reader.read_u32();
                let encoding = self.reader.read_u32();
                let compressed_length = self.reader.read_u32() as usize; // convert to usize

                match encoding {
                    1 => {
                        let decompressed_length = (self.read_array_type_size(self.type_char) * array_length) as usize;

                        let mut compressed_buffer: Vec<u8> = vec![0; compressed_length];
                        self.reader.read_to_heap(&mut compressed_buffer);
                        
                        let mut decompressed_buffer: Vec<u8> = vec![0; decompressed_length];
                        let mut compressed_slice = compressed_buffer.as_slice();
                        let decode = GzDecoder::new(&mut compressed_slice).read_to_end(&mut decompressed_buffer);
                        match decode {
                            Ok(_) => {}
                            Err(_) => {
                                self.last_property = true;
                            }
                        }

                        if decompressed_length != decompressed_buffer.len() {
                            panic!("Decompression failed as decompressed slice is {} long, instead of expected {}", decompressed_length, decompressed_buffer.len())
                        }

                        let decompressed_cursor = Cursor::new(decompressed_buffer);
                        let mut decompressed_reader = FbxReader::new(decompressed_cursor);

                        self.value = Self::read_primitive_array_to_vec(&mut decompressed_reader, self.type_char, array_length);

                    }
                    0 => {
                        self.value = Self::read_primitive_array_to_vec(self.reader, self.type_char, array_length);
                    }
                    _ => {
                        self.last_property = true;
                        return;
                        //panic!("Unsupported encoding type: {}, offset: {}", encoding, self.reader.offset);
                    }
                }
            }
        }
    }

    fn read_primitive_to_var(&mut self, type_char: char) -> Value{
        match type_char {
            'B' | 'C' => { // B: 1 bit boolean (1: true, 0: false) encoded as the LSB of a 1 Byte value.
                return Value::Bool(self.reader.read_u8() != 0);
            },
            'Y' => {  // Y: 2 byte signed Integer
                return Value::U16(self.reader.read_u16());
            },  // Y: 2 byte signed Integer
            'I' => {  // I: 4 byte signed Integer
                return Value::U32(self.reader.read_u32());
            },
            'L' => {  // L: 8 byte signed Integer
                return Value::U64(self.reader.read_u64());
            },
            'F' => {  // F: 4 byte single-precision IEEE 754 number
                return Value::F32(self.reader.read_f32());
            },
            'D' => {  // D: 8 byte double-precision IEEE 754 number
                return Value::F64(self.reader.read_f64());
            },
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


impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Value::U8(val) => write!(f, "{}", val),
            Value::Bool(val) => write!(f, "{}", val),
            Value::U16(val) => write!(f, "{}", val),
            Value::U32(val) => write!(f, "{}", val),
            Value::U64(val) => write!(f, "{}", val),
            Value::F32(val) => write!(f, "{}", val),
            Value::F64(val) => write!(f, "{}", val),
            Value::VecU8(val) => {
                for i in val {
                    write!(f, " {:?}", i).ok();
                } Ok(())
            },
            Value::VecBool(val) => {
                for i in val {
                    write!(f, " {:?} ", i).ok();
                } Ok(())
            },
            Value::VecU16(val) => {
                for i in val {
                    write!(f, " {:?} ", i).ok();
                } Ok(())
            },
            Value::VecU32(val) => {
                for i in val {
                    write!(f, " {:?} ", i).ok();
                } Ok(())
            },
            Value::VecU64(val) => {
                for i in val {
                    write!(f, " {:?} ", i).ok();
                } Ok(())
            },
            Value::VecF32(val) => {
                for i in val {
                    write!(f, " {:?} ", i).ok();
                } Ok(())
            },
            Value::VecF64(val) => {
                for i in val {
                    write!(f, " {:?} ", i).ok();
                } Ok(())
            },
        }
    }
}
