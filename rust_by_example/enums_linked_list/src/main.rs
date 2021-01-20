use crate::List::*;

// this enum contains all the types of nodes our list will have (2)
enum List {
	// Cons: Tuple struct that wraps an element and a ptr to the next node
	Cons(u32, Box<List>),
	// Nil: A a node that signigies the end of the linked list
	Nil,
}

// Methods can be attached to an enum
impl List {
	// Create an empty list
	fn new() -> List {
		//`Nil` has type `List`
		Nil
	}

	// Consume a list and return the same list with a new element at its front
	fn prepend(self, elem: u32) -> List {
		// `Cons` also has type	`List`
		Cons(elem, Box::new(self))
	}

	// Return length of the list
	fn len(&self) -> u32 {
		// `self` has to be matched b/c the behavior of this method depends on the
		// variant of `self`
		// `self` has type `&List` and `*self` has type `List`, matching on a
		// concrete type `T` is preferred over a match on a reference `&T`
		match *self {
			// Can't take ownership of the tail b/c `self` is borrowed;
			// instead take a reference to the tail
			Cons(_, ref tail) => 1 + tail.len(), // recursive counting!
			// Base case: An empty list has zero length
			Nil => 0
		}
	}

	// Return representation of the list as a (heap allocated) string
	fn stringify(&self) -> String {
		match *self {
			Cons(head, ref tail) => {
				// `format!` is similar to `print!` but returns a heap
				// allocated string instead of printing to the console
				format!("{} >> {}", head, tail.stringify())
			},
			Nil => {
				format!("Nil")
			},
		}
	}
}

fn main() {
	println!("");
	println!("=====Rust by Example=====");
	println!("=== enums: linked list ==");
	println!("");

	// Create an empty list
	let mut list = List::new();
	println!("Linked list has initial length: {}", list.len());
	println!("Inital linked list:\n{}", list.stringify());


	// prepend some elements
	list = list.prepend(1);
	list = list.prepend(2);
	list = list.prepend(3);

	// Show the final state of the list
	println!("Linked list has final length: {}", list.len());
	println!("Final linked list\n{}", list.stringify());

	println!("");
	println!("========== END ==========");
	println!("");
}
