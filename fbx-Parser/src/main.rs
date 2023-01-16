mod fbx_property;
mod fbx_file;
mod fbx_reader;

fn main() {
    println!("Called fbx_file");
    let file_path = "C:\\Krzysiek\\Programming\\Back-end\\RenderOnServer\\fbx-Parser\\sickle.fbx";
    let save_file = "C:\\Krzysiek\\Programming\\Back-end\\RenderOnServer\\fbx-Parser\\testsaved.txt";

    //let contents = fbx_file::read_file(&file);

    //println!("{}", &contents);

    fbx_file::read_file(file_path);
}