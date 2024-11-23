use std::io;

fn main() 
{
    let mut name = String::new();
    let mut dob = String::new();
    let mut email = String::new();
    let mut phone_no = String::new();
    let mut number_siblings = String::new();
    let mut child_count = String::new();
    let mut diagnosis = String::new();
    let mut village_residence = String::new();

    println!("Enter name: ");
    io::stdin().read_line(&mut name).expect("Error in reading line");

    println!("Enter date of birth (YYYY-MM-DD): ");
    io::stdin().read_line(&mut dob).expect("Error in reading line");

    println!("Enter email: ");
    io::stdin().read_line(&mut email).expect("Error in reading line");

    println!("Enter phone number: ");
    io::stdin().read_line(&mut phone_no).expect("Error in reading line");

    println!("Enter number of siblings: ");
    io::stdin().read_line(&mut number_siblings).expect("Error in reading line");
    let number_siblings: i32 = number_siblings.trim().parse().unwrap_or(0);

    println!("Enter number of children: ");
    io::stdin().read_line(&mut child_count).expect("Error in reading line");
    let child_count: i32 = child_count.trim().parse().unwrap_or(0);

    println!("Enter medical diagnosis: ");
    io::stdin().read_line(&mut diagnosis).expect("Error in reading line");
    
    println!("Enter village of residence: ");
    io::stdin().read_line(&mut village_residence).expect("Error in reading line");

    let base_amount = set_amount(&diagnosis);

    let age = 50;
    let discount = determine_discount(&diagnosis, age, child_count, number_siblings, &village_residence);

    let final_amount = base_amount * (1.0 - discount);
    println!("\nPatient Information");
    println!("Name: {}", name.trim());
    println!("DOB: {}", dob.trim());
    println!("Email: {}", email.trim());
    println!("Phone No: {}", phone_no.trim());
    println!("No of Siblings: {}", number_siblings);
    println!("No of Children: {}", child_count);
    println!("Diagnosis: {}", diagnosis.trim());
    println!("Village Residence: {}", village_residence.trim());
    println!("Discount: {}%", discount * 100.0);
    println!("Final Amount: ₦{:.2}", final_amount);

    if let Some(count) = latest_patient_count() 
    {
        if count >= 100 
        {
            println!("Congratulations! You have reached 100 patients.");
        }
    }
}

fn set_amount(diagnosis: &str) -> f64 
{
    match diagnosis.trim() {
        "Alzheimer" => 1_200_000.0,
        "Arrhythmia" => 550_000.0,
        "Chronic Kidney Disease" => 1_500_000.0,
        "Diabetes" => 800_000.0,
        "Arthritis" => 450_000.0,
        _ => 0.0,
    }
}

fn determine_discount(diagnosis: &str, age: i32, children: i32, siblings: i32, village: &str) -> f64 
{
    if diagnosis.eq_ignore_ascii_case("Alzheimer") && age > 50 && children > 4 && village.eq_ignore_ascii_case("Akpabom") {
        0.20
    } else if diagnosis.eq_ignore_ascii_case("Arrhythmia") && age == 30 && siblings > 4 && village.eq_ignore_ascii_case("Ngbauji") {
        0.05
    } else if diagnosis.eq_ignore_ascii_case("Chronic Kidney Disease") && age > 40 && children > 3 && siblings > 3 && village.eq_ignore_ascii_case("Atabrikang") {
        0.15
    } else if diagnosis.eq_ignore_ascii_case("Diabetes") && (28..45).contains(&age) && (2..=4).contains(&children) && village.eq_ignore_ascii_case("Okorobilom") {
        0.10
    } else if diagnosis.eq_ignore_ascii_case("Arthritis") && age > 58 && siblings > 5 && children > 5 && village.eq_ignore_ascii_case("Emeremen") {
        0.10
    } else {
        0.0
    }
}

fn latest_patient_count() -> Option<i32>
{
    static mut PATIENT_NO: i32 = 0;
    unsafe 
    {
        PATIENT_NO += 1;
        Some(PATIENT_NO)
    }
}