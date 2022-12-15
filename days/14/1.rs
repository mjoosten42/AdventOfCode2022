use std::ops::{AddAssign, Sub};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Point {
	x: i32,
	y: i32,
}

impl Point {
	fn from(s: &str) -> Self {
		let args = s.split(",").collect::<Vec<&str>>();
		Point { x: args[0].parse::<i32>().unwrap(), y: args[1].parse::<i32>().unwrap() }
	}

	fn normal(self) -> Point {
		Point { x: self.x.clamp(-1, 1), y: self.y.clamp(-1, 1) }
	}
}

impl Sub for Point {
	type Output = Point;

	fn sub(self, other: Point) -> Point {
		Point { x: self.x - other.x, y: self.y - other.y }
	}
}

impl AddAssign for Point {
	fn add_assign(&mut self, rhs: Point) {
		self.x += rhs.x;
		self.y += rhs.y;
	}
}


fn main() {
	let contents = std::fs::read_to_string(&std::env::args().collect::<Vec<String>>()[1]).unwrap();
	let mut rocks: Vec<Vec<Point>> = Vec::new();

	for line in contents.lines() {
		rocks.push(line.split(" -> ").filter(|s| !s.is_empty()).map(|point| Point::from(point)).collect());
	}

	let max_x = rocks.iter().map(|line| line.iter().map(|point| point.x).max().unwrap()).max().unwrap();
	let min_x = rocks.iter().map(|line| line.iter().map(|point| point.x).min().unwrap()).min().unwrap();
	let max_y = rocks.iter().map(|line| line.iter().map(|point| point.y).max().unwrap()).max().unwrap();

	rocks.iter_mut().for_each(|line| line.iter_mut().for_each(|point| point.x -= min_x - 1));

	let width = (max_x - min_x + 3) as usize;
	let height = (max_y + 1) as usize;
	let mut grid: Vec<Vec<char>> = vec![vec!['.'; width]; height];

	for line in &mut rocks {
		let mut begin = line.pop().unwrap();
		while !line.is_empty() {
			let end = line.pop().unwrap();
			let diff = (end - begin).normal();
			while begin != end {
				grid[begin.y as usize][begin.x as usize] = '#';
				begin += diff;
			}
			grid[begin.y as usize][begin.x as usize] = '#';
			begin = end;			
		}
	}

	let start = 500 - (min_x - 1);
	while grid[0][start as usize] == '.' {

		let mut sand = Point { x: start, y: 0 };
		while sand.y + 1 < height as i32 {
			let next = [
				Point { x: sand.x + 0, y: sand.y + 1},
				Point { x: sand.x - 1, y: sand.y + 1},
				Point { x: sand.x + 1, y: sand.y + 1}
			];
			match next.iter().find(|point| grid[point.y as usize][point.x as usize] == '.').and_then(|point| Some(point.clone())) {
				Some(point) => sand = point.clone(),
				None => break,
			}
		}
		if sand.y + 1 == height as i32 { break ; }
		grid[sand.y as usize][sand.x as usize] = 'o';
	}

	println!("{}", grid.iter().fold(0, |sum, line| {
		sum + line.iter().fold(0, |sum, point| {
			sum + (*point == 'o') as i32
		})
	}));

}
