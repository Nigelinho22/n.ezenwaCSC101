

use std::fs::File;
use std::io::{BufWriter, Write};

fn main() {
    // Define the separate datasets
    let names = ["Aigbogun Alamba Daudu", "Murtala Afeez Bendu", "Okorocha Calistus Ogbona", "Adewale Jimoh Akanbi", "Osazuwa Faith Etieye"];
    let ministries = ["Internal Affairs", "Justice", "Defense", "Power & Steel", "Petroleum"];
    let zones = ["South West", "North East", "South South", "South West", "South East"];

    // Merge the datasets into a vector of tuples
    let merged_data: Vec<(usize, &str, &str, &str)> = names.iter().zip(ministries.iter()).zip(zones.iter()).map(|((name, ministry), zone)| (name.len() + 1, name, ministry, zone)).collect();

    // Write the merged data to a CSV file
    let file = File::create("merged_data.csv").expect("Error creating file");
    let mut writer = BufWriter::new(file);

    writeln!(writer, "S/N,NAME OF COMMISIONER,S/N,MINISTRY,S/N,GEOPOLITICAL ZONE").expect("Error writing to file");
    for (sn, name, ministry, zone) in merged_data {
        writeln!(writer, "{},{},{},{},{},{}", sn, name, sn, ministry, sn, zone).expect("Error writing to file");
    }
    println!("Merged data saved to merged_data.csv");
}