use std::{fs::File};

use crate::{fbx_property::*, fbx_reader::*};

pub struct FbxNode<'a> {
    pub reader: &'a mut FbxReader<File>,
    name: String,
    num_properties: u32,
    property_list_length: u32,
    properties: Vec<(char, Value)>,
    end_offset: u32,
    pub last_node: bool,
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
            last_node: false,
        }
    }

    pub fn read_node(&mut self, filesize: &u64) {

        let start_offset = self.reader.offset;
        
        self.end_offset = (self.reader).read_u32();
        self.num_properties = (self.reader).read_u32();
        self.property_list_length = (self.reader).read_u32();
        self.name = self.read_property_name();

        if start_offset > (filesize - 159) as u32 { self.last_node = true }

        for _ in 0..self.num_properties {
            if self.reader.offset > (filesize - 159) as u32 { return }
            let mut fbx_property = FbxProperty::new(self.reader);
            fbx_property.read();

            self.properties.push((fbx_property.type_char, fbx_property.value));
        }
        
    }

    fn read_property_name(&mut self) -> String {
        
        let name_length = (self.reader).read_u8() as usize;
        let mut name_buffer = vec![0; name_length];
        self.reader.read_to_heap(&mut name_buffer);

        let name = String::from_utf8(name_buffer);
        match name {
            Ok(name) => {return name}
            Err(_) =>
                {
                    return String::from(""); 
                }
        }
    }
    
}
