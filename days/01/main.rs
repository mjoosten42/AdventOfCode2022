use std::fs;

fn main() {
	let contents = fs::read_to_string("input.txt").unwrap();
	let elfs: Vec<&str> = contents.split("\n\n").collect();

	let mut fruit: Vec<Vec<&str>> = Vec::new();
	for str in elfs {
		fruit.push(str.split("\n").collect());
	}

	let mut total: Vec<i32> = Vec::new();
	for elf in fruit {
		let mut calories = 0;
	
		for f in elf {
			if !f.is_empty() {
				calories += f.parse::<i32>().unwrap();
			}
		}

		total.push(calories);
	}

	total.sort();

	let three = total.pop().unwrap() + total.pop().unwrap() + total.pop().unwrap();

	print!("{}\n", three);

}
