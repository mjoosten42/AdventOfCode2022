fn main() {
	let contents = std::fs::read_to_string(&std::env::args().collect::<Vec<String>>()[1]).unwrap();
	let mut visited: Vec<(i32, i32)> = Vec::new();
	const SIZE: usize = 9;

	let mut tails: [(i32, i32); SIZE + 1] = [(0, 0); SIZE + 1];

	for line in contents.lines() {
		let words: Vec<&str> = line.split_whitespace().collect();
		for _ in 0..words[1].parse::<i32>().unwrap() {
			match words[0] {
				"R" => tails[0].0 += 1,
				"U" => tails[0].1 += 1,
				"L" => tails[0].0 -= 1,
				"D" => tails[0].1 -= 1,
				_ => ()
			}
			for i in 0..SIZE {
				tails[i + 1] = knot(&tails[i], tails[i + 1]);
			}
			visited.push(tails[SIZE]);
		}
	}

	visited.sort();
	visited.dedup();

	println!("{}", visited.len());
}

fn knot(head: &(i32, i32), mut tail: (i32, i32)) -> (i32, i32) {
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
	tail
}
