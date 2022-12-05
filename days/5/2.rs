use std::collections::VecDeque;

fn main() {
	let contents = std::fs::read_to_string(&std::env::args().collect::<Vec<String>>()[1]).unwrap();
	let mut cont: Vec<VecDeque<char>> = vec![VecDeque::new(); contents.find("\n").unwrap() / 4 + 1];

	for line in contents.split("\n\n").next().unwrap().lines() {
		let mut i = 0;
		for c in line.bytes().skip(1).step_by(4) {
			if c != b' ' {
				cont[i].push_back(c as char);
			}
			if c.is_ascii_digit() {
				cont[i].pop_back();
			}
			i += 1;
		}
	}
	
	for line in contents.split("\n\n").last().unwrap().lines() {
		let instr: Vec<usize> = line.split_whitespace().filter_map(|word|	word.parse().ok()).collect();
		let mut tmp: VecDeque<char> = VecDeque::new();
		for _ in 0..instr[0] {
			tmp.push_front(cont[instr[1] as usize - 1].pop_front().unwrap());
		}
		for _ in 0..instr[0] {
			cont[instr[2] as usize - 1].push_front(tmp.pop_front().unwrap());
		}
	}

	println!("{}", cont.iter().map(|f| f.front().unwrap()).collect::<String>());
}
