use std::collections::HashMap;

use regex::Regex;
use glam::IVec2;

fn main() {
	let args: Vec<String> = std::env::args().collect();
	let contents = std::fs::read_to_string(&args[1]).unwrap();
	let re = Regex::new(r"[,:]").unwrap();
	const MAX: i32 = 4000000;

	let mut sensors: Vec<IVec2> = Vec::new();
	let mut beacons: Vec<IVec2> = Vec::new();

	for line in contents.lines() {
		let numbers: Vec<i32> = line.split("=").skip(1).map(|s| re.split(s).next().unwrap().parse::<i32>().unwrap()).collect();
		sensors.push(IVec2::new(numbers[0], numbers[1]));
		beacons.push(IVec2::new(numbers[2], numbers[3]));
	}

	sensors.sort_by(|f, s| f.x.cmp(&s.x));

	let mut pairs: HashMap<IVec2, i32> = HashMap::new();
	for sensor in sensors {
		let mut closest: i32 = i32::MAX;
		for beacon in &beacons {
			let distance = (beacon.x - sensor.x).abs() + (beacon.y - sensor.y).abs();
			if distance < closest {
				closest = distance;
			}
		}
		pairs.insert(sensor, closest);
	}

	for line in 0..MAX {
		let mut covered: Vec<(i32, i32)> = Vec::new();
		for area in &pairs {
			let sensor = area.0;
			let distance = area.1;

			let sub = distance - (sensor.y - line).abs();
			if sub >= 0 {
				covered.push(((sensor.x - sub).clamp(0, MAX), (sensor.x + sub).clamp(0, MAX)));
			}
		}

		covered.sort();

		let mut retained: Vec<(i32, i32)> = vec![(0, 0)];
		for range in covered {
			let prev = retained.last_mut().unwrap();

			if range.1 > prev.1 {
				if range.0 > prev.1 +1 {
					retained.push(range);
				} else {
					prev.1 = range.1;
				}
			}
		}

		if retained.len() == 2 {
			println!("{}", (retained.first().unwrap().1 + 1) as u128 * MAX as u128 + line as u128);
			break ;
		}		
	}
}
