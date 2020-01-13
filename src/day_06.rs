use std::env;
use std::fs;
use std::path::Path;

const MATRIX_SIZE:usize =1000;

enum LightMode{
    Switch,
    Dimmer,
}

enum Ops{
    On,
    Off,
    Toggle,   
}


fn read_instructions(filename:&str)->String{
    let fpath = Path::new(filename);
    let abspath = env::current_dir()
        .unwrap()
        .into_boxed_path()
        .join(fpath);
    let content = fs::read_to_string(abspath)
        .expect("Error occurred while reading file!");
    return content
}

fn let_there_be_light(content:&String, light_mode:&LightMode) -> (i32, i32){
    let light_stats:(i32, i32);
    let mut lights = vec![vec![0;MATRIX_SIZE];MATRIX_SIZE];
    for line in content.lines(){
        let (mode, start, end) = parse_line(&line);
        set_lights(&mut lights, &light_mode,&mode, start, end);
    }
    match light_mode{
        &LightMode::Switch => light_stats = get_light_stats(&lights, light_mode),
        &LightMode::Dimmer => light_stats = get_light_stats(&lights, light_mode),
    }
    return light_stats;
}

fn parse_line(line:&str) -> (Ops, Vec<&str>, Vec<&str>){
    let fields:Vec<&str> = line.split_whitespace().collect();
    match fields[0]{
        "toggle" => {
            let mode = Ops::Toggle;
            let start:Vec<&str> = fields[1].split(",").collect();
            let end:Vec<&str> = fields[3].split(",").collect();
            (mode, start, end)
        },
        "turn" => {
            let mode:Ops;
            match fields[1]{
                "on" => mode = Ops::On,
                "off" => mode = Ops::Off,
                _ => panic!()
            }
            let start:Vec<&str> = fields[2].split(",").collect();
            let end:Vec<&str> = fields[4].split(",").collect();
            (mode, start, end)
        }
        &_ => panic!()
    }
}

fn set_lights(lights:&mut Vec<Vec<i32>>,pattern:&LightMode, mode:&Ops, start:Vec<&str>, end:Vec<&str>){
    let rs:usize = start[0].parse().unwrap();
    let cs:usize = start[1].parse().unwrap();
    let re:usize = end[0].parse().unwrap();
    let ce:usize = end[1].parse().unwrap();

    for r in rs..re+1{
        for c in cs..ce+1{
            match pattern{
                &LightMode::Switch => {
                    match mode{
                        &Ops::On => {
                            lights[r][c] = 1;
                        }
                        &Ops::Off => {
                            lights[r][c] = 0;
                        }
                        &Ops::Toggle => {
                            lights[r][c] ^= 1;
                        }
                    }
                }
                &LightMode::Dimmer => {
                    match mode{
                        &Ops::On => {
                            lights[r][c] += 1;
                        }
                        &Ops::Off => {
                            lights[r][c] -= 1;
                            if lights[r][c] < 0{
                                lights[r][c] =0;
                            }
                        }
                        &Ops::Toggle => {
                            lights[r][c] += 2;
                        }
                    }
                }
            }
        }
    }  
}

fn get_light_stats(lights:&Vec<Vec<i32>>, light_mode:&LightMode) -> (i32, i32){
    let (mut on, mut off) = (0, 0);
    for r in 0..MATRIX_SIZE{
        for c in 0..MATRIX_SIZE{
            match light_mode{
                &LightMode::Switch => {
                    if lights[r][c] == 1{
                        on +=1;
                    }else{
                        off +=1;
                    }
                }
                &LightMode::Dimmer => {
                    on += lights[r][c];
                }   
            }
        }
    }
    (on, off)
}

pub fn run(){
    let content = read_instructions("inputs/day-06.txt");
    let light_stats_switch = let_there_be_light(&content, &LightMode::Switch);
    let light_stats_dimmer = let_there_be_light(&content, &LightMode::Dimmer);

    println!("\n-- AoC 2015: -- Day 5: Doesn't He Have Intern-Elves For This? --");
    println!("\n💡  Turned (ON/OFF): {} / {} \n🔆  Total Brightness: {}", light_stats_switch.0, light_stats_switch.1, light_stats_dimmer.0);
    println!("\n-- DONE --\n");
}