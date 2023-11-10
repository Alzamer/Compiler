mod preprocessor;
mod lexer;

use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        println!("Program arguments:");
        println!("<FILENAME>");
        ()
    }
    
    let mut file_content = fs::read_to_string(&args[1])
        .expect("Couldn't read the content of the file!");

    preprocessor::delete_multi_line_comments(&mut file_content);
    preprocessor::delete_single_line_comments(&mut file_content);
    preprocessor::handle_macros(&mut file_content);

    println!("{:?}", lexer::scan(file_content));
}