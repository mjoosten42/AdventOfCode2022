fn main() {
	let contents = std::fs::read_to_string(&std::env::args().collect::<Vec<String>>()[1]).unwrap();
	let mut cycle = 0;
	let mut x = 1;
	let mut sum = 0;

	for line in contents.lines() {
		let words: Vec<&str> = line.split_whitespace().collect();
		cycle += 1;
		if (cycle - 20) % 40 == 0 { sum += cycle * x; }
		match words[0] {
			"addx" => {
				cycle += 1;
				if (cycle - 20) % 40 == 0 { sum += cycle * x; }
				x += words[1].parse::<i32>().unwrap();
			}
			_ => (),			
		}
	}

	println!("{sum}");

}
