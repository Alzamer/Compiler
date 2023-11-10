use regex::Regex;
use std::collections::HashMap;

pub fn delete_single_line_comments(source: &mut String) -> () {
	let re = Regex::new(r"//.*\n").unwrap();
	let matched: Vec<_> = re.find_iter(source).map(|m| (m.start(), m.end())).collect();

	for i in matched.iter().rev() {
		source.drain(i.0..i.1);
	}
}

pub fn delete_multi_line_comments(source: &mut String) -> () {
	let re = Regex::new(r"\/\*(.|\n|\r\n)*?\*\/").unwrap();	
	let matched: Vec<_> = re.find_iter(source).map(|m| (m.start(), m.end())).collect();
	
	for i in matched.iter().rev() {
		source.drain(i.0..i.1);
	}
}

pub fn handle_macros(file_content: &mut String) -> String {
	let re_define = Regex::new(r"#define [a-zA-Z]{1}[a-zA-Z0-9]* [a-zA-Z0-9]+$").unwrap();
	let re_undef = Regex::new(r"#undef [a-zA-Z]{1}[a-zA-Z0-9]*$").unwrap();
	let lines : Vec<_> = file_content.split("\r\n").map(|m| m.trim()).collect();
	let mut macros = HashMap::new();
	let mut result : Vec<_> = Vec::new();

	for line in lines {
		let mut altered_line = line.to_string();

		if let Some(matched) = re_define.find(line) {
			let matched_macro = &line[matched.start()..matched.end()].split(" ").collect::<Vec<_>>();
			macros.insert(matched_macro[1], matched_macro[2]);
			altered_line.replace_range(matched.start()..matched.end(), "");
		}

		if let Some(matched) = re_undef.find(line) {
			let matched_macro = &line[matched.start()..matched.end()].split(" ").collect::<Vec<_>>();
			macros.remove(&matched_macro[1]);
			altered_line.replace_range(matched.start()..matched.end(), "");			
		}

		for key in macros.keys() {
			altered_line = altered_line.replace(key, macros.get(key).unwrap());
		}

		result.push(altered_line);
	}

	*file_content = result.join("");
	file_content.to_string()
}