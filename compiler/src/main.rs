mod preprocessor;

use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1{
        println!("Program arguments:");
        println!("<FILENAME>");
        ()
    }
    
    let mut file_contents = fs::read_to_string(&args[1])
        .expect("LogRocket: Should have been able to read the file");

    preprocessor::delete_multi_line_comments(&mut file_contents);
    preprocessor::delete_single_line_comments(&mut file_contents);

    println!("{file_contents}");
}