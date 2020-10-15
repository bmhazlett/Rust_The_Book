fn main() {
    let number = 3;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    // ternary ?
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {}", number);

    // Simple Loop
    let mut counter = 0;

    let result = loop {
	counter += 1;

	if counter == 10 {
	    break counter * 2;
	}
    };

    println!("The result is {}", result);

    // while loop
    let mut number = 3;
    while number != 0 {
	println!("{}!", number);
	number -= 1;
    }

    println!("LIFTOFF!!!!");

    // for loop
    let a = [10, 20, 30, 40, 50];
    for element in a.iter() {
	println!("The value is: {}", element);
    }

    // for loop with Range
    for number in (1..4).rev() {
	println!("{}!", number);
    }
    println!("LIFTOFF!!!");
    
}
