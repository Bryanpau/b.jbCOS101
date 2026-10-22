use std::fs::File;
use std::io::Write;

fn main() {
    // Three separate datasets (arrays/vectors) representing backup pieces
    let commissioners = vec![
        "Aigbogun Alamba Daudu",
        "Murtala Afeez Bendu",
        "Okorocha Calistus Ogbona",
        "Adewale Jimoh Akanbi",
        "Osazuwa Faith Etieye",
    ];

    let ministries = vec![
        "Internal Affairs",
        "Justice",
        "Defense",
        "Power & Steel",
        "Petroleum",
    ];

    let zones = vec![
        "South West",
        "North East",
        "South South",
        "South West",
        "South East",
    ];

    let mut file = File::create("efcc_merged_ministers.txt").expect("Failed to create file");

    // Format headers
    let header = format!("{:<30} | {:<20} | {:<15}\n", "Minister Name", "Ministry", "Zone");
    let separator = format!("{}\n", "-".repeat(73));

    println!("{}", header.trim_end());
    println!("{}", separator.trim_end());

    let mut file_content = format!("{}{}", header, separator);

    // Merge datasets by index
    for i in 0..commissioners.len() {
        let record = format!("{:<30} | {:<20} | {:<15}", commissioners[i], ministries[i], zones[i]);
        println!("{}", record);
        file_content.push_str(&format!("{}\n", record));
    }

    // Write merged output to file
    file.write_all(file_content.as_bytes()).expect("Failed to write merged data");
    
    println!("\nDatasets successfully merged and saved to 'efcc_merged_ministers.txt'.");
}