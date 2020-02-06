use std::env;
use std::fs;
use std::path::Path;
use std::collections::HashMap;

fn read_instructions(filename:&str) -> String{
    let fpath = Path::new(filename);
    let abspath = env::current_dir()
        .unwrap()
        .into_boxed_path()
        .join(fpath);
    let replaced = fs::read_to_string(abspath)
        .expect("Error reading instructions!");
    let instructions = replaced.replace(",", "");
    return instructions;
}

fn parse_instructions(instructions_list:&str, is_overriden:bool){
    let mut registers:HashMap<&str,u32> = HashMap::new();
    let mut instructions:Vec<Vec<&str>> = Vec::new();
    for instruction in instructions_list.lines(){
        let ops = instruction
        .split_whitespace()
        .collect::<Vec<&str>>();
        instructions.push(ops);
    }

    if is_overriden{
        registers.insert("a", 1);
    }

    let mut i = 0;
    loop {
        if i >= instructions.len(){
            break;
        }
        match instructions[i][0]{
            "hlf"  => {
                if !registers.contains_key(&instructions[i][1]){
                    registers.insert(instructions[i][1], 0);
                }
                let r = *registers.get(&instructions[i][1]).unwrap();
                *registers.get_mut(&instructions[i][1]).unwrap() = r/2;
                i+=1;
            },
            "tpl" => {
                if !registers.contains_key(&instructions[i][1]){
                    registers.insert(instructions[i][1], 0);
                }
                *registers.get_mut(&instructions[i][1]).unwrap() *= 3;
                i+=1;
            },
            "inc" => {
                if !registers.contains_key(&instructions[i][1]){
                    registers.insert(instructions[i][1], 0);
                }
                *registers.get_mut(&instructions[i][1]).unwrap() += 1;
                i+=1;
            },
            "jmp" => {
                let inc:i32 =instructions[i][1].parse().unwrap();
                if inc < 0{
                    i -= (inc * -1) as usize;
                    continue;
                }
                i += inc as usize;
            },
            "jie" => {
                if !registers.contains_key(&instructions[i][1]){
                    registers.insert(instructions[i][1], 0);
                }
                let r = *registers.get(&instructions[i][1]).unwrap();
                if r % 2 == 0{
                    let inc:i32 =instructions[i][2].parse().unwrap();
                    if inc < 0{
                        i -= (inc * -1) as usize;
                        continue;
                    }
                    i += inc as usize;
                    continue;
                }
                i+=1;
            },
            "jio" => {
                if !registers.contains_key(&instructions[i][1]){
                    registers.insert(instructions[i][1], 0);
                }
                let r = *registers.get(&instructions[i][1]).unwrap();
                if r == 1{
                    let inc:i32 =instructions[i][2].parse().unwrap();
                    if inc < 0{
                        i -= (inc * -1) as usize;
                        continue;
                    }
                    i += inc as usize;
                    continue;
                }
                i+=1;
            },
            _=> {
                break;
            }
        }
    }
    println!("{:?}", registers);
}

pub fn run(){
    let instructions = read_instructions("inputs/day-23.txt");
    parse_instructions(&instructions, false);
    parse_instructions(&instructions, true);
}