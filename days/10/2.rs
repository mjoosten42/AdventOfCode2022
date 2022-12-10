fn main() {
	let contents = std::fs::read_to_string(&std::env::args().collect::<Vec<String>>()[1]).unwrap();
	let lines: Vec<&str> = contents.lines().collect();
	let mut x = 1;
	let mut index = 0;
	let mut remaining = 0;
	const CYCLES: u32 = 240;
	const WIDTH: u32 = 40;

	for cycle in 0..CYCLES {
		let words: Vec<&str> = lines[index].split_whitespace().collect();

		// Load
		if remaining == 0 {
			match words[0] {
				"addx" => remaining = 2,
				_ => remaining = 1,		
			}
		}

		if ((cycle % WIDTH) as i32 - x).abs() < 2 {
			print!("#");
		} else {
			print!(".");
		}
		if (cycle + 1) % WIDTH == 0 {
			println!("");
		}
	
		// Execute
		remaining -= 1;

		// Finish
		if remaining == 0 {
			if lines[index].starts_with("addx") {
				x += words[1].parse::<i32>().unwrap();
			}
			index += 1;
		}
	}

}
