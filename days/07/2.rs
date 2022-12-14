use std::{path::PathBuf, collections::BTreeMap};

fn main() {
	let contents = std::fs::read_to_string(&std::env::args().collect::<Vec<String>>()[1]).unwrap();
	let mut paths: BTreeMap<PathBuf, u32> = BTreeMap::from([(PathBuf::from("/"), 0)]);
	let mut cur = PathBuf::from("/");

	for line in contents.lines() {
		let words: Vec<&str> = line.split_whitespace().collect();
		match words[0] {
			"$" => {
				match words[1] {	
					"cd" =>	{
						match words[2] {
							"/" => { cur = PathBuf::from("/"); }
							".." =>  { cur.pop(); }
							_ => { cur.push(words[2]); }
						}
					}
					_ => (), // ls
				}
			}
			"dir" => { paths.insert(cur.join(words[1]), 0); }
			_ => {
				cur.ancestors().for_each(|path| {
					paths.entry(path.to_path_buf()).and_modify(|size| {
						*size += words[0].parse::<u32>().unwrap();
					}); 
				});
			}
		}
	}

	let missing = 30000000 - (70000000 - paths.get(&PathBuf::from("/")).unwrap());
	println!("{}", paths.iter().filter(|path| *path.1 >= missing).min_by(|x, y| x.1.cmp(y.1)).unwrap().1);

}
