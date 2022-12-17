use std::{collections::HashSet, cmp::Ordering};

use glam::IVec2;

const AMOUNT_OF_ROCKS: usize = 1000000000000;
const HEIGHT_MARGIN: i32 = 4;
const WIDTH_MARGIN: i32 = 2;
const WIDTH: i32 = 7;

fn main() {
	let args: Vec<String> = std::env::args().collect();
	let contents = std::fs::read_to_string(&args[1]).unwrap();
	let rock_info = std::fs::read_to_string(&args[2]).unwrap();

	let mut shapes: Vec<Vec<IVec2>> = Vec::new();
	let mut rocks: HashSet<IVec2> = (0..WIDTH).map(|x| IVec2::new(x, 0)).collect();

	for shape in rock_info.split("\n\n") {
		shapes.push(Vec::new());
		for line in shape.lines().rev().enumerate() {
			for c in line.1.bytes().enumerate() {
				if c.1 == b'#' {
					let new = IVec2::new(c.0 as i32, line.0 as i32);
					let cur = shapes.last_mut().unwrap();

					cur.push(new);
				}
			}
		}
	}

	let mut shape_it = shapes.iter().cycle();
	let mut wind_it = contents.bytes().cycle();

	let mut last = 1;
	for i in 0..AMOUNT_OF_ROCKS {

		if i == last * 2 {
			println!("{i}");
			last *= 2;
		}

		let current_height: i32 = max_height(&rocks);
		let mut rock = shape_it.next().unwrap().clone();

		for part in &mut rock {
			part.y += current_height + HEIGHT_MARGIN;
			part.x += WIDTH_MARGIN;
		}

		loop {
			let c = wind_it.next().unwrap();
			let x_move = if c == b'>' { 1 } else { -1 };
			let moved: Vec<IVec2> = rock.iter().map(|part| IVec2::new(part.x + x_move, part.y)).collect();

			if !moved.iter().any(|part| part.x < 0 || part.x >= WIDTH || rocks.contains(part)) {
				rock = moved;
			}

			let moved: Vec<IVec2> = rock.iter().map(|part| IVec2::new(part.x, part.y - 1)).collect();

			if moved.iter().any(|part| rocks.contains(part)) {
				rocks.extend(rock.into_iter());
			
				// println!("Before:");
				// print_grid(&rocks);
			
				let tmp = rocks.clone();
		
				rocks.retain(|p|{
					let adj = adjacent(p);
					!adj.iter().all(|n| n.x < 0 || n.x >= WIDTH || tmp.contains(n))
				});

				// println!("After:");
				// print_grid(&rocks);
			
				break ;
			}

			rock = moved;
		}
	}

	println!("{}", max_height(&rocks));

}

fn max_height(rocks: &HashSet<IVec2>) -> i32 {
	rocks.iter().max_by(|lhs, rhs| lhs.y.cmp(&rhs.y)).unwrap_or(&IVec2::new(0, 0)).y
}

fn adjacent(point: &IVec2) -> Vec<IVec2> {
	let mut v = vec![IVec2::new(point.x, point.y + 1)];

	if point.x > 0 { v.push(IVec2::new(point.x - 1, point.y)); }
	// if point.y > 0 { v.push(IVec2::new(point.x, point.y - 1)); }
	if point.x + 1 < WIDTH { v.push(IVec2::new(point.x + 1, point.y)); }

	v
}

fn print_grid(rocks: &HashSet<IVec2>) {
	for y in (0..5).rev() {
		for x in 0..WIDTH {
			let c = if rocks.contains(&IVec2::new(x, y)) { '#' } else { '.' };

			print!("{c}");
		}
		println!("");
	}
	println!("");
}