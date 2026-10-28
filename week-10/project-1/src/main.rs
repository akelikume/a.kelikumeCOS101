struct Electronics{
    brand:String,
    quantity:u32,
    price:u32
}
impl Electronics{
    fn calculate_cost(&self, quantity: u32) -> u32 {
    self.price * quantity
      }
}
fn main() {
   let hp = Electronics{   
    brand:String::from("HP"),
    quantity:10,
    price:650_000
   };
   let ibm = Electronics{
    brand:String::from("IBM"),
    quantity:6,
    price:755_000
   };
   let toshiba = Electronics{
    brand:String::from("Toshiba"),
    quantity:10,
    price:550_000
   };
   let dell = Electronics{
   brand:String::from("Dell"),
   quantity:4,
   price:850_000
   };
   let qty_to_buy = 3;
   let hp_cost = hp.calculate_cost(qty_to_buy);
   let ibm_cost = ibm.calculate_cost(qty_to_buy);
   let toshiba_cost = toshiba.calculate_cost(qty_to_buy);
   let dell_cost = dell.calculate_cost(qty_to_buy);
   let total_cost = hp_cost + ibm_cost + toshiba_cost + dell_cost;
   println!("--- Alaba Electronics ---");
    println!("HP Cost (3 Laptops): ₦{}", hp_cost);
    println!("IBM Cost (3 Laptops): ₦{}", ibm_cost);
    println!("Toshiba Cost (3 Laptops): ₦{}", toshiba_cost);
    println!("Dell Cost (3 Laptops): ₦{}", dell_cost);
    println!("-----------------------------------------------");
    println!("Total Cost For All Four Brands: ₦{}", total_cost);
}
