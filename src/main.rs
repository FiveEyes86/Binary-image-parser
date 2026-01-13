use std::io;
use std::{fs::File, io::Read};
use std::str;


const FILE_PATH: &str = r"D:\RustProjects\RustTutorial\BinaryImageParser\стратега.PNG";


#[derive(Debug)]
struct FileInfo {
    width: u32,
    height: u32,
}


fn main() {
    let mut buffer = Vec::new();
    let mut file = File::open(FILE_PATH).unwrap();//expect("Cannot open file");
    file.read_to_end(&mut buffer).expect("Cannot read file");

    let extencion_bytes = &buffer[1..4];
    let width = &buffer[16..20];
    let height = &buffer[20..24];

    let file_info = FileInfo {
        width: u32::from_be_bytes(width.try_into().expect("Cannot convert to [u8; 4]")),
        height: u32::from_be_bytes(height.try_into().expect("Cannot convert to [u8; 4]")),
    };
    let file_extencion = str::from_utf8(extencion_bytes).expect("Cannot convert from utf-8");

    println!("File extencion: {}", file_extencion);
    println!("File scale {} x {}", file_info.width, file_info.height);

    println!("Print ENTER to exit...");
    let mut stdin_buf = String::new();
    io::stdin().read_line(&mut stdin_buf);
}
