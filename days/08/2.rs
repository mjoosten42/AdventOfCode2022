fn main() {
	let contents = std::fs::read_to_string(&std::env::args().collect::<Vec<String>>()[1]).unwrap();
	let mut grid: Vec<Vec<(u8, usize)>> = Vec::new();

	for line in contents.lines() {
		grid.push(Vec::new());
		for c in line.as_bytes() {
			grid.last_mut().unwrap().push((*c - b'0', 0));
		}
	}

	for i in 0..grid.len() {
		for j in 0..grid.len() {
			let mut views = [0; 4];

			while i + (views[0] + 1) < grid.len() { views[0] += 1; if grid[i + views[0]][j] >= grid[i][j] { break } }
			while i >= (views[1] + 1) { views[1] += 1; if grid[i - views[1]][j] >= grid[i][j] { break } }
			while j + (views[2] + 1) < grid.len() { views[2] += 1; if grid[i][j + views[2]] >= grid[i][j] { break } }
			while j >= (views[3] + 1) { views[3] += 1; if grid[i][j - views[3]] >= grid[i][j] { break } }
			
			grid[i][j].1 = views.iter().fold(1, |sum, &view| sum * view);
		}
	}

	println!("{}", grid.iter().fold(0, |x, line| {
		let max = line.iter().fold(0, |y, tree| {
			if tree.1 > y { tree.1 } else { y }
		});
		if max > x { max } else { x }
	}));
}
