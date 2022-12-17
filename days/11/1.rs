use std::collections::BTreeSet;

#[derive(Debug)]
struct Monkey {
	items: Vec<u32>,
	op: [String; 2],
	div: u32,
	throw: (usize, usize),
	inspected: usize,
}

impl Monkey {
	fn new() -> Self {
		Monkey { items: Vec::new(), op: Default::default(), div: 0, throw: (0, 0), inspected: 0 }
	}
}

fn main() {
	let contents: String = std::fs::read_to_string(&std::env::args().collect::<Vec<String>>()[1]).unwrap();
	let mut monkeys: Vec<Monkey> = Vec::new();

	for line in contents.lines() {
		if line.is_empty() { continue; }
		let words: Vec<&str> = line.split_whitespace().collect();
		match words[0] {
			"Monkey" => monkeys.push(Monkey::new()),
			"Starting" => {
				for item in line.split_whitespace().skip(2) {
					monkeys.last_mut().unwrap().items.push(item.split(",").next().unwrap().parse().unwrap());
				}
			}
			"Operation:" =>	monkeys.last_mut().unwrap().op = [words[4].to_string(), words[5].to_string()],
			"Test:" => monkeys.last_mut().unwrap().div = words[3].parse().unwrap(),
			"If" => {
				match words[1] {
					"true:" => monkeys.last_mut().unwrap().throw.0 = words[5].parse().unwrap(),
					"false:" => monkeys.last_mut().unwrap().throw.1 = words[5].parse().unwrap(),
					_ => (),
				}
			}
			_ => (),
		}
	}

	let mut tmp: Vec<Vec<u32>> = vec![Vec::new(); monkeys.len()];
	
	for _ in 0..20 {
		for i in 0..monkeys.len() {
			monkeys[i].items.append(&mut tmp[i]);
			tmp[i].clear();
			monkeys[i].inspected += monkeys[i].items.len();
			for item in &monkeys[i].items {
				let expr = [item.to_string(), monkeys[i].op[0].clone(),
					if monkeys[i].op[1] == "old" { item.to_string() } else { monkeys[i].op[1].clone() }];
				let new = eval::eval(&expr.join(" ")).unwrap().as_u64().unwrap() as u32 / 3;
				if new % monkeys[i].div == 0 {
					tmp[monkeys[i].throw.0].push(new);
				} else {
					tmp[monkeys[i].throw.1].push(new);
				}
			}
			monkeys[i].items.clear();
		}
	}

	let result: BTreeSet<usize> = monkeys.iter().map(|monkey|monkey.inspected).collect();
	println!("{}", result.iter().rev().take(2).fold(1, |sum, ins| sum * ins));
}
