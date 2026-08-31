fn main() {
    let p: f64 = 520_000_000.0; // Principal loan amount (N520,000,000)
    let r: f64 = 10.0;          // Rate per annum (10%)
    let n: f64 = 5.0;           // Time in years (5 years)

    // Formula: A = P * [1 + (R/100)]^n
    let a = p * (1.0 + (r / 100.0)).powf(n);

    // Compound Interest: CI = A - P
    let ci = a - p;

    println!("Principal Amount (P): N{:.2}", p);
    println!("Total Amount (A): N{:.2}", a);
    println!("Compound Interest (CI): N{:.2}", ci);
}