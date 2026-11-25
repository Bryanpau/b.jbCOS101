use std::io::{self, Read, Write};

fn read_sql_file(filename: &str) {
    let mut file = std::fs::File::open(filename).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    println!("{}", contents);
}

fn administrator() {
    read_sql_file("globacom_db.sql");
}

fn project_manager() {
    read_sql_file("project_tb.sql");
}

fn employee() {
    read_sql_file("staff_tb.sql");
}

fn customer() {
    read_sql_file("customer_tb.sql");
}

fn vendor() {
    read_sql_file("dataplan_tb.sql");
}

fn main() {
    print!("Enter role (administrator, project manager, employee, customer, vendor): ");
    io::stdout().flush().unwrap();

    let mut role = String::new();
    io::stdin().read_line(&mut role).unwrap();

    match role.trim().to_lowercase().as_str() {
        "administrator" => administrator(),
        "project manager" => project_manager(),
        "employee" => employee(),
        "customer" => customer(),
        "vendor" => vendor(),
        _ => println!("Unknown role."),
    }
}