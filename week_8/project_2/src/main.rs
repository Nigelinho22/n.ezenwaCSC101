// Define a struct to hold the developers information
struct Developer {
    name: String,
    years_of_experience: u32,
}

fn main() {
    // List the developers
    let developers = vec![
        Developer {
            name: String::from("Alice"),
            years_of_experience: 5,
        },
        Developer {
            name: String::from("Bob"),
            years_of_experience: 8,
        },
        Developer {
            name: String::from("Charlie"),
            years_of_experience: 12,
        },
    ];

    // Find the developer with the highest experience
    let most_experienced = find_most_experienced(&developers);

    // Print the result
    if let Some(developer) = most_experienced {
        println!(
            "The most experienced developer is {} with {} years of experience.",
            developer.name, developer.years_of_experience
        );
    } 
}

// Function to find the developer with the highest experience
fn find_most_experienced(developers: &[Developer]) -> Option<&Developer> {
    developers.iter().max_by_key(|d| d.years_of_experience)
}
