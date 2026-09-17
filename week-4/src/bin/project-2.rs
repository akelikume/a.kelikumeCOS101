// The Incentive Calculator

use std::io;

fn main(){println!("=== Employee Incentive Calculator ===");

    // Get experience status
    println!("Is the employee experienced? (yes/no):");
    let mut experience_input = String::new();
    io::stdin().read_line(&mut experience_input).expect("Failed to read input");
    let is_experienced = experience_input.trim().to_lowercase();

    if is_experienced == "yes" || is_experienced == "y" {
        // Get age if experienced
        println!("Enter the employee's age:");
        let mut age_input = String::new();
        io::stdin().read_line(&mut age_input).expect("Failed to read input");
        let age: u32 = match age_input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number for age.");
                return;
            }
        };

        // Determine incentive based on age criteria
        let incentive = if age >= 40 {
            1_560_000
        } else if age >= 30 && age <= 39 {
            1_480_000
        } else if age < 28 {
            1_300_000
        } else {
            // Catch-all for ages 28 and 29 if not explicitly categorized
            1_300_000 
        };

        println!("The annual incentive is: N{}", incentive);
    } else if is_experienced == "no" || is_experienced == "n" {
        println!("The annual incentive is: N100,000");
    } else {
        println!("Invalid input for experience status. Please enter 'yes' or 'no'.");
    }
	
}