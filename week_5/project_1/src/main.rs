use std::io;

fn main() {
    let menu = [
        ("P = Poundo Yam/Edinkaiko Soup", 3200),
        ("F = Fried Rice & Chicken", 3000),
        ("A = Amala & Ewedu Soup", 2500),
        ("E = Eba & Egusi Soup", 2000),
        ("W = White Rice & Stew", 2500),
    ];

    println!("Menu:");
    for (item, price) in &menu {
        println!("{} - N{}", item, price);
    }

    let mut total_cost = 0;

    loop {
        println!("\nEnter the food code (P, F, A, E, W) or 'Q' to finish:");

        let mut food = String::new();
        io::stdin().read_line(&mut food).expect("Failed to read line");
        let food = food.trim().to_uppercase();

        if food == "Q" {
            break;
        }

        let mut quantity = String::new();
        println!("Enter the quantity:");
        io::stdin().read_line(&mut quantity).expect("Failed to read line");
        let quantity: u32 = quantity.trim().parse().expect("Please enter a valid number");

        match food.as_str() {
            "P" => total_cost += quantity * 3200,
            "F" => total_cost += quantity * 3000,
            "A" => total_cost += quantity * 2500,
            "E" => total_cost += quantity * 2000,
            "W" => total_cost += quantity * 2500,
            _ => println!("Invalid food code, please try again."),
        }
    }

    if total_cost > 10_000 {
        total_cost = (total_cost as f64 * 0.95) as u32;
        println!("\nYou received a 5% discount for orders above N10,000!");
    }

    println!("\nTotal cost of your order: N{}", total_cost);
}

