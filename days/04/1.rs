use regex;

fn main() {
	let mut result = 0;

	for line in std::fs::read_to_string(&std::env::args().collect::<Vec<String>>()[1]).unwrap().lines() {
		let array: Vec<u32> = regex::Regex::new(r",|-").unwrap().split(line).map(|x| x.parse::<u32>().unwrap()).collect();
		if (array[0] <= array[2] && array[1] >= array[3]) || (array[0] >= array[2] && array[1] <= array[3]) {
			result += 1;
		}
	}

	println!("{}", result);
}
