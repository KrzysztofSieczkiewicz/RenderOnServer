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
        println!("starting reading node");

        let end_offset = (self.reader).read_u32();
        println!("Offset after end_offset: {}", self.reader.offset);
        let num_properties = (self.reader).read_u32();
        println!("num of properties: {}", num_properties);
        println!("Offset after num_properties: {}", self.reader.offset);
        let property_list_length = (self.reader).read_u32();
        println!("Offset after property_list_length: {}", self.reader.offset);
        let name_length = (self.reader).read_u8() as usize;
        println!("Offset after name_length: {}", self.reader.offset);
        let mut name_buffer = vec![0; name_length];
        self.reader.read_to_heap(&mut name_buffer);

        println!("Name: {}", String::from_utf8(name_buffer).unwrap());

        println!("starting looping through properties for node");
        let mut property = FbxProperty::new(self.reader);
        for i in 0..num_properties {
            println!("property num: {}", i);
            property.read();
        }
        println!("finishing reading node");
    }
    
}
