fn main() {
	let p_tosh:f64 = 450_000.00;
	let p_mac:f64 = 1_500_000.00;
	let p_hp:f64 = 750_000.00;
	let p_dell:f64 = 2_850_000.00;
	let p_acer:f64 = 250_000.00;

	let q_tosh:f64 = 2.0;
	let q_mac:f64 = 1.0;
	let q_hp:f64 = 3.0;
	let q_dell:f64 = 3.0;
	let q_acer:f64 = 1.0;

	let a_tosh = p_tosh * q_tosh;
	let a_mac = p_mac * q_mac;
	let a_hp = p_hp * q_hp;
	let a_dell = p_dell * q_dell;
	let a_acer = p_acer * q_acer;


	let sum = a_tosh + a_mac + a_hp + a_dell + a_acer;
	let tot_quan = q_tosh + q_mac + q_hp + q_dell + q_acer;

	println!("The sum of all products is {}", sum);
	let average = sum / tot_quan;
	println!("The average of all products is {}", average);
	
}