use std::{fs::File, collections::HashMap};

use crate::{fbx_property::*, fbx_reader::*};

pub struct FbxNode<'a> {
    reader: &'a mut FbxReader<File>,
    name: String,
    num_properties: u32,
    property_list_length: u32,
    properties: Vec<(char, Value)>,
    end_offset: u32,
}

impl<'a> FbxNode<'a> {
    pub fn new(reader: &'a mut FbxReader<File>) -> FbxNode<'a> {
        FbxNode {
            reader,
            name: "".to_string(),
            num_properties: 0,
            property_list_length: 0,
            properties: vec![],
            end_offset: 0,
        }
    }

    pub fn read_node(&mut self) {
        self.end_offset = (self.reader).read_u32();
        self.num_properties = (self.reader).read_u32();
        self.property_list_length = (self.reader).read_u32();

        self.name = self.read_property_name();
        println!("Name: {}", self.name);

        for i in 0..self.num_properties {
            let mut property = FbxProperty::new(self.reader);
            property.read();

            //self.properties.push(property) -> TODO in each iteration create a char-Value tuple + push it to self.properties vector
        }
    }

    fn read_property_name(&mut self) -> String {
        let name_length = (self.reader).read_u8() as usize;
        let mut name_buffer = vec![0; name_length];
        self.reader.read_to_heap(&mut name_buffer);

        return String::from_utf8(name_buffer).unwrap()
    }
    
}
