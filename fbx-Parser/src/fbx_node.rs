use std::fs::File;

use crate::fbx_reader::*;

pub struct FbxNode {
    reader: FbxReader<File>
}

impl FbxNode {
    pub fn new(reader: FbxReader<File>) -> FbxNode {
        FbxNode {
            reader
        }
    }

    pub fn read_node(&mut self) {

        let end_offset = (&mut self.reader).read_i32();
    }
    
}
