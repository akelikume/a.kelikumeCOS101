fn main() {
    Let A:i32 = 10;
    Let B:i32 = 20;

    println!("Value of A:{} ",A);
    println!("Value of B : {} ",B);

    Let mut res = A>B ;
    println!("A greater than B: {} ",res);

    res = A<B ;
    println!("A lesser than B: {} ",res) ;

    res = A>=B ;
    println!("A greater than or equal to B: {} ",res);

    res = A<=B ;
    println!("A lesser than or equal to B: {}",res) ;

    res = A==B ;
    println!("A is equal to B: {}",res) ;

    res = A!=B ;
    println!("A is not equal to B: {} ",res);
}