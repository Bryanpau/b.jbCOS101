fn main() {
    // Sales amounts for each item
    let toshiba: f64 = 450_000.00;
    let mac: f64 = 1_500_000.00;
    let hp: f64 = 750_000.00;
    let dell: f64 = 2_850_000.00;
    let acer: f64 = 250_000.00;

    // Calculate total sum of sales
    let sum = toshiba + mac + hp + dell + acer;

    // Total number of items/records
    let count: f64 = 5.0;

    // Calculate average sale amount
    let average = sum / count;

    // Display the results
    println!("--- P.M. Okeke and Sons Ltd Sales Report ---");
    println!("Total Sales (Sum): N{:.2}", sum);
    println!("Average Sales: N{:.2}", average);
}