use std::io;

fn main() {
    println!("Hello, world!");
    let mut input = String::new();

    println!("Please enter your name: ");
    io::stdin()
    .read_line(&mut input) // fungsi ada &mut adalah untuk mengakses variabel mutable sehingga nilaij sebelumnya dapat diubah dan tidak hanya di salin 
    .expect("Failed to read line");
    println!("You typed: {}", input);
}
