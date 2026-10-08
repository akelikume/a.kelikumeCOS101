use std::io;

// 1. Trapezium Area: height / 2 * (base1 + base2)
fn calc_trapezium() {
    let mut input = String::new();
    
    println!("Enter height:");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let height: f64 = input.trim().parse().expect("Invalid number");

    input.clear();
    println!("Enter base1:");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let base1: f64 = input.trim().parse().expect("Invalid number");

    input.clear();
    println!("Enter base2:");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let base2: f64 = input.trim().parse().expect("Invalid number");

    let area = height / 2.0 * (base1 + base2);
    println!("The area of the Trapezium is: {}\n", area);
}

// 2. Rhombus Area: 1/2 * diagonal1 * diagonal2
fn calc_rhombus() {
    let mut input = String::new();
    
    println!("Enter diagonal1:");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let diagonal1: f64 = input.trim().parse().expect("Invalid number");

    input.clear();
    println!("Enter diagonal2:");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let diagonal2: f64 = input.trim().parse().expect("Invalid number");

    let area = 0.5 * diagonal1 * diagonal2;
    println!("The area of the Rhombus is: {}\n", area);
}

// 3. Parallelogram Area: base * altitude
fn calc_parallelogram() {
    let mut input = String::new();
    
    println!("Enter base:");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let base: f64 = input.trim().parse().expect("Invalid number");

    input.clear();
    println!("Enter altitude:");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let altitude: f64 = input.trim().parse().expect("Invalid number");

    let area = base * altitude;
    println!("The area of the Parallelogram is: {}\n", area);
}

// 4. Cube Surface Area: 6 * side * side
fn calc_cube() {
    let mut input = String::new();
    
    println!("Enter side length:");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let side: f64 = input.trim().parse().expect("Invalid number");

    let surface_area = 6.0 * side * side;
    println!("The surface area of the Cube is: {}\n", surface_area);
}

// 5. Cylinder Volume: pi * radius * radius * height
fn calc_cylinder() {
    let mut input = String::new();
    
    println!("Enter radius:");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let radius: f64 = input.trim().parse().expect("Invalid number");

    input.clear();
    println!("Enter height:");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let height: f64 = input.trim().parse().expect("Invalid number");

    let pi = std::f64::consts::PI;
    let volume = pi * radius * radius * height;
    println!("The volume of the Cylinder is: {}\n", volume);
}

fn main() {
    loop {
        println!("=== THE SHAPE CALCULATOR ===");
        println!("1. Trapezium (Area)");
        println!("2. Rhombus (Area)");
        println!("3. Parallelogram (Area)");
        println!("4. Cube (Surface Area)");
        println!("5. Cylinder (Volume)");
        println!("6. Exit");
        println!("Enter your choice (1-6):");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read input");
        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number!\n");
                continue;
            }
        };

        match choice {
            1 => calc_trapezium(),
            2 => calc_rhombus(),
            3 => calc_parallelogram(),
            4 => calc_cube(),
            5 => calc_cylinder(),
            6 => {
                println!("Exiting Shape Calculator. Goodbye!");
                break;
            }
            _ => println!("Invalid choice! Please select between 1 and 6.\n"),
        }
    }
}