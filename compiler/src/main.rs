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
    else{
        let file_contents = fs::read_to_string(&args[1]);
    }

    //preprocessor::delete_multi_line_comments(&mut ex);
    //preprocessor::delete_single_line_comments(&mut ex);
}