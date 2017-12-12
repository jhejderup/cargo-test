extern crate rand;


fn main() {
    let tuple = rand::random::<(f64, char)>();
    println!("{:?}", tuple);
    let tuple2 = rand::random::<(f64, char)>();
    println!("{:?}", tuple2);

    
}






