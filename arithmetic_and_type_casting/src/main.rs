use std::io;

fn main() {
    // let x = 100 as i32;
    // let y = 20_i8;
    // let z = x + (y as i32);
    // println!("{}" , z);

    let mut input_x = String::new();
    io::stdin().read_line(&mut input_x).expect("Failed to read line");
    let mut input_y = String::new();
    io::stdin().read_line(&mut input_y).expect("Failed to read line");

    let x: i64 = input_x.trim().parse().unwrap();
    let b: i64 = input_y.trim().parse().unwrap();
    
    let total = x + b;
    println!("{}" , total);

}
