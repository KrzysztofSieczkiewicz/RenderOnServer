
use core::panic;
use std::{fs::File,
    io::{Read, Write},
    str,
    convert::TryInto};

pub fn read_file(file_path: &str) {

    let mut file = File::open(&file_path)
        .expect("Should've been able to open a file");

    let mut file_contents: Vec<u8> = vec![];
    let number = file.read_to_end(&mut file_contents).unwrap();

    println!("{}, {}", number, file_contents.len());
    check_fbx(&file_contents);
}


pub fn check_fbx(file_contents: &Vec<u8>) {
    let magic = "Kaydara FBX Binary  ".as_bytes();

    println!("{}", &file_contents.len());
    let magic_actual = &file_contents[0..27];
    let magic_actual_str = str::from_utf8(&magic_actual[0..23]).unwrap();

    for i in 0..magic.len() {
        if magic[i] != magic_actual[i] {
            panic!("File shouldn't start with: '{}'", magic_actual_str);
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

    let max_version = 7400;
    let version_actual_ref = &magic_actual[23..27];

    let version_actual = i32::from_le_bytes(version_actual_ref.try_into().unwrap());
    
    if version_actual > max_version {
        panic!("File version should not exceed {}. Actual file version: {}", max_version, version_actual);
    }

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
