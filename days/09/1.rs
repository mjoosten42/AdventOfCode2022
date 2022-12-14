fn main() {
	let contents = std::fs::read_to_string(&std::env::args().collect::<Vec<String>>()[1]).unwrap();
	let mut visited: Vec<(i32, i32)> = Vec::new();

	let mut head = (0, 0);
	let mut tail = (0, 0);

	for line in contents.lines() {
		let words: Vec<&str> = line.split_whitespace().collect();
		for _ in 0..words[1].parse::<i32>().unwrap() {
			match words[0] {
				"R" => head.0 += 1,
				"U" => head.1 += 1,
				"L" => head.0 -= 1,
				"D" => head.1 -= 1,
				_ => ()
			}
			let mut tmp: (i32, i32) = (head.0 - tail.0, head.1 - tail.1);
			if tmp.0.abs() + tmp.1.abs() == 3 {
				if tmp.0.abs() == 1 {
					tmp.0 *= 2;
				} else {
					tmp.1 *= 2;
				}
			}
			tail.0 += tmp.0 / 2;
			tail.1 += tmp.1 / 2;
			visited.push(tail);
		}
	}

	visited.sort();
	visited.dedup();

	println!("{}", visited.len());
}
