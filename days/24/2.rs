use std::collections::{HashMap, HashSet};

use glam::IVec2;

fn main() {
	let args: Vec<String> = std::env::args().collect();
	let contents = std::fs::read_to_string(&args[1]).unwrap();

	let mut tiles: HashMap<IVec2, Vec<IVec2>> = HashMap::new();

	for (y, line) in contents.lines().enumerate() {
		for (x, c) in line.chars().enumerate() {
			if c != '#' {
				let pos = IVec2::new(x as i32, y as i32);

				let blizzard = match c {
					'>' => vec![IVec2::new( 1, 0)],
					'<' => vec![IVec2::new(-1, 0)],
					'^' => vec![IVec2::new(0, -1)],
					'v' => vec![IVec2::new(0,  1)],
					_ => Vec::new(),
				};
			
				tiles.insert(pos, blizzard);
			}
		}
	}

	let start = *tiles.iter().min_by(|(lhs, _), (rhs, _)| lhs.y.cmp(&rhs.y)).unwrap().0;
	let end = *tiles.iter().max_by(|(lhs, _), (rhs, _)| lhs.y.cmp(&rhs.y)).unwrap().0;

	let mut minutes = 0;

	minutes += go(&mut tiles, start, end);
	minutes += go(&mut tiles, end, start);
	minutes += go(&mut tiles, start, end);

	println!("{minutes}");
	
}

fn go(tiles: &mut HashMap<IVec2, Vec<IVec2>>, start: IVec2, end: IVec2) -> i32 {
	let mut edges = vec![start];
	let mut minutes = 0;

	loop {
		minutes += 1;

		*tiles = update_blizzards(tiles);

		let mut new: HashSet<IVec2> = HashSet::new();

		for edge in edges {
			new.extend(movable(&tiles, edge).into_iter());
		}

		edges = new.into_iter().collect();
		
		if edges.contains(&end) {
			break ;
		}
	}

	minutes
}

fn update_blizzards(tiles: &HashMap<IVec2, Vec<IVec2>>) -> HashMap<IVec2, Vec<IVec2>> {
	let mut updated: HashMap<IVec2, Vec<IVec2>> = HashMap::new();

	for (pos, blizzards) in tiles {
		updated.entry(*pos).or_insert(Vec::new());

		for blizzard in blizzards {
			let mut new = *pos + *blizzard;

			if !tiles.contains_key(&new) {
				new -= *blizzard;
				while tiles.contains_key(&new) {
					new -= *blizzard;
				}

				new += *blizzard;
			}

			updated.entry(new).or_insert(Vec::new()).push(*blizzard);
		}
	}

	updated
}

fn movable(tiles: &HashMap<IVec2, Vec<IVec2>>, pos: IVec2) -> Vec<IVec2> {
	[
		pos + IVec2::ZERO,
		pos + IVec2::new( 1, 0),
		pos + IVec2::new(0,  1),
		pos + IVec2::new(-1, 0),
		pos + IVec2::new(0, -1),
	].into_iter().filter(|m| tiles.contains_key(m) && tiles.get(m).unwrap().is_empty()).collect()
}
