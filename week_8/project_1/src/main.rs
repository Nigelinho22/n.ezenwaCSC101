use std::collections::HashMap;

// Define the APS levels and their corresponding ranges
const aps_levels: [&str; 5] = ["APS 1-2", "APS 3-5", "APS 5-8", "EL1 8-10", "EL2 10-13"];
const aps_ranges: [[usize; 2]; 5] = [[1, 2], [3, 5], [5, 8], [8, 10], [10, 13]];

// Define the staff positions and their corresponding APS levels
const staff_positions: HashMap<&str, &str> = HashMap::from([
    ("Intern", "APS 1-2"),
    ("Administrator", "APS 3-5"),
    ("Senior Administrator", "APS 5-8"),
    ("Office Manager", "EL1 8-10"),
    ("Director", "EL2 10-13"),
    ("CEO", "SES"),
    ("Research Assistant", "APS 3-5"),
    ("PhD Candidate", "APS 5-8"),
    ("Post-Doc Researcher", "EL1 8-10"),
    ("Senior Lecturer", "EL2 10-13"),
    ("Dean", "SES"),
    ("Paralegal", "APS 1-2"),
    ("Junior Associate", "APS 3-5"),
    ("Associate", "APS 5-8"),
    ("Senior Associate 1-2", "EL1 8-10"),
    ("Senior Associate 3-4", "EL2 10-13"),
    ("Partner", "SES"),
    ("Placement", "APS 1-2"),
    ("Classroom Teacher", "APS 3-5"),
    ("Snr Teacher", "APS 5-8"),
    ("Leading Teacher", "EL1 8-10"),
    ("Deputy Principal", "EL2 10-13"),
    ("Principal", "SES"),
]);

fn main() {
    // Get the staff position and years of experience from the user
    let mut position = String::new();
    println!("Enter the staff position:");
    std::io::stdin().read_line(&mut position).unwrap();
    position = position.trim().to_string();

    let mut experience = String::new();
    println!("Enter the years of work experience:");
    std::io::stdin().read_line(&mut experience).unwrap();
    let experience: usize = experience.trim().parse().unwrap();

    // Determine the APS level based on the staff position and experience
    let staff_aps_level = staff_aps_level.get(position.as_str()).unwrap();
    let aps_range = aps_ranges[aps_levels.iter().position(|&x| x == *staff_aps_level).unwrap()];
    let aps_level = if experience >= aps_range[1] {
        aps_levels[aps_levels.iter().position(|&x| x == *staff_aps_level).unwrap() + 1]
    } else {
        staff_aps_level
    };

    // Print the determined APS level
    println!("The staff holds the position of {}", aps_level);
}