
use std::io::Write;
fn main() {
    let student_name = vec!["Oluchi Mardi","Adams Aliyu","Shania Bolade", "Adekunle Gold", "Bianca Edemoh"];
    let matric_number = vec! ["ACC10211111","ECO10110101","CSC10328828","EEE11020202","MEE10202001"];
    let department = vec!["Accounting","Economics","Computer Science","Electrical Engineering","Mechanical Engineering"];
    let level = vec!["300","100","200","200","100"];
    let mut file = std::fs::File::create("PAU-SMIS.csv").expect("create failed");
   file.write_all ("Student Name,Matric Number,Department,Level\n"
        .as_bytes()).expect("write failed");
   println!("Welcome to PAU's Student Management Information System");
    for i in 0..student_name.len(){ 
        file.write_all(student_name[i].as_bytes()).expect("write failed");
        file.write_all(",".as_bytes()).expect("write failed");
    file.write_all(matric_number[i].as_bytes()).expect("write failed");
    file.write_all(",".as_bytes()).expect("write failed");
    file.write_all(department[i].as_bytes()).expect("write failed");
    file.write_all(",".as_bytes()).expect("write failed");
    file.write_all(level[i].as_bytes()).expect("write failed");
    file.write_all("\n".as_bytes()).expect("write failed");
    println!("Student Name: {}", student_name[i]);
        println!("Matric Number: {}", matric_number[i]);
        println!("Department: {}", department[i]);
        println!("Level: {}", level[i] );
    }
        

    println!("\nData written to file");    
}