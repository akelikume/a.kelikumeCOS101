// Rust program to find the roots of a quadratic equation

use std::io;

fn main() {
	let mut input1 = String::new();
	let mut input2 = String::new();
	let mut input3 = String::new();

        println!("Enter the coefficient a");
		io::stdin().read_line(&mut input1).expect("Not a valid string");
		let a:f32 = input1.trim().parse().expect("Not a valid number");

		println!("Enter the coefficient b");
		io::stdin().read_line(&mut input2).expect("Not a valid string");
		let b:f32 = input2.trim().parse().expect("Not a valid number");

		println!("Enter the coefficient c");
		io::stdin().read_line(&mut input3).expect("Not a valid string");
		let c:f32 = input3.trim().parse().expect("Not a valid number"); 

		if a == 0.0 {
		println!("This is not a quadratic equation (a cannot be 0).");
		return;
		}
		
        // Calculate discriminant: b^2 - 4ac
		let discriminant: f32 = (b * b)- (4.0 *a * c);

		if discriminant > 0.0 {
        // Case 1: Two Distinct Real Roots
        let root1 = (-b + discriminant.sqrt()) / (2.0 * a);
        let root2 = (-b - discriminant.sqrt()) / (2.0 * a);
        println!("Discriminant is positive ({})", discriminant);
        println!("Two distinct real roots:");
        println!("Root 1 = {}", root1);
        println!("Root 2 = {}", root2);

    } else if discriminant == 0.0 {
        // Case 2: One Double (Repeated) Real Root
        let root = -b / (2.0 * a);
        println!("Discriminant is zero ({})", discriminant);
        println!("One repeated real root (equal roots):");
        println!("Root 1 = Root 2 = {}", root);

    } else {
        // Case 3: Complex / Imaginary Roots (Discriminant < 0)
        let real_part = -b / (2.0 * a);
        let imaginary_part = (-discriminant).sqrt() / (2.0 * a);
        println!("Discriminant is negative ({})", discriminant);
        println!("Two complex (imaginary) roots:");
        println!("Root 1 = {} + {}i", real_part, imaginary_part);
        println!("Root 2 = {} - {}i", real_part, imaginary_part);
    }
}