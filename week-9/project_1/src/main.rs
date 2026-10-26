use std::fs::File;
use std::io::Write;

fn main() {
    // Create and open a file for writing
    let mut file = File::create("nigerian_breweries.txt").expect("Failed to create file");

    // Data to be written into the file
    let data = "--- NIGERIAN BREWERIES DRINKS CATEGORIES ---\n\n\
                LAGER:\n\
                - 33 Export\n\
                - Desperados\n\
                - Goldberg\n\
                - Gulder\n\
                - Heineken\n\
                - Star\n\n\
                STOUT:\n\
                - Legend\n\
                - Turbo King\n\
                - Williams\n\n\
                NON-ALCOHOLIC:\n\
                - Maltina\n\
                - Amstel Malta\n\
                - Malta Gold\n\
                - Fayrouz\n";

    // Write bytes to the file
    file.write_all(data.as_bytes()).expect("Failed to write to file");
    
    println!("Nigerian Breweries data successfully written to 'nigerian_breweries.txt'.");
} 