extern crate rand;

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::io;
use rand::seq::index::sample;

fn read_file_to_string_array(file_path: &str) -> Vec<String> {
    let file = File::open(file_path).expect("Failed to open file");
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().map(|line| line.expect("Failed to read line")).collect();
    lines
}

fn main() {
	let stdin = io::stdin();
    println!("OI where is my goodie bag?");
	
    let question_pools = read_file_to_string_array("question_pool/question.txt");
	
    let mut rng = rand::thread_rng();
    let random_questions = sample(&mut rng, question_pools.len(), 5);
	
	let mut marks = 0;
	
    for question in random_questions.into_iter() {
        println!("{}", question_pools[question]);
		let question_index = question + 1;
		let answer_path = "question_pool/".to_owned() + &question_index.to_string() + ".txt";
		let answers = read_file_to_string_array(&answer_path);
	    let mut user_input = String::new();

		stdin.read_line(&mut user_input);
		user_input = user_input.trim().to_string();

		if answers.contains(&user_input) {
			marks += 1;
			println!("aiyo smart siol");
		} else {
			println!("dumb ass what da hell u doin?");
		};
		
    }
	
	println!("Wow, you scored {} marks only ah! Lapsap", marks);
}