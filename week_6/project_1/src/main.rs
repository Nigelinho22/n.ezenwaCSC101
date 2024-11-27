use std::io;

fn main() {
    println!("Welcome to the Geometry Calculator!");
    loop {
        println!("What would you like to calculate?");
        println!("1. Area of Trapezium");
        println!("2. Area of Rhombus");
        println!("3. Area of Parallelogram");
        println!("4. Area of Cube");
        println!("5. Volume of Cylinder");
        println!("6. Exit");

        let choice = get_input("Enter your choice (1-6):")
            .trim()
            .parse::<u8>()
            .unwrap_or(0);

        if choice == 6 {
            println!("Goodbye!");
            break;
        }

        match choice {
            1 => calculate_trapezium(),
            2 => calculate_rhombus(),
            3 => calculate_parallelogram(),
            4 => calculate_cube(),
            5 => calculate_cylinder(),
            _ => println!("Invalid choice. Please try again."),
        }

        println!();
    }
}

fn calculate_trapezium() {
    let height = get_input("Enter the height:").trim().parse::<f64>().unwrap_or(0.0);
    let base1 = get_input("Enter base1:").trim().parse::<f64>().unwrap_or(0.0);
    let base2 = get_input("Enter base2:").trim().parse::<f64>().unwrap_or(0.0);

    let area = height / 2.0 * (base1 + base2);
    println!("The area of the trapezium is: {:.2}", area);
}

fn calculate_rhombus() {
    let diagonal1 = get_input("Enter diagonal1:").trim().parse::<f64>().unwrap_or(0.0);
    let diagonal2 = get_input("Enter diagonal2:").trim().parse::<f64>().unwrap_or(0.0);

    let area = 0.5 * diagonal1 * diagonal2;
    println!("The area of the rhombus is: {:.2}", area);
}

fn calculate_parallelogram() {
    let base = get_input("Enter the base:").trim().parse::<f64>().unwrap_or(0.0);
    let height = get_input("Enter the height:").trim().parse::<f64>().unwrap_or(0.0);

    let area = base * height;
    println!("The area of the parallelogram is: {:.2}", area);
}

fn calculate_cube() {
    let side = get_input("Enter the length of one side:").trim().parse::<f64>().unwrap_or(0.0);

    let area = 6.0 * side.powi(2);
    println!("The surface area of the cube is: {:.2}", area);
}

fn calculate_cylinder() {
    let radius = get_input("Enter the radius:").trim().parse::<f64>().unwrap_or(0.0);
    let height = get_input("Enter the height:").trim().parse::<f64>().unwrap_or(0.0);

    let volume = std::f64::consts::PI * radius.powi(2) * height;
    println!("The volume of the cylinder is: {:.2}", volume);
}

fn get_input(prompt: &str) -> String {
    let mut input = String::new();
    println!("{}", prompt);
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input
}
