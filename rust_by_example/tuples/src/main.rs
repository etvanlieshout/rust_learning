use std::fmt::{self, Formatter, Display};

// Tuples can be used as function args and as retvals
fn reverse( pair: (i32, bool)) -> (bool, i32) {
	// 'let' can be used to bind the members of a tuple to variables
	let (integer, boolean) = pair;

	(boolean, integer)
}

// The following struct is for the exercise
#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

impl Display for Matrix {
	// 'f' is a buffer; this method writes the formatted str to it
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		let row1 = (self.0, self.1);
		let row2 = (self.2, self.3);

		write!(f, "Matrix:\n{:?} \n{:?}", row1, row2)
	}
}

fn main() {

	println!("");
	println!("=====Rust by Example=====");
	println!("======== tuples  ========");
	println!("");

	// A tuple with a bunch of different types
	let long_tuple = (1u8, 2u16, 3u32, 4u64,
										-1i8, -2i16, -3i32, -4i64,
										0.1f32, 0.2f64,
										'a', true);

	// values can be extracted from the tuple using indexing
	println!("long tuple first value: {}", long_tuple.0);
	println!("long tuple second value: {}", long_tuple.1);

	// tuples can be members of tuples
	let tuple_of_tuples = ((3u8, 4u16, 5u32), (6u64, -3i8), -4i16);

	// Tuples are printable
	println!("tuple of tuples: {:?}", tuple_of_tuples);

	// But only within reason
	// let too_long_tuple = (1,2,3,4,5,6,7,8,9,10,11,12,13);
	// println!("Too long tuple: {:?}", too_long_tuple);
	// ^^ uncomment to see compiler error

	// Tuples as args:
	let pair = (1, true);
	println!("pair is {:?}", pair);
	println!("reversed pair is {:?}", reverse(pair));

	// comma is required for single element tuples aka singletons
	println!("One element tuple: {:?}", (5u32,));
	println!("versus just an int: {}", (5u32));

	// tuples can be destructured to create bindings
	let a_tuple = (1, "hello", 4.5, true);
	let (a,b,c,d) = a_tuple;
	println!("{:?}, {:?}, {:?}, {:?}", a,b,c,d);

	let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
	println!("Tuple Matrix Debug fmt: {:?}", matrix);
	println!("{}", matrix);
	println!("Transpose {}", transpose(matrix));

	println!("");
	println!("========== END ==========");
	println!("");
}

fn transpose(m: Matrix) -> Matrix {
	let Matrix(a,b,c,d) = m;

	Matrix(a,c,b,d)
}

