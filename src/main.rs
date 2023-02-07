use std::{env, fs};

mod fbx_property;
mod fbx_file;
mod fbx_reader;
mod fbx_node;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = args[1].to_owned();

    let metadata = fs::metadata(&file_path).unwrap();
    let file_size = metadata.len();

    if fbx_file::read_file(&file_path, &file_size) {
        println!("FBX file is correct")
    }
}