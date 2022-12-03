use std::collections::BTreeSet;

fn main() {
	let args: Vec<String> = std::env::args().collect();
	let contents = std::fs::read_to_string(&args[1]).unwrap();
	let mut items:BTreeSet<char> = BTreeSet::new();
	let mut result = 0;

	for rucksack in contents.lines() {
		for c in rucksack.char_indices() {
			if c.0 < rucksack.len() / 2 {
				items.insert(c.1);
			}
			else {
				if items.contains(&c.1) {
					result += item_value(c.1);
					break ;
				}
			}
		}
		items.clear();
	}

	println!("{}", result);
}

fn item_value(c: char) -> u32 {
	if c.is_lowercase() {
		c as u32 - 'a' as u32 + 1
	}
	else {
		c as u32 - 'A' as u32 + 26 + 1
	}
}