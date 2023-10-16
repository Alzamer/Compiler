use regex::Regex;

pub fn delete_single_line_comments(source: &mut String) -> () {
	let re = Regex::new(r"//.*\n").unwrap();	
	let matched: Vec<_> = re.find_iter(source).map(|m| (m.start(), m.end())).collect();

	for i in matched.iter().rev(){
		source.drain(i.0..i.1);
	}
}

pub fn delete_multi_line_comments(source: &mut String) -> () {
	let re = Regex::new(r"\/\*.*\*\/").unwrap();	
	let matched: Vec<_> = re.find_iter(source).map(|m| (m.start(), m.end())).collect();
	
	for i in matched.iter().rev(){
		source.drain(i.0..i.1);
	}
}