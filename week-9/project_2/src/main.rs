use std::fs::File;
use std::io::Write;

// Define a struct to represent student details
struct Student {
    name: &'static str,
    matric_num: &'static str,
    department: &'static str,
    level: u32,
}

fn main() {
    // 1. Read/store student details in a vector
    let students = vec![
        Student { name: "Oluchi Mordi", matric_num: "ACC10211111", department: "Accounting", level: 300 },
        Student { name: "Adams Aliyu", matric_num: "ECO10110101", department: "Economics", level: 100 },
        Student { name: "Shania Bolade", matric_num: "CSC10328828", department: "Computer", level: 200 },
        Student { name: "Adekunle Gold", matric_num: "EEE11020202", department: "Electrical", level: 200 },
        Student { name: "Blanca Edemoh", matric_num: "MEE10202001", department: "Mechanical", level: 100 },
    ];

    let mut file = File::create("pau_students.txt").expect("Failed to create file");

    // Prepare table header formatting
    let header = format!("{:<20} | {:<15} | {:<15} | {:<5}\n", "Student Name", "Matric. Number", "Department", "Level");
    let separator = format!("{}\n", "-".repeat(65));

    // 2. Display on screen
    print!("{}", header);
    print!("{}", separator);

    let mut file_content = format!("{}{}", header, separator);

    for student in &students {
        let row = format!("{:<20} | {:<15} | {:<15} | {:<5}", student.name, student.matric_num, student.department, student.level);
        println!("{}", row);
        file_content.push_str(&format!("{}\n", row));
    }

    // 3. Write to the file
    file.write_all(file_content.as_bytes()).expect("Failed to write to file");
    
    println!("\nStudent details successfully saved to 'pau_students.txt'.");
}