

fn read_primitive_value(Reader: &reader, char: type_char) {

    if type_char == 'Y' { // Y: 2 byte signed Integer
        value.i16 = reader.readInt16();
    } else if type_char == 'C' || type_char == 'B' { // C: 1 bit boolean (1: true, 0: false) encoded as the LSB of a 1 Byte value.
        value.bool = reader.readUint8() != 0;
    } else if type_char = 'I' { // I: 4 byte signed Integer
        value.i32 = reader.readInt32();
    } else if type_char = 'F' {
        value.f32 = reader.readFloat();
    } else if type_char = 'D' {
        value.f64 = reader.readDouble();
    } else if type_char = 'L' {
        value.i64 = reader.readInt64();
    } else {
        panic!("Unsupported property type");
    }

    return value;
}