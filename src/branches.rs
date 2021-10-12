#[allow(unused)]
pub fn run() {
    let n = 3;
    if n < 5 {
        println!("true");
    } else {
        println!("false")
    }

    // err: expected 'bool', found integer
    // if n {
    //     println!("number was three")
    // }

    // infinite loop
    // loop {
    //     println!("again!")
    // }

    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("result = {}", result); // result = 20

    // the lame way
    println!("starting lame liftoff");
    let mut n = 3;
    while n != 0 {
        println!("{}", n);
        n -= 1;
    }
    println!("LIFTOFF!");

    // the cooler and cleaner way
    println!("Starting cooler liftoff");
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!");

    let a = [10, 20, 30 , 40 , 50];
    // following is bad, error prone and slow since the compiler
    // adds checks every time for an index out of bounds
    let mut index = 0;
    while index < 5 {
        println!("the value is: {}", a[index]);
        index += 1;
    }

    // better
    for element in a.iter() {
        println!("for loop values: {}", element);
    }


}