use std::io;[cite: 13]

fn main() {[cite: 13]

    println!("Enter a number");[cite: 13]
    let mut input1 = String::new();[cite: 13]
    io::stdin().read_line(&mut input1).expect("Failed to read input");[cite: 13]
    let mut num:i32 = input1.trim().parse().expect("Failed to input");[cite: 13]

    while num < 10 {[cite: 13]

        println!("inside loop number value is {}",num);[cite: 13]
        num+=1;[cite: 13]
    }[cite: 13]
    println!("outside loop number value is {}",num);[cite: 13]
}[cite: 13]