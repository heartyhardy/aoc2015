use std::env;
use std::fs;
use std::path::Path;

enum Rules{
    Basic,
    Advanced
}

fn read_instructions(filename:&str)->String{
    let fpath = Path::new(filename);
    let abspath = env::current_dir()
        .unwrap()
        .into_boxed_path()
        .join(fpath);
    let content = fs::read_to_string(abspath)
        .expect("Error occurred while reading the file!");
    return content;
}

fn nice_or_naughty(content:&String, rules:&Rules)-> u16{
    let mut nc = 0;
    let lines = content.lines();
    for l in lines{
        match rules{
            &Rules::Basic => {
                if not_contains(&l) &&
                has_repeating_chars(&l) &&
                contains_three_vs(&l)
                {
                    nc+=1;
                }
            },
            &Rules::Advanced => {
                if pair_appears_twice(&l) &&
                repeats_after_one(&l)
                {
                    nc+=1;
                }
            }
        }
    }
    return nc;
}

//Rule 3 - does not contain the strings ab, cd, pq, or xy
fn not_contains(line: &str) -> bool{
    let comps:[&str;4] = ["ab", "cd", "pq", "xy"];
    for comp in comps.iter(){
        if line.contains(comp){
            return false;
        }
    }
    return true;
}

//Rule 2 - contains at least one letter that appears twice in a row
fn has_repeating_chars(line: &str) -> bool{
    let mut lc = ' ';
    for c in line.chars(){
        if lc == ' '{
            lc=c;
            continue;
        }
        if lc == c{
            return true
        }
        lc=c
    }
    return false;
}

//Rule 1 - contains at least three vowels
fn contains_three_vs(line:&str) -> bool{
    let mut cnt = 0;
    let vowels:[&str;5] = ["a", "e", "i", "o", "u"];
    for v in vowels.iter(){
        let m: Vec<&str> = line.matches(v).collect();
        cnt += m.iter().count();
        if cnt >= 3{
            return true
        }
    }
    return false;
}

//New Rule 1 - contains a pair of any two letters that appears at least twice (NO Overlapping)
fn pair_appears_twice(line:&str) -> bool {
    for i in 1..line.len(){
        let pair = &line[i-1..i+1];
        let m:Vec<&str> = line.matches(pair).collect();
        if m.iter().count() >=2{
            return true
        }
    }
    return false;
}

//New Rule 2 - contains at least one letter which repeats with exactly one letter between them
fn repeats_after_one(line:&str) -> bool {
    for i in 0..line.len(){
        if (i+2) < line.len(){
            if &line[i..i+1] == &line[i+2..i+3]{
                return true;
            }
        }
    }
    return false;
}

pub fn run(){
    let content = read_instructions("inputs/day-05.txt");
    let nice_basic = nice_or_naughty(&content, &Rules::Basic);
    let nice_adv = nice_or_naughty(&content, &Rules::Advanced);

    println!("\n-- AoC 2015: -- Day 5: Doesn't He Have Intern-Elves For This? --");
    println!("\n😇  Or 😈  (Basic Rules): {} \n😇  Or 😈  (Advanced Rules): {}", nice_basic, nice_adv);
    println!("\n-- DONE --\n");
}