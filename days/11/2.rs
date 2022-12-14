struct Monkey {
	items: Vec<u32>,
	op: Box<dyn FnMut(&mut u32)>,
	throw: Box<dyn Fn(u32) -> usize>,
	inspected: u32,
	div: u32,
}

impl Monkey {
	fn new() -> Self {
		Monkey { items: Vec::new(), op: Box::new(|_| ()), throw: Box::new(|_| 0), inspected: 0, div: 0 }
	}
}

impl std::fmt::Display for Monkey {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		write!(f, "{:?}", self.items)
	}
}

fn main() {
	let contents = std::fs::read_to_string(&std::env::args().collect::<Vec<String>>()[1]).unwrap();
	let mut monkeys: Vec<Monkey> = Vec::new();

	let mut test = (0, 0, 0);
	for line in contents.lines() {
		if line.is_empty() { continue; }
		let words: Vec<&str> = line.split_whitespace().collect();
		match words[0] {
			"Monkey" => monkeys.push(Monkey::new()),
			"Starting" => {
				words.iter().skip(2).for_each(|item| {
					monkeys.last_mut().unwrap().items.push(item.trim_end_matches(",").parse::<u32>().unwrap())
				});
			}
			"Operation:" => {
				let mut cur = monkeys.last_mut().unwrap();
				if words[5] == "old" {
					cur.op = Box::new(|item: &mut u32| *item *= *item);
				} else {
					let n = words[5].parse::<u32>().unwrap();
					if words[4] == "+" {
						cur.op = Box::new(move |item| *item += n);
					} else {
						cur.op = Box::new(move |item| *item *= n);
					}
				}
			}
			"Test:" => monkeys.last_mut().unwrap().div = words[3].parse::<u32>().unwrap(),
			"If" => { 
				match words[1] {
					"true:" => test.1 = words[5].parse::<usize>().unwrap(),
					"false:" => {
						test.0 = monkeys.last().unwrap().div;
						test.2 = words[5].parse::<usize>().unwrap();
						monkeys.last_mut().unwrap().throw = Box::new(move |item| {
							if item % test.0 == 0 { test.1 } else { test.2 }
						});
					}
					_ => (),
				}
			}
			_ => (),
		}
	}

	let mut tmp: Vec<Vec<u32>> = (0..monkeys.len()).map(|_| Vec::new()).collect();

	const ROUNDS: u32 = 20;
	for _ in 0..ROUNDS {
		for entry in monkeys.iter_mut().enumerate() {
			let monkey = entry.1;
			monkey.items.append(&mut tmp[entry.0]);
			println!("{monkey}");
			monkey.inspected += monkey.items.len() as u32;
			for item in &mut monkey.items {
				*item %= monkey.div;
				(monkey.op)(item);
				let index = (monkey.throw)(*item);
				tmp[index].push(*item);
			}
			monkey.items.clear();
		}
	}
	println!("");

	for monkey in &mut monkeys.iter_mut().enumerate() {
		monkey.1.items.append(&mut tmp[monkey.0]);
		println!("{:?}", monkey.1.inspected);
	}
}
