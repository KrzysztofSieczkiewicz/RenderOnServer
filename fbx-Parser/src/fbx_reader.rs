use std::io::Read;
pub struct FbxReader<R: Read> {
    reader: R,
    pub offset: u32,
    buf_magic: [u8; 23],
    buf_u8: [u8; 1],
    buf_u16: [u8; 2],
    buf_u32: [u8; 4],
    buf_u64: [u8; 8],
    buf_f32: [u8; 4],
    buf_f64: [u8; 8],
}

impl<R: Read> FbxReader<R> {
    pub fn new(inner: R) -> FbxReader<R> {
        FbxReader {
            reader: inner,
            offset: 0,
            buf_magic: [0; 23],
            buf_u8: [0; 1],
            buf_u16: [0; 2],
            buf_u32: [0; 4],
            buf_u64: [0; 8],
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
            .read_exact(&mut self.buf_u32)
            .expect("Version should be readable");
        
        self.offset += 4;
        return &self.buf_u32;
    }

    pub fn read_u8(&mut self) -> u8 {
        self.reader.read_exact(&mut self.buf_u8)
            .expect("u8 value should be readable");

        self.offset += 1;
        return u8::from_le_bytes(self.buf_u8)
    }

    pub fn read_u16(&mut self) -> u16 {
        self.reader.read_exact(&mut self.buf_u16)
            .expect("u16 value should be readable");

        self.offset += 2;
        return u16::from_le_bytes(self.buf_u16)
    }

    pub fn read_u32(&mut self) -> u32 {
        self.reader.read_exact(&mut self.buf_u32)
            .expect("u32 value should be readable");

        self.offset += 4;
        return u32::from_le_bytes(self.buf_u32)
    }

    pub fn read_u64(&mut self) -> u64 {
        self.reader.read_exact(&mut self.buf_u64)
            .expect("u64 value should be readable");

        self.offset += 8;
        return u64::from_le_bytes(self.buf_u64)
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
