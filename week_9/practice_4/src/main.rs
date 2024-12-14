use std::fs::OpenOptions;
use std::io::Write;

fn main() {
    let mut file = OpenOptions::new().append(true).open("hello.txt").expect("Unable to open file");
    file.write_all("\nHello class".as_bytes()).expect("Unable to write data");
    file.write_all("\nThis is the appendage to the document.".as_bytes()).expect("Unable to write data");


    println!("File Append Success");
}
