#[derive(Debug)]
struct Person {
	name: String,
	age: u8,
}

// A unit struct
struct Unit;

// A tuple struct
struct Pair(i32, f32);

// A struct with two fields
struct Point {
	x: f32,
	y: f32,
}

//structs can be reused as fields of another struct
#[allow(dead_code)]
struct Rectangle {
	// A rectangle can be specified by where the top left and bottom right
	// corners are in space.
	top_left: Point,
	bottom_right: Point,
}

fn main() {
	println!("");
	println!("=====Rust by Example=====");
	println!("======== structs ========");
	println!("");

	// Create a struct with field init shorthand
	let name =String::from("Eric");
	let age = 31;
	let eric = Person { name, age };

	// Print debug struct
	println!("{:?}", eric);

	// Init a point
	let point1: Point = Point { x: 10.3, y: 0.4 };

	// Access fields
	println!("Point coordinates: ({}, {})", point1.x, point1.y);

	// Make a new point by using struct update syntax to use the fields of our
	// other one
	let bottom_right = Point { x: 5.2, ..point1 };

	// 'bottom_right.y' will be the same as 'point1.y' bc we used that field from
	// point1
	println!("second point: ({}, {})", bottom_right.x, bottom_right.y);

	// Destructure teh point using a 'let' binding
	let Point { x:top_edge, y: left_edge } = point1;

	let _rectangle = Rectangle {
		// struct instantiation is an expression too
		top_left: Point { x: left_edge, y: top_edge },
		bottom_right: bottom_right,
	};

	// Instantiate a unit struct
	let _unit = Unit;

	// Instantiate a tuple struct
	let pair = Pair(1, 0.1);

	// Access the fields of a tuple struct
	println!("pair contains {:?} and {:?}", pair.0, pair.1);

	// Destructure a tuple struct
	let Pair(integer, decimal) = pair;

	println!("pair contains {:?} and {:?}", integer, decimal);

	println!("area of the rectangle: {}", rect_area(&_rectangle));
	let sq1 = Point{ x: 0.0, y: 5.5 };
	let sq2 = Point{ x: 5.5, y: 0.0 };
	let square = Rectangle{ top_left: sq1, bottom_right: sq2 };
	println!("area of squared rectangle: {}", rect_area(&square));

	let sq2_dim: f32 = 1.0;
	let square2 = make_square(&point1, sq2_dim);
	println!("square2 bottom_left: ({}, {})", point1.x, point1.y);
	println!("square2 dimension: {}", sq2_dim);
	// Destructure the points of the new square
	let Point{ x: btm_rt_x, y: btm_rt_y } = square2.bottom_right;
	let Point{ x: top_lt_x, y: top_lt_y } = square2.top_left;
	println!("square2 bottom_right: ({}, {})", btm_rt_x, btm_rt_y);
	println!("square2 top left: ({}, {})", top_lt_x, top_lt_y);
	println!("square2 area = {}", rect_area(&square2));

	println!("");
	println!("========== END ==========");
	println!("");
}

fn rect_area(rect: &Rectangle) -> f32 {
	let Rectangle{ top_left, bottom_right } = rect;
	let length = bottom_right.x - top_left.x;
	let height = top_left.y - bottom_right.y;
	length * height
}

fn make_square(start: &Point, dim: f32) -> Rectangle {
	let top_left = Point{ x: start.x, y: start.y + dim };
	let bottom_right = Point{ x: start.x + dim, y: start.y };
	Rectangle { top_left: top_left, bottom_right: bottom_right }
}
