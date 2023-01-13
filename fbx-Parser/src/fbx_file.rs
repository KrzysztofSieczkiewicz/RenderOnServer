
use core::panic;
use std::{fs::File,
    io::{Read, Write},
    str, string};

pub fn read_file(file_path: &str) {

    let mut file = File::open(&file_path)
        .expect("Should've been able to open a file");

    let mut file_contents: Vec<u8> = vec![];
    file.read_to_end(&mut file_contents).unwrap();

    check_fbx(&file_contents);
}


pub fn check_fbx(file_contents: &Vec<u8>) {

    let magic_actual = file_contents[0..23].to_vec();
    println!("{}", magic_actual.len());
    let magic_actual_str = str::from_utf8(&magic_actual).unwrap();
    println!("{}A", magic_actual_str);

    let magic_str = "Kaydara FBX Binary  ";
    let magic_u8 = magic_str.as_bytes();

    for i in 0..magic_u8.len() {
        if magic_u8[i] != file_contents[i] {
            panic!("File should start with {} instead of {}", &magic_str, magic_actual_str);
        }
    }

    if magic_actual[20] != 0x00 {
        println!("{} instead of 0x00", magic_actual[20])
    }
    if magic_actual[21] != 0x1A {
        println!("{} instead of 0x00", magic_actual[20])
    }
    if magic_actual[22] != 0x00 {
        println!("{} instead of 0x00", magic_actual[20])
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
