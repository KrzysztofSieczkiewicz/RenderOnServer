use std::io::Read;
pub struct FbxReader<R: Read> {
    reader: R,
    pub offset: i32,
    buf_magic: [u8; 23],
    buf_i8: [u8; 1],
    buf_i32: [u8; 4],
}

impl<R: Read> FbxReader<R> {
    pub fn new(inner: R) -> FbxReader<R> {
        FbxReader {
            reader: inner,
            offset: 0,
            buf_magic: [0; 23],
            buf_i8: [0; 1],
            buf_i32: [0; 4],
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

    pub fn read_i32(&mut self) -> i32 {
        self.reader.read_exact(&mut self.buf_i32)
            .expect("i32 value should be readable");

        self.offset += 8;
        return i32::from_le_bytes(self.buf_i32)
    }

}
