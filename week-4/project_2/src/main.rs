use std::io;

fn main() {
    let mut exp_input = String::new();
    let mut age_input = String::new();

    println!("Is the employee experienced? (yes/no): ");
    io::stdin().read_line(&mut exp_input).expect("Failed to read input");
    let is_experienced = exp_input.trim().to_lowercase() == "yes";

    println!("Enter the employee's age: ");
    io::stdin().read_line(&mut age_input).expect("Failed to read input");
    let age: u32 = age_input.trim().parse().expect("Not a valid number");

    if is_experienced {
        if age >= 40 {
            println!("Annual incentive: N1,560,000");
        } else if age >= 30 && age <= 39 {
            println!("Annual incentive: N1,480,000");
        } else if age < 28 {
            println!("Annual incentive: N1,300,000");
        } else {
            println!("Annual incentive: Not specified for age 28-29");
        }
    } else {
        println!("Annual incentive: N100,000");
    }
}