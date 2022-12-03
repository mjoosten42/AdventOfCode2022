use std::collections::BTreeSet;

fn main() {
	let args: Vec<String> = std::env::args().collect();
	let contents = std::fs::read_to_string(&args[1]).unwrap();
	const RUCKSACK_SIZE: usize = 3;
	let mut items: [BTreeSet<char>; RUCKSACK_SIZE] = Default::default();
	let mut result = 0;
	let mut line = 0;

	for rucksack in contents.lines() {
		for c in rucksack.chars() {
			items[line].insert(c);
		}
		line += 1;
		if line == RUCKSACK_SIZE {
			items[0] = items[0].intersection(&items[1]).cloned().collect();
			items[0] = items[0].intersection(&items[2]).cloned().collect();

			result += item_value(*items[0].iter().next().unwrap());

			for n in 0..RUCKSACK_SIZE {
				items[n].clear();
			}
			line = 0;
		}
	}

	println!("{}", result);
}

fn item_value(c: char) -> u32 {
	if c.is_lowercase() {
		(c as u8 - b'a' + 1) as u32
	}
	else {
		(c as u8 - b'A' + 26 + 1) as u32
	}
}