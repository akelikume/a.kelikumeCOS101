// Rust program to output name and age

use std::io;

fn main() {
    println!("\nStudent Information Management System!");

    // Input name
    println!("\nPlease enter your name:");
    let mut name = String::new();
    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read input");
    println!("Your name is: {}", name.trim());

    // Input age
    println!("\nEnter your age:");
    let mut age_input = String::new();
    io::stdin()
        .read_line(&mut age_input)
        .expect("Failed to read input");
    
    let age: i32 = age_input
        .trim()
        .parse()
        .expect("Input not an integer");
        
    println!("Your age is: {}", age);
}