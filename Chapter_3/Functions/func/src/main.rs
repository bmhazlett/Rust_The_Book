fn main() {
    println!("Hello, World!");
    another_function();
    para_func(5);

    let x = 5;
    let y = {
	let x = 3;
	x + 1
    };

    println!("The value of main x is {}", x);
    println!("The value of y is {}", y);

    let ret = return_func();
    println!("The value of ret is: {}", ret);

    let add = plus_one(1);
    println!("The value of add is: {}", add);
	     
}

fn another_function() {
    println!("Another function.");
}

fn para_func(x: i32) {
    println!("The value of func x is: {}", x);
}


fn return_func() -> i32 {
    println!("WOOOW");
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
