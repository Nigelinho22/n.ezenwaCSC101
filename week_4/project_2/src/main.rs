use std::io;

fn main() {
    let experience = get_input("Enter the employee's experience (in years): ");
    let age = get_input("Enter the employee's age: ");

    let incentive = determine_incentive(experience, age);

    println!("The annual incentive for the employee is: ${}", incentive);
}

fn get_input(prompt: &str) -> i32 {
    let mut input = String::new();
    println!("{}", prompt);

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    input.trim().parse().expect("Please enter a valid number")
}

fn determine_incentive(experience: i32, age: i32) -> i32 {
    let base_incentive = if experience > 10 {
        5000
    } else if experience >= 5 {
        3000
    } else {
        1000
    };

    let age_bonus = if age > 50 {
        1000
    } else {
        0
    };

    base_incentive + age_bonus
}
