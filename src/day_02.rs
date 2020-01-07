use std::env;
use std::fs;
use std::cmp;
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

fn calc_current(vec: &mut Vec<&str>) -> (i32, i32){
    let (l, w, h):(i32, i32, i32) = (
        vec[0].parse().unwrap(),
        vec[1].parse().unwrap(),
        vec[2].parse().unwrap()
    );
    let (lw, wh, hl) = (2*l*w, 2*w*h, 2*h*l);
    // Part I - calculate paper
    let ma = cmp::min(hl, cmp::min(lw, wh)) / 2;
    // Part II - calculate ribbons
    let mp = cmp::min(2*(l+w), cmp::min(2*(w+h), 2*(h+l)));
    let bow = l * w * h;

    (lw + wh + hl + ma, mp+bow)
}

fn calc_all(content: String) -> (i32, i32) {
    let (mut paper, mut ribbons) = (0, 0);

    let lines = content.lines();
    for l in lines{
        let mut vec: Vec<&str> = l.split("x").collect();
        let (p, r) = calc_current(&mut vec);
        paper += p;
        ribbons += r;
    }
    (paper, ribbons)
}

pub fn run(){
    let content = read_instructions("inputs/day-02.txt");
    let res = calc_all(content);

    println!("\n-- AoC 2015: Day 02 - I Was Told There Would Be No Math --");
    println!("\nTotal Paper: {} \nTotal Ribbons: {}", res.0, res.1 );
    println!("\n-- DONE --\n");
}