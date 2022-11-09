pub fn lex(file_text : &String) -> Vec<String> {
	file_text
		.split_whitespace()
		.map(|s| s.chars().fold(Vec::new(), |mut acc, c| 
			if ['(', ')'].contains(&c) {
				//return acc.concat([&c].to_vec());
				let paren_string : String = [c].iter().collect();
				acc.push(paren_string);
				acc
			}
			else {
				if acc.len() == 0 {
					acc.push(format!("{}", c));
					acc
				}
				else {
					let last_char = acc.last().unwrap().chars().last().unwrap();
					if ['(', ')'].contains(&last_char) {
						acc.push("".to_string());
					}
					let last_index = acc.len() - 1;
					acc[last_index] = format!("{}{}", acc.last().unwrap(), c);
					acc
				}
			}
		))
		.fold(Vec::new(), |mut acc: Vec<String>, v: Vec<String>| {
			for s in v {
				acc.push(s.clone());
			}
			acc
		})
}