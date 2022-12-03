fn main() {
	let args: Vec<String> = std::env::args().collect();
	let contents = std::fs::read_to_string(&args[1]).unwrap();
	let mut result = 0;

	println!("{}", result);
}
