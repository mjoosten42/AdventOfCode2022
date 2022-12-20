const INDICES: [usize; 3] = [1000, 2000, 3000];

fn main() {
	let args: Vec<String> = std::env::args().collect();
	let contents = std::fs::read_to_string(&args[1]).unwrap();

	let file: Vec<i32> = contents.lines().map(|line| line.parse().unwrap()).collect();

	let mut mix = file.clone();

	for n in &file {
		let index = mix.iter().position(|&p| p == *n).unwrap();
		let mut target = index as i32 + n;

		mix.remove(index);
	
		let len: i32 = mix.len() as i32;

		if target > len {
			target %= len;
		}

		if target < 0 {
			target %= len;
			target += len;
		}
	
		mix.insert(target as usize, *n);

	}

	let zero = mix.iter().position(|&p| p == 0).unwrap();

	println!("{}", INDICES.iter().fold(0, |sum, i| {
		let index = (zero + i) % file.len();
		sum + mix[index]
	}));
}
