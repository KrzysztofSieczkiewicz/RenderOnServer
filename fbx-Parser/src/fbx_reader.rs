use std::io::Read;
pub struct FbxReader<R: Read> {
    reader: R,
    buf_magic: [u8; 23],
    buf_i32: [u8; 4],
}

impl<R: Read> FbxReader<R> {
    pub fn new(inner: R) -> FbxReader<R> {
        FbxReader {
            reader: inner,
            buf_magic: [0; 23],
            buf_i32: [0; 4],
        }
    }

    pub fn read_magic(&mut self) -> &[u8; 23] {
        self.reader
            .read_exact(&mut self.buf_magic)
            .expect("Magic should be readable");
        return &self.buf_magic;
    }

    pub fn read_version(&mut self) -> &[u8; 4] {
        self.reader
            .read_exact(&mut self.buf_i32)
            .expect("Version should be readable");
        return &self.buf_i32;
    }

    pub fn read_i32(&mut self) -> std::io::Result<i32> {
        self.reader.read_exact(&mut self.buf_i32)?;
        Ok(i32::from_le_bytes(self.buf_i32))
    }

    pub fn read_i64(&mut self) -> std::io::Result<i64> {
        let mut buf = [0; 8];
        self.reader.read_exact(&mut buf)?;
        Ok(i64::from_le_bytes(buf))
    }

    pub fn read_f32(&mut self) -> std::io::Result<f32> {
        let mut buf = [0; 4];
        self.reader.read_exact(&mut buf)?;
        Ok(f32::from_le_bytes(buf))
    }

    pub fn read_f64(&mut self) -> std::io::Result<f64> {
        let mut buf = [0; 8];
        self.reader.read_exact(&mut buf)?;
        Ok(f64::from_le_bytes(buf))
    }
}
