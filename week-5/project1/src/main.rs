use std::io;

fn main() {
    println!("--------------------------------------------------");
    println!("                   FOOD MENU                      ");
    println!("--------------------------------------------------");
    println!(" Code | Food Item                 | Price        ");
    println!("--------------------------------------------------");
    println!("  P   | Poundo Yam / Edinkaiko    | N3,200       ");
    println!("  F   | Fried Rice & Chicken      | N3,000       ");
    println!("  A   | Amala & Ewedu Soup        | N2,500       ");
    println!("  E   | Eba & Egusi Soup          | N2,000       ");
    println!("  W   | White Rice & Stew         | N2,500       ");
    println!("--------------------------------------------------");

    println!("\nEnter food code (P, F, A, E, W): ");
    let mut food_code = String::new();
    io::stdin()
        .read_line(&mut food_code)
        .expect("Failed to read input");

    let food_code = food_code.trim().to_uppercase();

    let price = match food_code.as_str() {
        "P" => 3200.0,
        "F" => 3000.0,
        "A" => 2500.0,
        "E" => 2000.0,
        "W" => 2500.0,
        _ => {
            println!("Invalid food selection!");
            return;
        }
    };

    println!("Enter quantity: ");
    let mut quantity_str = String::new();
    io::stdin()
        .read_line(&mut quantity_str)
        .expect("Failed to read input");

    let quantity: f64 = match quantity_str.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid quantity!");
            return;
        }
    };

    let total = price * quantity;

    println!("\nSubtotal: N{:.2}", total);

    if total > 10000.0 {
        let discount = total * 0.05;
        let final_total = total - discount;
        println!("Discount (5%): N{:.2}", discount);
        println!("Total Charge: N{:.2}", final_total);
    } else {
        println!("Total Charge: N{:.2}", total);
    }
}
