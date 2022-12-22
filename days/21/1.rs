use std::collections::BTreeMap;

use eval::eval;

fn main() {
	let args: Vec<String> = std::env::args().collect();
	let contents = std::fs::read_to_string(&args[1]).unwrap();

	let mut monkeys: BTreeMap<String, String> = BTreeMap::new();

	for line in contents.lines() {
		let words: Vec<&str> = line.split(":").collect();

		monkeys.insert(words[0].to_string(), words[1].trim().to_string());
	}

	println!("{}", execute(&monkeys, "root"));

}

fn execute(monkeys: &BTreeMap<String, String>, monkey: &str) -> i64 {
	let value = monkeys.get(monkey).unwrap();

	let number = value.parse::<i64>();
	if number.is_ok() {
		return number.unwrap();
	}
	
	let words: Vec<&str> = value.split_whitespace().collect();

	let mut expr: Vec<String> = vec![
		execute(monkeys, words[0]).to_string(),
		words[1].to_string(),
		execute(monkeys, words[2]).to_string(),
	];

	if expr[2].starts_with("-") {
		expr[2].remove(0);
		expr[2].push(')');
		expr.insert(2, "(0 -".to_string());
	}

	if expr[0].starts_with("-") {
		expr[0].remove(0);
		expr[0].push(')');
		expr.insert(0, "(0 -".to_string());
	}

	let result = eval(&expr.join(" ")).unwrap();

	if result.is_i64() {
		result.as_i64().unwrap()
	} else {
		result.as_f64().unwrap() as i64
	}
}
