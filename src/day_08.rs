use std::env;
use std::fs;
use std::path::Path;
use regex::Regex;

fn read_instructions(filename:&str) ->String{
    let fpath = Path::new(filename);
    let abspath = env::current_dir()
        .unwrap()
        .into_boxed_path()
        .join(fpath);
    let content = fs::read_to_string(abspath)
        .expect("Failed to read file!");
    return content
}

fn count_all(content:&str) -> usize{
    let rwords = Regex::new(r"\w").unwrap();
    let rhex = Regex::new(r"(\\x[a-z0-9]{2})").unwrap();
    let rquots = Regex::new("\"").unwrap();
    let rslash = Regex::new(r"\\\\").unwrap();

    let (mut code_len, mut actual_len) = (0, 0);
    for line in content.lines(){
        let trimmed = &line[1..line.len()-1];        

        code_len+= line.len();

        let hexas = rhex.find_iter(trimmed).count();
        let alphas = rwords.find_iter(trimmed).count() - (hexas*2);
        let quots = rquots.find_iter(trimmed).count();
        let slashes = rslash.find_iter(trimmed).count();

        actual_len += alphas+quots+slashes;

    }
    return code_len-actual_len;
}

fn count_all_after_encoding(content:&str) -> usize{
    let rslashes =Regex::new(r"\\").unwrap();
    let rdquotes =Regex::new("\"").unwrap();
    let mut total = 0;
    for line in content.lines(){
        let slashes = rslashes.find_iter(line).count()+2;
        let dquotes = rdquotes.find_iter(line).count();

        total+= slashes + dquotes
    }
    return total;
}

pub fn run(){
    let content = read_instructions("inputs/day-08.txt");
    let count_before = count_all(&content);
    let count_after =count_all_after_encoding(&content);

    // Note: First answer is off by 6
    println!("\n-- AoC 2015: -- Day 8: Matchsticks --");
    println!("\n🔷  Count Before Encoding: {} \n🔶  Count After Encoding: {}", count_before,count_after);
    println!("\n-- DONE --\n");
}

