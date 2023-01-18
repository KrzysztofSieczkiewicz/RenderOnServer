use std::fs::File;

use crate::fbx_reader::*;

pub struct FbxNode<'a> {
    reader: &'a mut FbxReader<File>
}

impl<'a> FbxNode<'a> {
    pub fn new(reader: &'a mut FbxReader<File>) -> FbxNode<'a> {
        FbxNode {
            reader
        }
    }

    pub fn read_node(&mut self) {

        let end_offset = (self.reader).read_i32();
        let num_properties = (self.reader).read_i32();
        let property_list_length = (self.reader).read_i32();
        let name_length = (self.reader).read_i8();

        let bytes = 13 + name_length;
        
        println!("end_offset: {}", &end_offset);
        println!("num_properties: {}", &num_properties);
        println!("property_list_length: {}", &property_list_length);
        println!("name_length: {}", &name_length);
        println!("bytes: {}", &bytes);
    }
    
}
