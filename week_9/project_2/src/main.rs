use std::fs::File;
use std::io::{BufWriter, Write};

#[derive(Debug)]
struct Student {
    name: String,
    matric_number: String,
    department: String,
    level: u32,
}

fn main() {
    let students = [
        Student {
            name: "Oluchi Mardi".to_string(),
            matric_number: "ACC10211111".to_string(),
            department: "Accounting".to_string(),
            level: 300,
        },
        Student {
            name: "Adams Aliyu".to_string(),
            matric_number: "ECO10110101".to_string(),
            department: "Economics".to_string(),
            level: 100,
        },
        Student {
            name: "Shania Bolade".to_string(),
            matric_number: "CSC10328828".to_string(),
            department: "Computer".to_string(),
            level: 200,
        },
        Student {
            name: "Adekunle Gold".to_string(),
            matric_number: "EEE11020202".to_string(),
            department: "Electrical".to_string(),
            level: 200,
        },
        Student {
            name: "Blanca Edomoh".to_string(),
            matric_number: "MEE10202001".to_string(),
            department: "Mechanical".to_string(),
            level: 100,
        },
        // Add more students as needed
    ];

    // Display student details
    println!("PAU SMIS");
    println!("Student Name\tMatric. Number\tDepartment\tLevel");
    for student in &students {
        println!("{}\t{}\t{}\t{}", student.name, student.matric_number, student.department, student.level);
    }

    // Save student details to a CSV file
    let file = File::create("student_data.csv").expect("Error creating file");
    let mut writer = BufWriter::new(file);

    writeln!(writer, "PAU SMIS").expect("Error writing to file");
    writeln!(writer, "Student Name,Matric. Number,Department,Level").expect("Error writing to file");
    for student in &students {
        writeln!(writer, "{},{},{},{}", student.name, student.matric_number, student.department, student.level).expect("Error writing to file");
    }
    println!("Student data saved to student_data.csv");
}