use std::env;
use std::fs;
use std::path::Path;

fn read_instructions(filename: &str) -> String{
    let fpath = Path::new(filename);
    let abspath = env::current_dir()
        .unwrap()
        .into_boxed_path()
        .join(fpath);
    let content = fs::read_to_string(abspath)
        .expect("Error occurred while reading the file!");
    return content;
}

fn get_floor_count(content:&String) -> (i32, usize) {
    // PART I
    let (mut cnt, mut pos) = (0, 0);
    for (i, c) in content.chars().enumerate(){
        match c{
            '(' => cnt+=1,
            ')' => cnt-=1,
            _ => cnt+=0
        }
        // PART II
        if cnt == -1 && pos == 0 {
            pos = i+1;
        }
    }
    (cnt, pos)
}

pub fn run(){
    let content = read_instructions("inputs/day-01.txt");
    let res = get_floor_count(&content);
    
    println!("\n-- AoC 2015: Day 01 - Not Quite Lisp --");
    println!("\nDestination Floor: {} \nFirst Basement Occurrence: {}", res.0, res.1 );
    println!("\n-- DONE --\n");
}