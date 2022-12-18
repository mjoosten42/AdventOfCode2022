use std::collections::HashSet;

use glam::IVec3;

fn main() {
	let args: Vec<String> = std::env::args().collect();
	let contents = std::fs::read_to_string(&args[1]).unwrap();

	let mut cubes: HashSet<IVec3> = HashSet::new();

	for line in contents.lines() {
		let words: Vec<i32> = line.split(",").map(|s| s.parse::<i32>().unwrap()).collect();
	
		cubes.insert(IVec3::new(words[0], words[1], words[2]));
	}

	println!("{}", surface(&cubes));

}

fn surface(cubes: &HashSet<IVec3>) -> i32 {
	cubes.iter().fold(0, |sum, cube| {
		let adj = [
			IVec3::new(1, 0, 0),
			IVec3::new(-1, 0, 0),
			IVec3::new(0, 1, 0),
			IVec3::new(0, -1, 0),
			IVec3::new(0, 0, 1),
			IVec3::new(0, 0, -1),
		];

		sum + adj.iter().fold(0, |total, face| total + !cubes.contains(&(*cube + *face)) as i32)
	})
}
