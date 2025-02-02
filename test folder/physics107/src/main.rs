use std::io;

fn main() {
    println!("Welcome to the Appliance Carbon Footprint Calculator!");
    println!("We'll calculate the annual carbon footprint of your air conditioning unit, refrigerator, television, and laptop.");
    println!("--------------------------------------------------");

    // Step 1: Get daily usage in hours for each appliance
    println!("Enter the daily usage of your appliances in hours:");
    println!("These are your appliances:");
    println!("1. Air Conditioning Unit");
    println!("2. Refrigerator");
    println!("3. Television");
    println!("4. Laptop");

    println!("1. Air Conditioning Unit (hours/day):");
    let ac_hours = get_input();

    println!("2. Refrigerator (hours/day):");
    let fridge_hours = get_input();

    println!("3. Television (hours/day):");
    let tv_hours = get_input();

    println!("4. Laptop (hours/day):");
    let laptop_hours = get_input();

    // Step 2: Define power ratings (in watts) for each appliance
    let ac_power = 2000.0; // Average power usage for air conditioning unit (in watts)
    let fridge_power = 150.0; // Average power usage for refrigerator (in watts)
    let tv_power = 100.0; // Average power usage for television (in watts)
    let laptop_power = 50.0; // Average power usage for laptop (in watts)

    // Step 3: Calculate annual energy consumption (kWh) for each appliance
    let ac_annual_kwh = calculate_annual_kwh(ac_hours, ac_power);
    let fridge_annual_kwh = calculate_annual_kwh(fridge_hours, fridge_power);
    let tv_annual_kwh = calculate_annual_kwh(tv_hours, tv_power);
    let laptop_annual_kwh = calculate_annual_kwh(laptop_hours, laptop_power);

    // Step 4: Define emission factor (in kg CO2 per kWh)
    let emission_factor = 0.5; // Average emission factor

    // Step 5: Calculate carbon footprint for each appliance
    let ac_emissions = ac_annual_kwh * emission_factor;
    let fridge_emissions = fridge_annual_kwh * emission_factor;
    let tv_emissions = tv_annual_kwh * emission_factor;
    let laptop_emissions = laptop_annual_kwh * emission_factor;

    // Step 6: Calculate total emissions
    let total_emissions = ac_emissions + fridge_emissions + tv_emissions + laptop_emissions;

    // Step 7: Display the results
    println!("\nYour Annual Carbon Footprint:");
    println!("Air Conditioning Unit: {:.2} kg CO2", ac_emissions);
    println!("Refrigerator: {:.2} kg CO2", fridge_emissions);
    println!("Television: {:.2} kg CO2", tv_emissions);
    println!("Laptop: {:.2} kg CO2", laptop_emissions);
    println!("--------------------------------------------------");
    println!("Total Carbon Footprint: {:.2} kg CO2", total_emissions);
    println!("Thank you for using the calculator!");
}

// Function to get user input as a floating-point number
fn get_input() -> f64 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().parse::<f64>().unwrap_or(0.0)
}

// Function to calculate annual energy consumption (in kWh)
fn calculate_annual_kwh(daily_hours: f64, power: f64) -> f64 {
    let daily_kwh = (daily_hours * power) / 1000.0; // Convert watts to kilowatts
    daily_kwh * 365.0 // Annual energy consumption
}