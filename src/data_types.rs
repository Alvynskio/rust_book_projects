#[allow(unused_variables)]
pub fn run() {
    // two data type subsets: scalar and compound.
    // scalar %%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%

    let num_apples = 20; // default i32
    let height = 180.3; // default f64

    // other types
    let x: i8 = 2;
    let x: i16 =  20;
    let x: i32 = 5000;
    let x: i64 = 999_999_999;
    let x: i128 = 299999999; // this can be huge ...
    let x: isize = 2424; // architecture defined -- 32 or 64

    let x: u8 = 2;
    let x: u16 =  20;
    let x: u32 = 5000;
    let x: u64 = 999_999_999;
    let x: u128 = 299999999; // this can be huge ...
    let x: usize = 2424; // architecture defined -- 32 or 64

    let x: f32 = 1.999_875; // two's complement
    let x: f64 = 3.14159; // also two's complement
    let x = std::f64::consts::PI; // another example

    let decimal = 98_222;
    let hex = 0xff;
    let octal = 0o77;
    let binary = 0b1111_0000;
    let byte = b'A'; // u8 only

    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;

    // remainder
    let remainder = 43 % 5;

    let t = true;
    let f: bool = false; // with explicit type annotation

    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';

    // compound types %%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%
    // tuple
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("y = {}", y);
    // this won't work
    //println!("tup = {}", tup);
    println!("tup = {:?}", tup);

    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;

    // array
    let a = [1, 2, 3, 4, 5];
    // this won't work
    // println!("a = {}", a);
    // but this does
    println!("a = {:?}", a); // regular
    println!("a = {:#?}", a); // pretty print

    let months = ["January", "February", "March", "April", "May", "June", "July",
        "August", "September", "October", "November", "December"];

    println!("The first and third months of the year are {} and {} respectively.",
        months[0], months[2]);

    // this makes it panic, index out of bound
    // let thirteenth_month = months[12];
}