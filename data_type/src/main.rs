fn main() {
    // primitive data types : 
    // - scalar : int, float, bool, char
    // - compound : array, tuple
    
    // int data type: i8, u8, i16, u16, i32, u32, i64, u64, i128, u128

    let x: i32 = 2;
    println!("{}" , x);

    // float data type: f32, f64

    let float_point: f32 = 2.0;
    println!("{}" , float_point);

    // bool data type: true, false

    let boolean: bool = true;
    println!("{}" , boolean);


    let kata: char = 'a';
    println!("{}" , kata);


    // compound data types
    // array
    // tuple

    // array
    let array = [1, 2, 3, 4, 5];
    println!("{}" , array[1]);

    // tuple
    let tuple:(i32 , char, bool) = (1, 's' , false);

    println!("{}" , tuple.2);
}
