use std::io::{self, BufRead};
use std::fs;
use std::env;

fn main(){
    let args: Vec<String> =  env::args().collect();

    if args.len() < 2 {
        println!("usage {} <filename>", &args[0]);
        return;
    }

    let filename = &args[1];

    match count_file(filename){
        Ok((lines, words, characters)) => {
            println!("Filename: {}", filename);
            println!("Lines: {}", lines);
            println!("Words: {}", words);
            println!("Characters: {}", characters);
        },
        Err(err) => {
            println!("Errors: {}", err);
        }
    }
}

fn count_file(filename: &str) -> io::Result<(usize, usize, usize)> {
    let file = fs::File::open(filename)?;
    let reader = io::BufReader::new(file);

    let mut line = 0;
    let mut word = 0;
    let mut character = 0;
    
    for i in reader.lines(){
        let i = i?;
        line += 1;
        word += i.split_whitespace().count();
        character += i.chars().count();
    }

    Ok((line, word, character))
}