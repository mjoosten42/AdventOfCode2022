use std::collections::HashSet;

use glam::IVec3;

fn main() {
	let args: Vec<String> = std::env::args().collect();
	let contents = std::fs::read_to_string(&args[1]).unwrap();

	let mut cubes: HashSet<IVec3> = HashSet::new();

	let mut max = IVec3::ZERO;
	for line in contents.lines() {
		let words: Vec<i32> = line.split(",").map(|s| s.parse::<i32>().unwrap()).collect();
	
		cubes.insert(IVec3::new(words[0], words[1], words[2]));

		if words[0] > max.x { max.x = words[0] };
		if words[1] > max.y { max.y = words[1] };
		if words[2] > max.z { max.z = words[2] };
	}

	let air = air_around(&cubes, IVec3::NEG_ONE, max + IVec3::ONE);

	println!("{}", surface(&cubes, &air));

}

fn air_around(cubes: &HashSet<IVec3>, min: IVec3, max: IVec3) -> HashSet<IVec3> {
	let mut air: HashSet<IVec3> = vec![IVec3::ZERO].into_iter().collect();
	let mut edges = vec![IVec3::ZERO];

	while !edges.is_empty() {
		let mut new: Vec<IVec3> = Vec::new();

		for edge in &edges {
			for a in [IVec3::AXES, IVec3::AXES.map(|v| -v)].concat() {
				let moved = *edge + a;

				if !air.contains(&moved) && !cubes.contains(&moved)
					&& moved.x >= min.x && moved.y >= min.y && moved.z >= min.z
					&& moved.x <= max.x && moved.y <= max.y && moved.z <= max.z {
						air.insert(moved);
						new.push(moved);
					}
			}
		}

		edges = new;
	}

	air
}

fn surface(cubes: &HashSet<IVec3>, air: &HashSet<IVec3>) -> i32 {
	cubes.iter().fold(0, |sum, cube| {
		let adj = [IVec3::AXES, IVec3::AXES.map(|v| -v)].concat();
		
		sum + adj.iter().fold(0, |total, face| {
			let next = *cube + *face;
		
			total + air.contains(&next) as i32
		})
	})
}
