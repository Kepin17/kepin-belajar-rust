fn main() {

    // print
    println!("Hello, world!");

    // variable with mutable
    let mut name= "Kepin";
    println!("Hi, my name is {}", name);
    name = "Jarvis";
    println!("Hi, my name is {}", name);
    
    // variable with immutable
    let name= "Kepin";
    println!("Hi, my name is {}", name);
    let name = "Jarvis";
    println!("Hi, my name is {}", name);

    let x = 5;
    println!("The value of x is: {}", x);
    {
        let x =x +  3;
        println!("The value of x is: {}", x);

        let x = x + 1;
        println!("The value of x is: {}", x);
    }

    let x = x + 1;
    print!("The value of x is: {}", x);


    // const value

    const SECONDS_IN_MINUTE: u32 = 60;
    println!("{}", SECONDS_IN_MINUTE);
}
