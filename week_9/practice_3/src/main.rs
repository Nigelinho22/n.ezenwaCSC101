use std::fs;

fn main() {
    fs::remove_file("hello.txt").expect("Unable to delete file");
    println!("file is removed");
}
