use std::collections::{HashSet, HashMap, VecDeque};

use glam::IVec2;

const ROUNDS: usize = 10;

fn main() {
	let args: Vec<String> = std::env::args().collect();
	let contents = std::fs::read_to_string(&args[1]).unwrap();

	let mut elves: HashSet<IVec2> = HashSet::new();
	for (y, line) in contents.lines().enumerate() {
		for (x, c) in line.bytes().enumerate() {
			if c == b'#' {
				elves.insert(IVec2::new(x as i32, y as i32));
			}
		}
	}

	let mut directions: VecDeque<Vec<IVec2>> = vec![
		vec![IVec2::new(0, -1), IVec2::new(1, -1), IVec2::new(-1, -1)],
		vec![IVec2::new(0, 1), IVec2::new(1, 1), IVec2::new(-1, 1)],
		vec![IVec2::new(-1, 0), IVec2::new(-1, -1), IVec2::new(-1, 1)],
		vec![IVec2::new(1, 0), IVec2::new(1, -1), IVec2::new(1, 1)],
	].into();

	for _ in 0..ROUNDS {
		let mut moves: HashMap<IVec2, (usize, IVec2)> = HashMap::new();
	
		for elf in elves.clone() {
			let adj = adjacent_elves(&elves, elf);

			if adj.is_empty() {
				continue;
			}

			for direction in &directions {
				if !direction.iter().any(|&p| adj.contains(&(elf + p))) {
					let to = elf + direction[0];
					let entry = moves.entry(to).or_insert((0, IVec2::ZERO));

					entry.0 += 1;
					entry.1 = elf;
					break;
				}
			}
		}

		moves.retain(|_, (amount, _)| *amount == 1);

		for (to, (_, elf)) in moves {
			elves.remove(&elf);
			elves.insert(to);
		}

		directions.rotate_left(1);

	}

	let mut total = 0;
	let min = min(&elves);
	let max = max(&elves);

	for y in (min.y)..(max.y + 1) {
		for x in (min.x)..(max.x + 1) {
			if !elves.contains(&IVec2::new(x, y)) {
				total += 1;
			}
		}
	}
	
	println!("{total}");

}

fn print(elves: &HashSet<IVec2>) {
	let min = min(elves) - IVec2::ONE;
	let max = max(elves) + IVec2::ONE;

	for y in (min.y)..(max.y + 1) {
		for x in (min.x)..(max.x + 1) {
			if elves.contains(&IVec2::new(x, y)) {
				print!(" #");
			} else {
				print!(" .");
			}
		}
		println!("");
	}
	println!("");
}

fn adjacent_elves(elves: &HashSet<IVec2>, elf: IVec2) -> Vec<IVec2> {
	adjacent(elf).into_iter().filter(|e| elves.contains(e)).collect()
}

fn adjacent(pos: IVec2) -> Vec<IVec2> {
	vec![
		IVec2::new(pos.x - 1, pos.y - 1),
		IVec2::new(pos.x + 0, pos.y - 1),
		IVec2::new(pos.x + 1, pos.y - 1),
		IVec2::new(pos.x - 1, pos.y + 0),
		IVec2::new(pos.x + 1, pos.y + 0),
		IVec2::new(pos.x - 1, pos.y + 1),
		IVec2::new(pos.x + 0, pos.y + 1),
		IVec2::new(pos.x + 1, pos.y + 1),
	]
}

fn min(elves: &HashSet<IVec2>) -> IVec2 {
	let x = elves.iter().min_by(|lhs, rhs| lhs.x.cmp(&rhs.x)).unwrap().x;
	let y = elves.iter().min_by(|lhs, rhs| lhs.y.cmp(&rhs.y)).unwrap().y;

	IVec2::new(x, y)
}

fn max(elves: &HashSet<IVec2>) -> IVec2 {
	let x = elves.iter().max_by(|lhs, rhs| lhs.x.cmp(&rhs.x)).unwrap().x;
	let y = elves.iter().max_by(|lhs, rhs| lhs.y.cmp(&rhs.y)).unwrap().y;

	IVec2::new(x, y)
}