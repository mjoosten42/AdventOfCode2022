#[derive(Debug)]
#[derive(Clone, Copy)]
struct Tile {
	x: usize,
	y: usize,
	height: u8,
	reached: bool,
	end: bool,
}

fn main() {
	let contents = std::fs::read_to_string(&std::env::args().collect::<Vec<String>>()[1]).unwrap();

	let mut grid: Vec<Vec<Tile>> = Vec::new();
	let mut edges: Vec<Tile> = Vec::new();
	
	for line in contents.lines().enumerate() {
		grid.push(Vec::new());
		for c in line.1.bytes().enumerate() {
			let new = Tile { x: c.0, y: line.0, height: match c.1 {
				b'S' => b'a', 
				b'E' => b'z',
				_ => c.1,
			}, reached: c.1 == b'S', end: c.1 == b'E'};
			grid.last_mut().unwrap().push(new);
			if c.1 == b'S' { edges.push(new); }
		}
	}

	let mut steps = 0;
	let mut add: Vec<Tile> = Vec::new();
	'outer: loop {

		println!("{steps}");
		for line in &grid {
			for tile in line {
				print!("{}", if tile.reached { '#' } else { '.' });
			}
			println!("");
		}

		steps += 1;
		for tile in &edges {
			let mut neighbors: Vec<Tile> = Vec::new();
			if tile.x >= 1 { neighbors.push(grid[tile.y][tile.x - 1]); }
			if tile.x + 1 < grid.first().unwrap().len() { neighbors.push(grid[tile.y][tile.x + 1]); }
			if tile.y >= 1 { neighbors.push(grid[tile.y - 1][tile.x]); }
			if tile.y + 1 < grid.len() { neighbors.push(grid[tile.y + 1][tile.x]); }

			for edge in &mut neighbors {
				if !edge.reached && edge.height <= tile.height + 1 {
					if edge.end { break 'outer; }
					grid[edge.y][edge.x].reached = true;
					add.push(grid[edge.y][edge.x]);
				}
			}
		}
		
		edges = add.clone();
	}


	println!("{steps}");



}
