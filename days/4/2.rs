use regex;

fn main() {
	let mut result = 0;

	std::fs::read_to_string(&std::env::args().collect::<Vec<String>>()[1]).unwrap().lines().for_each(|x| {
		let array: Vec<u32> = regex::Regex::new(r",|-").unwrap().split(x).map(|y| y.parse::<u32>().unwrap()).collect();
		if array[0] <= array[3] && array[2] <= array[1] {
			result += 1;
		}
	}
	);

	println!("{}", result);
}
