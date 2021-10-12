#[allow(unused)]
pub fn run() {
    { // s not valid here, not yet declared
        let mut s = "hello"; // s now valid
        s = "hi";
        // s = s + "5"; // will err
        s = &(s.to_owned() + "5");

        // do stuff with s
    } // this scope is over, and s is no longer valid

    let s = String::from("hello"); // immutable, on the heap
    let mut s = String::from("hello");
    s.push_str(", world"); // appends a literal to a String
    print!("{}", s);


    let s2 = s.clone();
    println!("s2 is '{}', and s is '{}'", s2, s);

}