use core::panic;
use std::{fs::File,
    io::{Write},
    str};

use crate::fbx_reader::*;

pub fn read_file(file_path: &str) {

    let file = File::open(&file_path)
        .expect("Should've been able to open a file");

    let mut reader = FbxReader::new(file);

    check_fbx_magic((&mut reader).read_magic());
    check_fbx_version((&mut reader).read_version());
}


pub fn check_fbx_magic(buffer: &[u8; 23]) {
    let magic = "Kaydara FBX Binary  ".as_bytes();
    let magic_actual = &buffer[0..23];

    for i in 0..magic.len() {
        if magic[i] != magic_actual[i] {
            panic!("File should have valid magic");
        }
    }

    if magic_actual[20] != 0x00 {
        panic!("There should be 0x00 instead of {}", magic_actual[20])
    }
    if magic_actual[21] != 0x1A {
        panic!("There should be 0x1A instead of {}", magic_actual[21])
    }
    if magic_actual[22] != 0x00 {
        panic!("There should be 0x00 instead of {}", magic_actual[22])
    }
    println!("Magic check passed!")
}


pub fn check_fbx_version(buffer: &[u8; 4]) {

    let max_version = 7400;
    let version_actual = i32::from_le_bytes(*buffer);
    
    if version_actual > max_version {
        panic!("File version should not exceed {}. Actual file version: {}", max_version, version_actual);
    }
    println!("Version check passed!")
}


pub fn write_file_as_txt(file_path: &str, text: String) {
    let mut output = File::create(file_path);

    let mut file = match File::create(file_path) {
        Err(why) => panic!("couldn't create file in path: {}", file_path),
        Ok(file) => file,
    };

    match file.write_all(text.as_bytes()) {
        Err(why) => panic!("couldn't write to file: {}", why),
        Ok(_) => println!("successfully wrote to file"),
    };
}
