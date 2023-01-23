use std::{collections::HashMap, env, fs};

// Static array used for checking if a token is a keyword
static KEYWORDS: &'static [&'static str] = &["iadd", "out"];

fn main() {
    let args: Vec<String> = env::args().collect(); // Collect args passed on run
    if args.len() != 2 {
        panic!("Invalid arguments")
    } // Panic on invalid no. args
    let filepath: &String = &args[1]; // Get filepath

    let mut memory: HashMap<String, i32> = HashMap::new();
    let lines: Vec<String> = read_lines(&filepath);
    let tokenised_lines: Vec<Vec<String>> = tokenise_lines(&lines);

    // dbg!(
    //     &args,
    //     &filepath,
    //     &lines,
    //     &tokenised_lines,
    //     &memory,
    // );

    println!("\n--- Output ---");
    run_lines(&tokenised_lines, &mut memory);
    println!("--------------\n");

    // dbg!(&memory);
}

fn run_line(line: &Vec<String>, mem: &mut HashMap<String, i32>) -> () {
    /*
        Runs a line
    */
    let first: &str = &line[0][..];

    if !KEYWORDS.contains(&first) {
        panic!("Invalid keyword")
    }

    if line[0] == "iadd" {
        let a = line[1]
            .parse::<i32>()
            .unwrap_or_else(|_| get_val(&line[1], mem));
        let b = line[2]
            .parse::<i32>()
            .unwrap_or_else(|_| get_val(&line[2], mem));
        let val = iadd(&a, &b);
        istore(&line[3], &val, Box::new(mem).as_mut());
    } else if line[0] == "out" {
        out(&line[1].to_owned(), mem);
    }
}

fn get_val(var: &String, mem: &mut HashMap<String, i32>) -> i32 {
    match mem.get(var) {
        Some(i) => return *i,
        None => panic!("No such variable name {}", var),
    };
}

fn run_lines(lines: &Vec<Vec<String>>, mem: &mut HashMap<String, i32>) -> () {
    /*
        Runs the lines
    */
    for line in lines {
        run_line(&line, mem);
    }
}

fn out(var: &String, mem: &mut HashMap<String, i32>) -> () {
    /*
        Prints a character referenced to by a variable name var in memory mem
    */
    match mem.get(var) {
        Some(x) => println!("{:?}", x),
        None => panic!("No variable named {}", var),
    };
}

fn istore(var: &String, val: &i32, mem: &mut HashMap<String, i32>) -> () {
    /*
        Inserts something into mem with key var and value val
    */
    mem.insert(var.to_string(), *val);
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
        .map(|x| x.to_owned().replace("\r\n", "").trim().to_owned())
        .filter(|x| x != "")
        .collect::<Vec<String>>()
}

fn tokenise_lines(lines: &Vec<String>) -> Vec<Vec<String>> {
    /*
        Takes a Vec<String> of lines and splits each line into
        a Vec<String> by single spaces. Returns a Vec<Vec<String>>.
    */
    lines
        .iter()
        .map(|line| {
            line.split(" ")
                .map(|x| x.to_owned())
                .collect::<Vec<String>>()
        })
        .collect::<Vec<Vec<String>>>()
}

fn iadd(a: &i32, b: &i32) -> i32 {
    /*
    Adding function for two i64.
    */
    a + b
}