use std::io;

fn main() {
    // Display the menu[cite: 1]
    println!("========================================");
    println!("       PROJECT: THE RESTAURANT MENU     ");
    println!("========================================");
    println!(" [P] Poundo Yam / Edinkaiko Soup  - N3,200[cite: 1]");
    println!(" [F] Fried Rice & Chicken         - N3,000[cite: 1]");
    println!(" [A] Amala & Ewedu Soup           - N2,500[cite: 1]");
    println!(" [E] Eba & Egusi Soup             - N2,000[cite: 1]");
    println!(" [W] White Rice & Stew            - N2,500[cite: 1]");
    println!("========================================");

    // Read the food type (letter) from the customer[cite: 1]
    println!("Enter the letter corresponding to your food choice (P, F, A, E, W):");
    let mut food_choice = String::new();
    io::stdin()
        .read_line(&mut food_choice)
        .expect("Failed to read input");
    let food_choice = food_choice.trim().to_uppercase();

    // Read the quantity from the customer[cite: 1]
    println!("Enter the quantity:");
    let mut quantity_input = String::new();
    io::stdin()
        .read_line(&mut quantity_input)
        .expect("Failed to read input");
    let quantity: u32 = match quantity_input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a valid number for quantity.");
            return;
        }
    };

    // Determine the price per item based on the letter choice
    let price_per_item = match food_choice.as_str() {
        "P" => 3200.0,
        "F" => 3000.0,
        "A" => 2500.0,
        "E" => 2000.0,
        "W" => 2500.0,
        _ => {
            println!("Invalid food choice code. Please restart and pick a valid letter.");
            return;
        }
    };

    // Compute the total charge[cite: 1]
    let mut total_charge = price_per_item * (quantity as f64);
    let mut discount_applied = false;

    // Check if the total is greater than N10,000 to give a 5% discount[cite: 1]
    if total_charge > 10000.0 {
        total_charge *= 0.95; // Apply 5% discount (pay 95%)
        discount_applied = true;
    }

    // Display the final summary
    println!("\n----------------------------------------");
    println!("             ORDER SUMMARY              ");
    println!("----------------------------------------");
    if discount_applied {
        println!("A 5% discount has been applied (Total exceeded N10,000)[cite: 1]!");
    }
    println!("Final Total Charge: N{:.2}", total_charge);
    println!("----------------------------------------");
}
