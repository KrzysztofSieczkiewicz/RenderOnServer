mod fbx_property;
mod fbx_file;

fn main() {
    println!("Called fbx_file");
    let file = fbx_file::open_file("C:\\Krzysiek\\Programming\\Back-end\\RenderOnServer\\fbx-Parser\\test.txt");
    let save_file = "C:\\Krzysiek\\Programming\\Back-end\\RenderOnServer\\fbx-Parser\\testsaved.txt";

    let contents = fbx_file::read_file(&file);

    println!("{}", &contents);

    fbx_file::write_file_as_txt(save_file, contents)
}