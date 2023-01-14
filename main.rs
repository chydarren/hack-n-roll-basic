use std::fs::File;
use std::io::{BufRead, BufReader};

fn read_file_to_string_array() -> Vec<String> {
	let file_path = "question_pool/question.txt";
    let file = File::open(file_path).expect("Failed to open file");
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().map(|line| line.expect("Failed to read line")).collect();
    lines
}

fn main() {
    println!("Hello, World!");
	let question_pools = read_file_to_string_array();
	for question in question_pools {
		println!("{}", question);
	}
}