// Define struct for the laptops
struct Laptop {
    brand: String,
    quantity: u32,
    price_each: u32,
}

// Implement a method to calculate purchase cost for a given quantity
impl Laptop {
    fn purchase_cost(&self, qty: u32) -> u32 {
        self.price_each * qty
    }
}

fn main() {
    // Initialize the consignment items as given in the table
    let hp = Laptop {
        brand: String::from("HP"),
        quantity: 10,
        price_each: 650000,
    };

    let ibm = Laptop {
        brand: String::from("IBM"),
        quantity: 6,
        price_each: 755000,
    };

    let toshiba = Laptop {
        brand: String::from("Toshiba"),
        quantity: 10,
        price_each: 550000,
    };

    let dell = Laptop {
        brand: String::from("Dell"),
        quantity: 4,
        price_each: 850000,
    };

    // Customer purchases 3 laptops from each brand
    let units_bought = 3;

    let hp_total = hp.purchase_cost(units_bought);
    let ibm_total = ibm.purchase_cost(units_bought);
    let toshiba_total = toshiba.purchase_cost(units_bought);
    let dell_total = dell.purchase_cost(units_bought);

    let grand_total = hp_total + ibm_total + toshiba_total + dell_total;

    // Display receipt and total cost
    println!("=== Alaba Electronics Consignment Purchase ===");
    println!("Purchasing {} units of each brand:", units_bought);
    println!("- {} ({} units): ₦{}", hp.brand, units_bought, hp_total);
    println!("- {} ({} units): ₦{}", ibm.brand, units_bought, ibm_total);
    println!("- {} ({} units): ₦{}", toshiba.brand, units_bought, toshiba_total);
    println!("- {} ({} units): ₦{}", dell.brand, units_bought, dell_total);
    println!("----------------------------------------------");
    println!("Total Cost across all brands: ₦{}", grand_total);
}