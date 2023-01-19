use std::fs::File;

use crate::fbx_reader::*;

enum Value {
    I8(i8),
    I32(i32),
    I64(i64),
    F32(f32),
    F64(f64),
    VecF64(Vec<f64>),
}

pub struct FbxProperty<'a> {
    reader: &'a mut FbxReader<File>,
    name: String,
    type_char: char,
    label: String,
    flags: String,
    value: Option<Value>,
}

impl<'a> FbxProperty<'a> {
    pub fn new(reader: &'a mut FbxReader<File>) -> FbxProperty<'a> {
        FbxProperty {
            reader,
            name: todo!(),
            type_char: todo!(),
            label: todo!(),
            flags: todo!(),
            value: None,
        }
    }

    fn read_primitive_type_value(&mut self, type_char: char) {
        match type_char {
            'Y' => Value::I8(*&self.reader.read_i8()),  // Y: 2 byte signed Integer
            'C' => Value::I8(*&self.reader.read_i8()), // TODO?!  // C: 1 bit boolean (1: true, 0: false) encoded as the LSB of a 1 Byte value.
            'I' => Value::I32(*&self.reader.read_i32()),  // I: 4 byte signed Integer
            'F' => Value::F32(*&self.reader.read_f32()), // F: 4 byte single-precision IEEE 754 number
            'D' => Value::F64(*&self.reader.read_f64()),  // D: 8 byte double-precision IEEE 754 number
            'L' => Value::I64(*&self.reader.read_i64()),  // L: 8 byte signed Integer
            _ => panic!("Invalid type char")
        };
    }
}

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