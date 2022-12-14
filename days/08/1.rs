fn main() {
	let contents = std::fs::read_to_string(&std::env::args().collect::<Vec<String>>()[1]).unwrap();
	let mut grid: Vec<Vec<(u8, bool)>> = Vec::new();

	for line in contents.lines() {
		grid.push(Vec::new());
		for c in line.as_bytes() {
			grid.last_mut().unwrap().push((*c - b'0', false));
		}
	}

	for i in 0..grid.len() {
		let mut highest: i8 = -1;
		for j in 0..grid.len() {
			if grid[i][j].0 as i8 > highest {
				highest = grid[i][j].0 as i8;
				grid[i][j].1 = true;
			}
		}
	}

	for i in 0..grid.len() {
		let mut highest: i8 = -1;
		for j in (0..grid.len()).rev() {
			if grid[i][j].0 as i8 > highest {
				highest = grid[i][j].0 as i8;
				grid[i][j].1 = true;
			}
		}
	}

	for j in 0..grid.len() {
		let mut highest: i8 = -1;
		for i in 0..grid.len() {
			if grid[i][j].0 as i8 > highest {
				highest = grid[i][j].0 as i8;
				grid[i][j].1 = true;
			}
		}
	}

	for j in 0..grid.len() {
		let mut highest: i8 = -1;
		for i in (0..grid.len()).rev() {
			if grid[i][j].0 as i8 > highest {
				highest = grid[i][j].0 as i8;
				grid[i][j].1 = true;
			}
		}
	}

	println!("{}", grid.iter().fold(0, |x, line| {
		x + line.iter().fold(0, |y, tree| {
			y + (tree.1 as i32)
		})
	}));

}
