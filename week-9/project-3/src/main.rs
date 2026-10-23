
use std::io::Write;
fn main() {
    let commisioner = vec!["Aigbogun Alamba Daudu","Murtala Afeez Bendu ","Okorocha Calistus Ogbona", "Adewale Jimoh Akanbi", "Osazuwa Faith Etieye"];
    let ministry = vec! ["Internal Affairs","Justice","Defense","Power & Steel","Petroleum"];
    let zone = vec!["South West","North East","South South","South West","South East"];
    let mut file = std::fs::File::create("The EFCC Merge.csv").expect("create failed");
   file.write_all ("Commisioner,Ministry,Zone\n"
        .as_bytes()).expect("write failed");
   println!("Welcome to The EFCC Merge");
    for i in 0..commisioner.len(){ 
        file.write_all(commisioner[i].as_bytes()).expect("write failed");
        file.write_all(",".as_bytes()).expect("write failed");
    file.write_all(ministry[i].as_bytes()).expect("write failed");
    file.write_all(",".as_bytes()).expect("write failed");
    file.write_all(zone[i].as_bytes()).expect("write failed");
    file.write_all("\n".as_bytes()).expect("write failed");
    println!("Commisioner: {}", commisioner[i]);
        println!("Ministry: {}", ministry[i]);
        println!("Zone: {}", zone[i]);
    }
        

    println!("\nData written to file");    
}