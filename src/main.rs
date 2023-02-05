mod fbx_property;
mod fbx_file;
mod fbx_reader;
mod fbx_node;

fn main() {
    println!("Called fbx_file");
    let file_path = "C:\\Krzysiek\\Programming\\Back-end\\RenderOnServer\\fbx-Parser\\sickle.fbx";

    //let contents = fbx_file::read_file(&file);

    //println!("{}", &contents);

    fbx_file::read_file(file_path);
}