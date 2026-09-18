// Rust program to count numbers[cite: 12]

use std::io;[cite: 12]

fn main(){[cite: 12]

    println!("Enter lower bound");[cite: 12]
    let mut input1 = String::new();[cite: 12]
    io::stdin().read_line(&mut input1).expect("Failed to read input");[cite: 12]
    let lower_bound:i32 = input1.trim().parse().expect("Failed to input");[cite: 12]

    println!("Enter upper bound");[cite: 12]
    let mut input2 = String::new();[cite: 12]
    io::stdin().read_line(&mut input2).expect("Failed to read input");[cite: 12]
    let upper_bound:i32 = input2.trim().parse().expect("Failed to input");[cite: 12]

    for x in lower_bound..upper_bound{ // upper_bound is not inclusive[cite: 12]

        println!("Count Level is {}",x);[cite: 12]
    }[cite: 12]
}[cite: 12]