extern crate unicode_segmentation;

use unicode_segmentation::UnicodeSegmentation;
use std::str;

fn main() {
	let name = "하효설";
	let graphemes = UnicodeSegmentation::graphemes(name,
	true).collect::<Vec<&str>>();

	for c in graphemes {
		println!("{}", c);
	}

	for d in name.chars() {
		println!("{}", d);
	}

	if graphemes[2] == "설" {
		println!("graphemes[3] is Seol!");
	}

	let name2_chars = "Eric".chars();
	if name2_chars[2] == 'i' {
		println!(".chars() outputs a char");
	}
}
