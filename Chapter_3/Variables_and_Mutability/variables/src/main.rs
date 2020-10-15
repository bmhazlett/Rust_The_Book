fn main() {
    // mut means mutable
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is {}", x);

    // const are always immutable
    const MAX_POINTS: u32 = 100_000;
    println!("The value of MAX_POINTS is {}", MAX_POINTS);

    // Shadowing
    let y = 5;
    let y = y + 1;
    let y = y + 2;

    println!("The value of y is: {}", y);

    // Shadowing lets you change varialbe 'types'
    let spaces = "    ";
    println!("The value of spaces is {} <END>", spaces);
    let spaces = spaces.len();
    println!("The value of spaces is {}", spaces);
    

}
