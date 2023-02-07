use std::env;

mod fbx_property;
mod fbx_file;
mod fbx_reader;
mod fbx_node;

fn main() {
    let args: Vec<String> = env::args().collect();

    fbx_file::read_file(args[1].to_owned());
}