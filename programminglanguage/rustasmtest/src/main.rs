use std::{env, fs, collections::HashMap};

// Static array used for checking if a token is a keyword
static KEYWORDS: &'static [&'static str] = &[
    "iadd",
    "out",
];

fn main() {
    let args: Vec<String> = env::args().collect();  // Collect args passed on run
    if args.len() != 2 {panic!("Invalid arguments")}// Panic on invalid no. args
    let filepath: &String = &args[1];               // Get filepath
    let lines: Vec<String> = read_lines(&filepath);
    let tokens: Vec<Vec<String>> = tokenise_lines(&lines);
    run_line(&tokens[0]);
    //// Debug
    //// dbg!(
    ////     &args, 
    ////     &filepath, 
    ////     &lines,
    ////     &tokens,
    //// );
}

fn run_line(line: &Vec<String>) -> Result<(), &str> {
    let first: &str = &line[0][..];
    if !KEYWORDS.contains(&first) { return Err("Invalid keyword") }
    if line[0] == "iadd" {
        let a = line[1].parse::<i32>().unwrap();
        let b = line[2].parse::<i32>().unwrap();
        let val = iadd(&a, &b);
    }  

    Ok(())
}

fn read_lines(filepath: &String) -> Vec<String> {
    /*
        Reads all of the lines in the file specified by filepath.
        Returns a Vec<String> of the lines split by ";".
        Removes \r\n and leading/trailing whitespace.
    */
    fs::read_to_string(&filepath)
        .expect("Failed to read file.")
        .split(";")
        .map(|x| x
            .to_owned()
            .replace("\r\n", "")
            .trim()
            .to_owned())
        .collect::<Vec<String>>()
}

fn tokenise_lines(lines: &Vec<String>) -> Vec<Vec<String>> {
    /*
        Takes a Vec<String> of lines and splits each line into
        a Vec<String> by single spaces. Returns a Vec<Vec<String>>.
    */
    lines
        .iter()
        .map(|line| line
            .split(" ")
            .map(|x| x.to_owned())
            .collect::<Vec<String>>())
        .collect::<Vec<Vec<String>>>()
}

fn iadd(a: &i32, b: &i32) -> i32 {
    /*
    Adding function for two i64.
    */
    a + b
}