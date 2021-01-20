use std::io;

fn main() {
	//let n: u32 = 7;

	println!("Calculate nth fiboacci number.");
	println!("Enter n:");

	let mut n = String::new();

	io::stdin()
		.read_line(&mut n)
		.expect("Failed to read stdin");

	let n: i32 = n.trim().parse().expect("Not a number");

	// use a recursive function call
	let nth_fib = fib(n-1);
	println!("{}th fib is {}", n, nth_fib);

	// use a while loop
	let mut count: i32 = 0;
	let mut fn2 = 0;
	let mut fn1 = 1;
	let mut fn0 = 1;
	while count < n-1 {
		fn0 = fn1 + fn2;
		fn2 = fn1;
		fn1 = fn0;
		count += 1;
	}
	println!("{}th fib is {}", n, fn0);
}


fn fib(x: i32) -> i32 {
	if x < 2 {
		return 1
	}

	fib(x-1) + fib(x-2)
}
