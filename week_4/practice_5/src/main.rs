use std::io;

fn main() {
    let mut input = String::new();

    println!("\nEnter Your Height (in centimeters):");
    io::stdin().read_line(&mut input).expect("Not a valid string");
    let height:f32 = input.trim().parse().expect("Not a valid number");

    if height >= 150.0 && height <= 170.0 {
        println!("You are an average height person");
    }

    else if height > 170.0 && height <= 195.0 {
        println!("You are Tall");
    }

    else if height < 150.0 && height >100.0 {
        println!("You are Dwarf");
    }

    else {
        println!("Abnormal Height");
    }

}
