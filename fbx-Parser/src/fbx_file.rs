
use std::{fs::File, io::{Read, Write}};

pub fn open_file(file_path: &str) -> File {

    let file = File::open(&file_path)
        .expect  (&format!("Should've been able to open file in directory {}", &file_path));

    return file;
}

pub fn read_file(mut file: &File) -> String {

    let mut content: Vec<u8> = vec![];
    file.read_to_end(&mut content).unwrap();

    let text:String = String::from_utf8(content).unwrap();

    return text;
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
