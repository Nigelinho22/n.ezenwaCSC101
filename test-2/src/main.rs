
use std::io;

fn main() {
    let mut name = String::new();
    let mut dob = String::new();
    let mut email = String::new();
    let mut phone_number = String::new();
    let mut number_siblings = String::new();
    let mut number_of_children = String::new();
    let mut medical_diagnosis = String::new();
    let mut village_of_residence = String::new();

    println!("Enter name: ");
    io::stdin().read_line(&mut name).expect("Failed to read line");
    println!("Enter date of birth (YYYY-MM-DD): ");
    io::stdin().read_line(&mut dob).expect("Failed to read line");
    println!("Enter email: ");
    io::stdin().read_line(&mut email).expect("Failed to read line");
    println!("Enter phone number: ");
    io::stdin().read_line(&mut phone_number).expect("Failed to read line");
    println!("Enter number of siblings: ");
    io::stdin().read_line(&mut number_siblings).expect("Failed to read line");
    let number_siblings: i32 = number_siblings.trim().parse().expect("Invalid input");

    println!("Enter number of children: ");
    io::stdin().read_line(&mut number_of_children).expect("Failed to read line");
    let number_of_children: i32 = number_of_children.trim().parse().expect("Invalid input");

    println!("Enter medical diagnosis: ");
    io::stdin().read_line(&mut medical_diagnosis).expect("Failed to read line");
    println!("Enter village of residence: ");
    io::stdin().read_line(&mut village_of_residence).expect("Failed to read line");

    let base_amount = match medical_diagnosis.trim() {
        "Alzheimer" => 1_200_000.0,
        "Arrhythmia" => 550_000.0,
        "Chronic Kidney Disease" => 1_500_000.0,
        "Diabetes" => 800_000.0,
        "Arthritis" => 450_000.0,
        _ => 0.0,
    };

    let mut discount = 0.0;
    println!("\nPatient Information");
    println!("Name: {}", name.trim());
    println!("DOB: {}", dob.trim());
    println!("Email: {}", email.trim());
    println!("Phone Number: {}", phone_number.trim());
    println!("Number of Siblings: {}", number_siblings);
    println!("Number of Children: {}", number_of_children);
    println!("Medical Diagnosis: {}", medical_diagnosis.trim());
    println!("Village of Residence: {}", village_of_residence.trim());

    let age: i32 = 50;

    if medical_diagnosis.trim().eq_ignore_ascii_case("Alzheimer")
        && age > 50
        && number_of_children > 4
        && village_of_residence.trim().eq_ignore_ascii_case("Akpabom") {
            discount = 0.20;
        } else if medical_diagnosis.trim().eq_ignore_ascii_case("Arrhythmia")
            && age == 30
            && number_siblings > 4
            && village_of_residence.trim().eq_ignore_ascii_case("Ngbauji") {
            discount = 0.05;
        } else if medical_diagnosis.trim().eq_ignore_ascii_case("Chronic Kidney Disease")
            && age > 40
            && number_of_children > 3
            && number_siblings > 3
            && village_of_residence.trim().eq_ignore_ascii_case("Atabrikang") {
            discount = 0.15;
        } else if medical_diagnosis.trim().eq_ignore_ascii_case("Diabetes")
            && age > 28 && age < 45
            && (2..=4).contains(&number_of_children)
            && village_of_residence.trim().eq_ignore_ascii_case("Okorobilom") {
            discount = 0.10;
        } else if medical_diagnosis.trim().eq_ignore_ascii_case("Arthritis")
            && age > 58
            && number_siblings > 5
            && number_of_children > 5
            && village_of_residence.trim().eq_ignore_ascii_case("Emeremen") {
            discount = 0.10;
        }
    
        let final_amount = base_amount * (1.0 - discount);
        println!("Discount: {}%", discount * 100.0);
        println!("Final Amount: ₦{:.2}", final_amount);

        let mut patient_count = 0;
        patient_count += 1;

        if patient_count >= 100 {
            println!("Congratulations! You have reached 100 patients.");
        }
    }
}   
