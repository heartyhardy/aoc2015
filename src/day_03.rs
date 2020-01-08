use std::env;
use std::fs;
use std::path::Path;
use std::collections::HashMap;

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

fn deliver_presents(content: &String) -> usize{
   let (mut x, mut y) = (0, 0);
    let mut grid: HashMap<(i32, i32), i32> = HashMap::new();
    let mut xy:(i32, i32) = (x,y);
    grid.insert(xy, 1);

    for c in content.chars(){
       match c{
           '^' => y+=1,
           'v' => y-=1,
           '<' => x-=1,
           '>' => x+=1,
           _ => panic!()
       }
       xy = (x,y);
       *grid.entry(xy).or_insert(1) += 1;
    }
    return grid.len();
}

fn deliver_presents_duo(content: &String) -> usize{
    let (mut x1, mut y1, mut x2, mut y2) = (0, 0, 0, 0);
    let mut grid: HashMap<(i32, i32), i32> = HashMap::new();
    let mut xy:(i32, i32) = (x1,y1);
    let mut turn = 0;
    grid.insert(xy, 1);    

    for c in content.chars(){
        match turn{
            0 => {
                match c{
                    '^' => y1+=1,
                    'v' => y1-=1,
                    '<' => x1-=1,
                    '>' => x1+=1,
                    _ => panic!()
               }
               xy = (x1,y1);
               *grid.entry(xy).or_insert(1) += 1;
            },
            1 => {
                match c{
                    '^' => y2+=1,
                    'v' => y2-=1,
                    '<' => x2-=1,
                    '>' => x2+=1,
                    _ => panic!()
               }
               xy = (x2,y2);
               *grid.entry(xy).or_insert(1) += 1;
            },
            _ => panic!()
        }
        turn = turn ^ 1;
    }
    return grid.len();
 }

pub fn run(){
    let content = read_instructions("inputs/day-03.txt");
    let santa_run = deliver_presents(&content);
    let duo_run = deliver_presents_duo(&content);

    println!("\n-- AoC 2015: Day 3: Perfectly Spherical Houses in a Vacuum --");
    println!("\nSolo Run: {} \nDuo Run with Santa Bot: {}", santa_run, duo_run );
    println!("\n-- DONE --\n");
}