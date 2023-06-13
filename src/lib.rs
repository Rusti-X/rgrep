use std::env;
use std::fmt::Display;
use std::io;
use std::fs;
use std::process;
use regex::Regex;



pub struct Line {
    pub string: String,
    pub number: usize,
}

impl Line {
    pub fn new(string: &str, number: usize) -> Self {
        Self {
            string: String::from(string),
            number,
        }
    }
}


pub struct Config {
    pub query: String,
    pub file_path: String,
    pub any_case: bool,
}


pub fn parse_args() -> Result<Config, io::ErrorKind> { // (query, file_path)
    let args: Vec<String> = env::args().collect();
    let length = args.len();
    if length > 2 {
        match args[1].as_str() {
            "--help" | "-h" => {
                print_help(); process::exit(-1);
            }
            _ => {}
        }
    }
    if length < 3 {
        return Err(io::ErrorKind::InvalidInput);
    }
    let mut any_case = false;
    if length > 3 {
        match args[3].as_str() {
            "-C" | "--any-case" | "-c" | "--ignore-case" => any_case = true,
            _ => {}
        }
    }
    Ok( Config {
        query: args[1].clone(),
        file_path: args[2].clone(),
        any_case,
    } )
}


pub fn get_file_contents(file_path: &str) -> String {
    let result = fs::read_to_string(file_path);
    result.unwrap_or_else(|err| {
        error(err)
    })
}


pub fn error<T>(object: T) -> !
where
    T: Display
{
    eprintln!("Error!: {}", object.to_string());
    process::exit(-1)
}


pub fn search_ignored_case(
    query: &str,
    contents: &str,
) -> Vec<Line> {
    let query = query.to_lowercase();
    let mut results: Vec<Line> = Vec::new();

    let mut line_number: usize = 1;

    let re = Regex::new(&query.to_lowercase()).expect("Invalid regular expression");

    for line in contents.lines() {
        if re.is_match(&line.to_lowercase()) {
            results.push(
                Line::new(line, line_number)
            );
        }
        line_number += 1;
    }

    results
}

pub fn search(
    query: &str,
    contents: &str,
) -> Vec<Line> {
    let mut results: Vec<Line> = Vec::new();

    let mut line_number: usize = 1;

    let re = Regex::new(query).expect("Invalid regular expression");

    for line in contents.lines() {
        if re.is_match(line) {
            results.push(
                Line::new(line, line_number)
            );
        }
        line_number += 1;
    }

    results
}


pub fn strmul(string: &str, count: usize) -> String {
    if count == 0 {
        return String::from("");
    }

    let mut result = String::new();

    let mut count = count;

    while count != 0 {
        result.push_str(string);
        count -= 1;
    };

    result
}


// Get Vector of Line's as results
pub fn get_results(config: &Config) -> Vec<Line> {
    let all_length = 30 + config.file_path.len() + config.query.len() + 5;
    println!(
        "File '{}', query: '{}', any case: {}",
        config.file_path, config.query, config.any_case
    );
    println!("{}", strmul("-", all_length));

    let contents = get_file_contents(&config.file_path);

    if config.any_case {
        return search_ignored_case(&config.query, &contents);
    }
    search(&config.query, &contents)
}




fn print_help() {
    println!("rgrep - rust grep");
    println!("Minimalistic fork of GREP (search globally for lines matching the regular expression, and print them) utility, by written on Rust.");
    println!("\nUsage: ");
    println!("  $ rgrep %find% %file% %optional: [-C/-c/--any-case/--ignore-case]%");
}





