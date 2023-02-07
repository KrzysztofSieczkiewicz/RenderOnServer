use std::{fs::File};

use crate::{fbx_property::*, fbx_reader::*};

pub struct FbxNode<'a> {
    reader: &'a mut FbxReader<File>,
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

    pub fn read_node(&mut self) {
        let mut bytes = 0;

        self.end_offset = (self.reader).read_u32();
        self.num_properties = (self.reader).read_u32();
        self.property_list_length = (self.reader).read_u32();
        self.name = self.read_property_name();

        if self.name == "Footer" { return }

        println!("Name: {}", self.name);
        println!("end_offset: {}", self.end_offset);
        println!("reader offset: {}", self.reader.offset);
        println!("num_properties: {}", self.num_properties);
        println!("property_list_length: {}", self.property_list_length);

        for _ in 0..self.num_properties {
            let mut fbx_property = FbxProperty::new(self.reader);
            fbx_property.read();

            println!("type_char: {},  value: {}", fbx_property.type_char, fbx_property.value);

            self.properties.push((fbx_property.type_char, fbx_property.value));
        }
        
    }

    fn read_property_name(&mut self) -> String {
        
        let name_length = (self.reader).read_u8() as usize;
        let mut name_buffer = vec![0; name_length];
        println!("Name length: {}", name_buffer.len());
        self.reader.read_to_heap(&mut name_buffer);
        let name_buffer_clone = name_buffer.clone();

        let name = String::from_utf8(name_buffer);
        match name {
            Ok(name) => {return name}
            Err(_) =>
                {
                    let mut end_vector: Vec<u8> = vec![];
                    if Self::check_if_footer(&name_buffer_clone) {
                        self.last_node = true;
                        self.reader.read_to_end(&mut end_vector);
                        self.properties = vec![('Z', Value::VecU8(end_vector))];
                    return String::from("Footer"); 
                    }
                    return String::from(""); 
                }
        }
    }

    fn check_if_footer(buffer: &Vec<u8>) -> bool {
        let footer = [247, 38, 126, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 232, 28, 0, 0, 0, 0, 0, 0];

        println!("I'm checking for footer: ");
        for (i, &element) in footer.iter().enumerate() {
            println!("{}: {}, {}", i, buffer[i], element);
            if element != buffer[i] {
                return false;
            }
        }
        return true;
    }
    
}
