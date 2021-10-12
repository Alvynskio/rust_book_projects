pub fn run() {
    // mutable variables -- if 'mut' is removed, it will err
    let mut x = 5;
    println!("x = {}", x);
    x = 6;
    println!("x = {}", x);

    const MAX_POINTS: u32 = 100_000;

    println!("mp = {}", MAX_POINTS);

    // shadowing ...
    let y = 4;
    let y = y + 2;
    let y = y * 3;

    println!("y = {}", y);

    // another use for shadowing
    let spaces = "   ";
    let spaces = spaces.len(); // now a different type

    println!("spaces = {}", spaces);
}