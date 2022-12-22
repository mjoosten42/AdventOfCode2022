use std::collections::HashMap;

use glam::{IVec2, Vec2};

#[derive(Clone, Debug)]
struct Face {
	start: IVec2,
	edges: HashMap<IVec2, IVec2>,
}

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

	let size = f32::sqrt((tiles.len() / 6) as f32) as usize;
	
	let mut faces: Vec<Face> = Vec::new();
	for y in (0..tiles.len()).step_by(size) {
		for x in (0..tiles.len()).step_by(size) {
			let tile = IVec2::new(x as i32, y as i32);
			if tiles.contains_key(&tile) {
				faces.push(Face { start: tile, edges: HashMap::new() });
			}
		}
	}

	let unit_vecs: Vec<IVec2> = [IVec2::AXES, IVec2::AXES.map(|v| -v)].concat().iter().map(|v| *v * size as i32).collect();

	for face in &mut faces {
		let adj: Vec<IVec2> = unit_vecs.clone().into_iter().map(|v| v + face.start).collect();

		for tile in adj {
			if tiles.contains_key(&tile) {
				face.edges.insert((tile - face.start).signum(), tile);
			}
		}
	}

	let mut i = 0;

	loop {
		i += 1;
		for a in &faces {
			println!("{a:?}");
		}
		println!("");
		
		for face in faces.clone() {
			if face.edges.len() == 2 {
				let mut it = face.edges.iter();
				let first = *it.next().unwrap().0;
				let second = *it.next().unwrap().0;

				if first.dot(second) == 0 {
					let first_edge = face.edges.get(&first).unwrap();
					let second_edge = face.edges.get(&second).unwrap();

					faces.iter_mut().find(|f| f.start == *first_edge).unwrap().edges.insert(second, *second_edge);
					faces.iter_mut().find(|f| f.start == *second_edge).unwrap().edges.insert(first, *first_edge);
				}
			}
		}
		
		if i == 4 {
			break ;
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
