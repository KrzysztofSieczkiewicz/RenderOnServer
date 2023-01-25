use std::fs::File;

use crate::{fbx_property::*, fbx_reader::*};

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

        let end_offset = (self.reader).read_u32();
        println!("Offset after end_offset: {}", self.reader.offset);
        let num_properties = (self.reader).read_u32();
        println!("Offset after num_properties: {}", self.reader.offset);
        let property_list_length = (self.reader).read_u32();
        println!("Offset after property_list_length: {}", self.reader.offset);
        let name_length = (self.reader).read_u8();
        println!("Offset after name_length: {}", self.reader.offset);

        let bytes = 13 + name_length;

        let mut property = FbxProperty::new(self.reader);
        property.read();

        println!("Offset after primitive type: {}", self.reader.offset);
        
        println!("end_offset: {}", &end_offset);
        println!("num_properties: {}", &num_properties);
        println!("property_list_length: {}", &property_list_length);
        println!("name_length: {}", &name_length);
        println!("bytes: {}", &bytes);
    }
    
}
