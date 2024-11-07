use std::io;

fn main() {
    let a = get_input("Enter the value of a: ");
    let b = get_input("Enter the value of b: ");
    let c = get_input("Enter the value of c: ");

    let discriminant = b * b - 4.0 * a * c;

    if discriminant > 0.0 {
        let root1 = (-b + discriminant.sqrt()) / (2.0 * a);
        let root2 = (-b - discriminant.sqrt()) / (2.0 * a);
        println!("The roots are real and different: root1 = {}, root2 = {}", root1, root2);
    } else if discriminant == 0.0 {
        let root = -b / (2.0 * a);
        println!("The roots are real and same: root = {}", root);
    } else {
        let real_part = -b / (2.0 * a);
        let imaginary_part = (discriminant.abs().sqrt()) / (2.0 * a);
        println!(
            "The roots are complex and different: root1 = {} + {}i, root2 = {} - {}i",
            real_part, imaginary_part, real_part, imaginary_part
        );
    }
}

fn get_input(prompt: &str) -> f64 {
    let mut input = String::new();
    println!("{}", prompt);

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    
    input.trim().parse().expect("Please enter a valid number")
}
