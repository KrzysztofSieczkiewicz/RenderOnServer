use std::fs::File;

use crate::fbx_reader::*;

pub struct FbxNode<'a> {
    reader: &'a mut FbxReader<File>
}

impl<'a> FbxNode<'a> {
    pub fn new(reader: &'a mut FbxReader<File>) -> FbxNode<'a> {
        FbxNode {
            reader: reader
        }
    }

    pub fn read_node(&mut self) {

        let end_offset = (self.reader).read_i32();
    }
    
}
