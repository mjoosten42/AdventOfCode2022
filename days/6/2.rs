use std::collections::{HashSet, VecDeque};

fn main() {
	let contents = std::fs::read_to_string(&std::env::args().collect::<Vec<String>>()[1]).unwrap();
	let size = 14;
	let mut last: VecDeque<u8> = Default::default();

	for i in 0..contents.len() {
		last.push_back(contents.as_bytes()[i]);
		if last.len() == size {
			let mut unique = HashSet::new();
			if last.iter().all(|x| unique.insert(x)) {
				return println!("{}", i + 1);
			}
			last.pop_front();
		}
	}
}
