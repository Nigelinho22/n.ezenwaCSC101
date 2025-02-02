use std::io;

fn main() {
// Welcome message
println!("Welcome to the Carbon Footprint Calculator!");
println!("This program calculates your carbon footprint based on some basic inputs.");
println!("--------------------------------------------------");

// Ask for transportation input
println!("Transportation:");
println!("How many kilometers do you drive per week?");
let weekly_km = get_input();
println!("What is your car's fuel efficiency in liters per 100 km?");
let fuel_efficiency = get_input();

// Ask for electricity usage input
println!("Electricity:");
println!("How much electricity do you use per month (in kWh)?");
let monthly_kwh = get_input();

// Ask for waste data
println!("Waste:");
println!("How many kilograms of waste do you produce per week?");
let weekly_waste = get_input();

// Calculate results
let car_emissions = calculate_car_emissions(weekly_km, fuel_efficiency);
let electricity_emissions = calculate_electricity_emissions(monthly_kwh);
let waste_emissions = calculate_waste_emissions(weekly_waste);
let total_emissions = car_emissions + electricity_emissions + waste_emissions;

// Show results
println!("--------------------------------------------------");
println!("Here is your annual carbon footprint:");
println!("From driving: {} kg CO2", car_emissions);
println!("From electricity: {} kg CO2", electricity_emissions);
println!("From waste: {} kg CO2", waste_emissions);
println!("Total: {} kg CO2", total_emissions);
println!("Thank you for using this program!");
}

// Function to get user input
fn get_input() -> f64 {
let mut input = String::new();
io::stdin().read_line(&mut input).unwrap(); // Reads user input
let input = input.trim().parse::<f64>(); // Try to convert to a number

// Handle invalid input
if input.is_err() {
println!("Invalid input! Using 0 as default value.");
0.0
} else {
input.unwrap()
}
}

// Function to calculate car emissions
fn calculate_car_emissions(weekly_km: f64, fuel_efficiency: f64) -> f64 {
let liters_per_year = (weekly_km / 100.0) * fuel_efficiency * 52.0; // 52 weeks in a year
let emission_factor = 2.31; // kg CO2 per liter of gasoline
liters_per_year * emission_factor
}

// Function to calculate electricity emissions
fn calculate_electricity_emissions(monthly_kwh: f64) -> f64 {
let emission_factor = 0.5; // kg CO2 per kWh
monthly_kwh * 12.0 * emission_factor // 12 months in a year
}

// Function to calculate waste emissions
fn calculate_waste_emissions(weekly_waste: f64) -> f64 {
let emission_factor = 52.0; // kg CO2 per kg of waste per year
weekly_waste * emission_factor
}