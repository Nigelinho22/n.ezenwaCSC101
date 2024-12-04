fn main() {
    let v : Vec<i64> = Vec::new();

    println!("\nThe length of vec::new: {}", v.len());

    let v = vec!["Grace", "Jade", "Mike", "Sara", "Basil"];

    println!("\nThe length of vec macro is: {}", v.len());
}
