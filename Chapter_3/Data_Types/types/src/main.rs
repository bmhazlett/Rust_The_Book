fn main() {
    // Integer Data Types
    // i8 u8 (Signed and Unsigned 8-bit integers)
    // i16 u16 (Signed and Unsigned 16-bit integers)
    // i32 u32 (Signed and Unsigned 32-bit integers)
    // i64 u64 (Signed and Unsigned 64-bit integers)
    // i128 u128 (Signed and Unsigned 128-bit integers)
    // isize usize (Signed and Unsigned arch integers)
    let guess: u32 = "42".parse().expect("Not a number!");
    println!("The value of guess is {}", guess);

    // Decimal numbers 98_222
    let decimal: i32 = 98_222;
    println!("The value of decimal is {}", decimal);	
    // Hex 0xff
    let hex: u16 = 0xff;
    println!("The value of hex is {}", hex);
    // Octal 0o77
    let oct: i16 = 0o77;
    println!("The value of oct is {}", oct);
    // Binary 0b1111_0000
    let bin: u8 = 0b1111_0000;
    println!("The value of bin is {}", bin);
    // Byte b'A'
    let chomp: u8 = b'A';
    println!("the value of chomp is {}", chomp);

    // Floating points f32 and f64
    let x = 2.0; // f64
    let y: f32 = 3.0;
    println!("The value of x is {}", x);
    println!("The value of y is {}", y);

    // addition
    let sum = 5 + 10;
    println!("The value of sum is {}", sum);

    // subtraction
    let difference = 95.5 - 4.3;
    println!("The value of difference is {}", difference);

    // multiplication
    let product = 4 * 30;
    println!("The value of product is {}", product);

    // division
    let quotient = 56.7 / 32.2;
    println!("The value of quotient is {}", quotient);

    // remainder
    let remainder = 43 % 3;
    println!("The value of remainder is {}", remainder);
    
    // boolean
    let t = true;
    let f: bool = false;
    println!("True {}, False {}", t, f);

    // character type
    let c = 'z';
    let z = 'Z';
    let heart_eyed_cat = '😻';
    println!("The value of c is {}", c);
    println!("The value of z is {}", z);
    println!("The value of heart_eyed_cat is {}", heart_eyed_cat);

    // Tuple
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (a, b, c) = tup;
    println!("The value of b is: {}", b);
    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = x.2;

    // Array
    let a = [1, 2, 3, 4, 5];
    let months = ["Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sept", "Oct", "Nov", "Dec"];
    // Five values all signed 32-bit ints
    let new_a: [i32; 5] = [1, 2, 3, 4, 5];
    // [3, 3, 3, 3, 3]
    let newer_a = [3; 5];

    let jan = months[0];
    let feb = months[1];

    // OUT OF INDEX
    // a[6] -> causes runtime error
    
			  
	
    
    
}
