fn main() {
    let p: f64 = 210_000.0; // Initial cost of the TV (N210,000)
    let r: f64 = 5.0;       // Depreciation rate per annum (5%)
    let n: f64 = 3.0;       // Time in years (3 years)

    // Formula: A = P * [1 - (R/100)]^n
    let a = p * (1.0 - (r / 100.0)).powf(n);

    // Calculate total depreciation amount
    let total_depreciation = p - a;

    println!("Initial Cost of TV (P): N{:.2}", p);
    println!("Value of TV after 3 years (A): N{:.2}", a);
    println!("Total Depreciation: N{:.2}", total_depreciation);
}