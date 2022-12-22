use std::collections::HashMap;

use glam::IVec2;

fn main() {
	let args: Vec<String> = std::env::args().collect();
	let contents = std::fs::read_to_string(&args[1]).unwrap();

	let mut tiles: HashMap<IVec2, bool> = HashMap::new();

	for (y, row) in contents.split("\n\n").next().unwrap().lines().enumerate() {
		for (x, tile) in row.bytes().enumerate() {
			if tile == b'.' || tile == b'#' {
				tiles.insert(IVec2::new(x as i32, y as i32), tile == b'.');
			}
		}
	}

	let mut pos = *tiles.iter().filter(|&tile| {
		tile.0.y == 0 && *tile.1
	}).min_by(|lhs, rhs| lhs.0.x.cmp(&rhs.0.x)).unwrap().0;

	let path = contents.split("\n\n").last().unwrap();
	
	let mut instr: Vec<&str> = Vec::new();
	let mut last = 0;
	for (index, matched) in path.match_indices(|c| c == 'L' || c == 'R') {
		if last != index {
			instr.push(&path[last..index]);
		}
		instr.push(matched);
		last = index + matched.len();
	}
	if last < path.len() {
		instr.push(&path[last..])
	}

	let mut dir = IVec2::new(1, 0);
	for s in instr {
		match s {
			"L" => dir = dir.rotate(IVec2::new(0, -1)),
			"R" => dir = dir.rotate(IVec2::new(0, 1)),
			_ => pos = step(&tiles, pos, dir, s.trim().parse::<usize>().unwrap()),
		}
	}

	let facing = match dir {
		IVec2 { x:1, y:0 } => 0,
		IVec2 { x:0, y:1 } => 1,
		IVec2 { x:-1, y:0 } => 2,
		IVec2 { x:0, y:-1 } => 3,
		_ => 0,
	};

	pos += IVec2::ONE;

	println!("{}", 1000 * pos.y + 4 * pos.x + facing);

}

fn step(tiles: &HashMap<IVec2, bool>, mut pos: IVec2, dir: IVec2, amount: usize) -> IVec2 {
	for _ in 0..amount {
		let next = pos + dir;
		let opt = tiles.get(&next);

		if opt.is_some() {
			if *opt.unwrap() {
				pos = next;
			} else {
				break ;
			}
		} else {
			let mut begin = match dir {
				IVec2 { x:1, y:0 } => IVec2::new(0, pos.y),
				IVec2 { x:0, y:1 } => IVec2::new(pos.x, 0),
				IVec2 { x:-1, y:0 } => IVec2::new(200, pos.y),
				_ => IVec2::new(pos.x, 200),
			};
			
			while !tiles.contains_key(&begin) {
				begin += dir;
			}

			if *tiles.get(&begin).unwrap() {
				pos = begin;
			} else {
				break ;
			}

		}
	}

	pos




}
