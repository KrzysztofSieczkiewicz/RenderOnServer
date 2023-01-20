use std::io::Read;
pub struct FbxReader<R: Read> {
    reader: R,
    pub offset: i32,
    buf_magic: [u8; 23],
    buf_i8: [u8; 1],
    buf_i16: [u8; 2],
    buf_i32: [u8; 4],
    buf_i64: [u8; 8],
    buf_f32: [u8; 4],
    buf_f64: [u8; 8],
}

impl<R: Read> FbxReader<R> {
    pub fn new(inner: R) -> FbxReader<R> {
        FbxReader {
            reader: inner,
            offset: 0,
            buf_magic: [0; 23],
            buf_i8: [0; 1],
            buf_i16: [0; 2],
            buf_i32: [0; 4],
            buf_i64: [0; 8],
            buf_f32: [0; 4],
            buf_f64: [0; 8],
        }
    }

    pub fn read_magic(&mut self) -> &[u8; 23] {
        self.reader
            .read_exact(&mut self.buf_magic)
            .expect("Magic should be readable");
        
        self.offset += 23;
        return &self.buf_magic;
    }

    pub fn read_version(&mut self) -> &[u8; 4] {
        self.reader
            .read_exact(&mut self.buf_i32)
            .expect("Version should be readable");
        
        self.offset += 4;
        return &self.buf_i32;
    }

    pub fn read_i8(&mut self) -> i8 {
        self.reader.read_exact(&mut self.buf_i8)
            .expect("i8 value should be readable");

        self.offset += 1;
        return i8::from_le_bytes(self.buf_i8)
    }

    pub fn read_i16(&mut self) -> i16 {
        self.reader.read_exact(&mut self.buf_i16)
            .expect("i16 value should be readable");

        self.offset += 2;
        return i16::from_le_bytes(self.buf_i16)
    }

    pub fn read_i32(&mut self) -> i32 {
        self.reader.read_exact(&mut self.buf_i32)
            .expect("i32 value should be readable");

        self.offset += 4;
        return i32::from_le_bytes(self.buf_i32)
    }

    pub fn read_i64(&mut self) -> i64 {
        self.reader.read_exact(&mut self.buf_i64)
            .expect("i64 value should be readable");

        self.offset += 8;
        return i64::from_le_bytes(self.buf_i64)
    }

    pub fn read_f32(&mut self) -> f32 {
        self.reader.read_exact(&mut self.buf_f32)
            .expect("f32 value should be readable");

        self.offset += 4;
        return f32::from_le_bytes(self.buf_f32)
    }

    pub fn read_f64(&mut self) -> f64 {
        self.reader.read_exact(&mut self.buf_f64)
            .expect("f64 value should be readable");

        self.offset += 8;
        return f64::from_le_bytes(self.buf_f64)
    }

}
