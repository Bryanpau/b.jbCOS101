use std::io;

// Function for Trapezium Area
// Formula: height / 2 * (base1 + base2)
fn calculate_trapezium_area(height: f64, base1: f64, base2: f64) -> f64 {
    (height / 2.0) * (base1 + base2)
}

// Function for Rhombus Area
// Formula: 1/2 * diagonal1 * diagonal2
fn calculate_rhombus_area(diagonal1: f64, diagonal2: f64) -> f64 {
    0.5 * diagonal1 * diagonal2
}

// Function for Parallelogram Area
// Formula: base * altitude[cite: 1]
fn calculate_parallelogram_area(base: f64, altitude: f64) -> f64 {
    base * altitude
}

// Function for Cube Surface Area
// Formula: 6 * side * side[cite: 1]
fn calculate_cube_surface_area(side: f64) -> f64 {
    6.0 * side * side
}

// Function for Cylinder Volume
// Formula: pi * radius * radius * height[cite: 1]
fn calculate_cylinder_volume(radius: f64, height: f64) -> f64 {
    std::f64::consts::PI * radius * radius * height
}

// Helper function to read a floating-point number from standard input
fn read_input(prompt: &str) -> f64 {
    loop {
        println!("{}", prompt);
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
        
        match input.trim().parse::<f64>() {
            Ok(num) => return num,
            Err(_) => println!("Invalid input. Please enter a valid number."),
        }
    }
}

fn main() {
    println!("=== Shape Calculator (MTH 101) ==[cite: 1]");
    println!("Select a shape to calculate:");
    println!("1. Trapezium (Area)");
    println!("2. Rhombus (Area)");
    println!("3. Parallelogram (Area)");
    println!("4. Cube (Surface Area)");
    println!("5. Cylinder (Volume)");

    let choice = read_input("Enter your choice (1-5):");

    if choice == 1.0 {
        let height = read_input("Enter height:");
        let base1 = read_input("Enter base 1:");
        let base2 = read_input("Enter base 2:");
        let area = calculate_trapezium_area(height, base1, base2);
        println!("The area of the trapezium is: {:.2}", area);
    } else if choice == 2.0 {
        let d1 = read_input("Enter diagonal 1:");
        let d2 = read_input("Enter diagonal 2:");
        let area = calculate_rhombus_area(d1, d2);
        println!("The area of the rhombus is: {:.2}", area);
    } else if choice == 3.0 {
        let base = read_input("Enter base:");
        let altitude = read_input("Enter altitude:");
        let area = calculate_parallelogram_area(base, altitude);
        println!("The area of the parallelogram is: {:.2}", area);
    } else if choice == 4.0 {
        let side = read_input("Enter side length:");
        let area = calculate_cube_surface_area(side);
        println!("The surface area of the cube is: {:.2}", area);
    } else if choice == 5.0 {
        let radius = read_input("Enter radius:");
        let height = read_input("Enter height:");
        let volume = calculate_cylinder_volume(radius, height);
        println!("The volume of the cylinder is: {:.2}", volume);
    } else {
        println!("Invalid selection. Please restart the program and choose between 1 and 5.");
    }
}
