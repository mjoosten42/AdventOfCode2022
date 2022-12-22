use std::{collections::{HashMap, HashSet}};

use eval::eval;

fn main() {
	let args: Vec<String> = std::env::args().collect();
	let contents = std::fs::read_to_string(&args[1]).unwrap();

	let mut monkeys: HashMap<String, Vec<String>> = HashMap::new();

	for line in contents.lines() {
		let words: Vec<&str> = line.split(":").collect();

		monkeys.insert(words[0].to_string(), words[1].trim().split_whitespace().map(|s| s.to_string()).collect());
	}

	let mut route: HashSet<String> = HashSet::new();

	find_route(&monkeys, &mut route, "root");

	let mut expr = monkeys.get("root").unwrap().clone();

	expr[1] = "-".to_string();
	
	equals(&monkeys, &route, 0, expr);
}

// Number == expr
fn equals(monkeys: &HashMap<String, Vec<String>>, route: &HashSet<String>, lhs: i64, expr: Vec<String>) {
	if expr.len() == 1 {
		println!("{lhs}");
		return;
	}

	let solved = if route.contains(&expr[0]) { 2 } else { 0 };
	let unsolved = if solved == 0 { 2 } else { 0 };
	let rhs: i64 = execute(monkeys, &expr[solved]);
	let new_expr = monkeys.get(&expr[unsolved]).unwrap().clone();

	let mut solve = 0;
	match expr[1].as_str() {
		"+" => solve = lhs - rhs,
		"-" => {
			if solved == 0 {
				solve = lhs - rhs;
			} else {
				solve = lhs + rhs;
			}
		}
		"*" =>solve = lhs / rhs,
		"/" => {
			if solved == 0 {
				solve = rhs / lhs;
			} else {
				solve = lhs * rhs;
			}
		},
		_ => (),
	}

	equals(monkeys, route, solve, new_expr);
}

fn find_route(monkeys: &HashMap<String, Vec<String>>, route: &mut HashSet<String>, monkey: &str) -> bool {
	if monkey == "humn" {
		route.insert(monkey.to_string());
		return true;
	}

	let expr = monkeys.get(monkey).unwrap();

	if expr.len() == 3 	{
		if find_route(monkeys, route, &expr[0]) || find_route(monkeys, route, &expr[2]) {
			route.insert(monkey.to_string());
			return true
		}
	}
	false
}

fn execute(monkeys: &HashMap<String, Vec<String>>, monkey: &str) -> i64 {
	let value = monkeys.get(monkey).unwrap();

	if value.len() == 1 {
		return value[0].parse().unwrap()
	}

	let mut expr: Vec<String> = vec![
		execute(monkeys, &value[0]).to_string(),
		value[1].to_string(),
		execute(monkeys, &value[2]).to_string(),
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
