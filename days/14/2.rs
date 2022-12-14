use regex::Regex;
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

	let re = Regex::new(r"[ \->]").unwrap();
	for line in contents.lines() {
		rocks.push(re.split(line).filter(|s| !s.is_empty()).map(|point| Point::from(point)).collect());
	}

	let max_y = rocks.iter().map(|line| line.iter().map(|point| point.y).max().unwrap()).max().unwrap();

	rocks.push(vec![Point { x: 0, y: max_y + 2}, Point { x: 990, y: max_y + 2 } ]);
	let width = 1000;
	let height = (max_y + 3) as usize;
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

	const START: usize = 500;
	while grid[0][START] == '.' {
		let mut sand = Point { x: START as i32, y: 0 };
		loop {
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
		grid[sand.y as usize][sand.x as usize] = 'o';
	}

	println!("{}", grid.iter().fold(0, |sum, line| {
		sum + line.iter().fold(0, |sum, point| {
			sum + (*point == 'o') as i32
		})
	}));

}
