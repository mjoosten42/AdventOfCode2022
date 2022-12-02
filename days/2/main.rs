use core::panic;

fn main() {
	let contents = std::fs::read_to_string("input.txt").unwrap();
	let mut result: u32 = 0;

	for line in contents.lines() {
		result += calc(line);
	}

	println!("{}", result);
}

fn calc(line: &str) -> u32 {
	let outcome = line.chars().last().unwrap();
	let their_score: u32 = (line.chars().next().unwrap() as u8 - b'A' + 1) as u32;
	let mut result = 0;

	match outcome {
		'X' => {
			result += 0;
			if their_score == 1 {
				result += 3;
			}
			else {
				result += (their_score - 1) as u32;
			}
			},
		'Y' => result += 3 + (their_score)  as u32,
		'Z' => {
			result += 6;
			if their_score == 3 {
				result += 1;
			}
			else {
				result += (their_score + 1) as u32;
			}
		},
		_ => panic!(),
	}

	println!("{} {}: {}", their_score, outcome, result);

	result

}