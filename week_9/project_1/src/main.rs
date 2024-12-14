use std::fs::File;
use std::io::{self, Write};

fn main() -> io::Result<()> {
    // Define the drink categories
    let drinks = vec![
        ("33 Export", "Legend", "Maltina"),
        ("Desperados", "Turbo King", "Amstel Malta"),
        ("Goldberg", "Williams", "Malta Gold"),
        ("Gulder", "", "Fayrouz"),
        ("Heineken", "", ""),
        ("Star", "", ""),
    ];

    // Create or open a file to write the data
    let mut file = File::create("drinks.txt")?;

    // Write the table header
    writeln!(file, "{:<15} {:<15} {:<15}", "Lager", "Stout", "Non-Alcoholic")?;
    writeln!(file, "-----------------------------------------------")?;

    // Write the drink categories row by row
    for (lager, stout, non_alcoholic) in drinks {
        writeln!(file, "{:<15} {:<15} {:<15}", lager, stout, non_alcoholic)?;
    }

    println!("Data successfully written to drinks.txt");

    Ok(())
}
