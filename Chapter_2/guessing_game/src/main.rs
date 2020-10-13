// Get the std io library
use std::io;
use std::cmp::Ordering;
// Get the rand dependency
use rand::Rng;

fn main() {
    // Macro to write to the screen
    println!("Guess the number!");

    // We use the thread_rng() function from the rand library
    // Then we call gen_range to get a random integer
    let secret_number = rand::thread_rng().gen_range(1,101);
    //    println!("The secret number is: {}", secret_number);

    // This is an infinite loop
    loop {
	println!("Please input your guess.");

	// Create new variable that is mutable to a type of string
	// :: new is an associated function of the String type (think static method)
	// new creates a new empty string
	// line has created a mutable variable that is currently bound to a new
	// empty instance of a String
	let mut guess = String::new();
	
	// This is where we use the "use std::io"
	// We could have written it as std::io::stdin
	// read_line method to handled getting input and is passed the mutable string guess
	// & says the arguement is a reference (which is immuatable by default)
    // expect will help handle failure it will get the result of the .read_line
	// this result will be Ok or Err
	// expect will print out that message on Err
	io::stdin()
	    .read_line(&mut guess) 
	    .expect("Failed to read line");

	// Convert from string to int (if not ok coninue) else set the value
	let guess: u32 = match guess.trim().parse() {
	    Ok(num) => num,
	    Err(_) => continue,
	};
	
	// println! placeholders
	// {} will be replaced by guess
	println!("You guess: {}", guess);

	// Check if guess is equal to the secret number
	// You then have less, greater, and equal which will do different things
	match guess.cmp(&secret_number) {
	    Ordering::Less => println!("Too small!"),
	    Ordering::Greater => println!("Too big!"),
	    Ordering::Equal => {
		println!("You win!");
		break;
	    }
	}
    }
}
