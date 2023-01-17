use std::fs::File;

use crate::fbx_reader::*;

pub struct FbxNode {
}

impl FbxNode {
    pub fn new() {
    }

    fn read_node(mut reader: &mut FbxReader<File>) {

    let end_offset = (&mut reader).read_i32();
}
    
}
