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
        let num_properties = (self.reader).read_u32();
        let property_list_length = (self.reader).read_u32();
        let name_length = (self.reader).read_u8();

        let bytes = 13 + name_length;

        let mut property = FbxProperty::new(self.reader);
        property.read_primitive_type_value('Y');
        if let Value::U8(i) = property.value {
            println!("Value: {}", i)
        }

        //println!("name: {}", property.value);
        
        println!("end_offset: {}", &end_offset);
        println!("num_properties: {}", &num_properties);
        println!("property_list_length: {}", &property_list_length);
        println!("name_length: {}", &name_length);
        println!("bytes: {}", &bytes);
    }
    
}
