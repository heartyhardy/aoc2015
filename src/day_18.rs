use std::env;
use std::fs;
use std::path::Path;
use colored::*;
use std::{thread, time};

const MATRIX_SIZE:i32 = 100;


fn read_light_setup(filename:&str) -> String{
    let fpath = Path::new(filename);
    let abspath = env::current_dir()
        .unwrap()
        .into_boxed_path()
        .join(fpath);
    let light_data = fs::read_to_string(abspath)
        .expect("Error reading light data!");
    return light_data;
}

fn set_initial_light_config(light_data:&str,is_game_of_life:bool)->Vec<Vec<[i32;2]>>{
    let mut lights = vec![vec![[0i32;2];MATRIX_SIZE as usize];MATRIX_SIZE as usize];
    for (r, row) in light_data.lines().enumerate(){
        for (c,light) in row.chars().enumerate(){
            match light{
                '.' => lights[r][c] = [0,0],
                '#' => lights[r][c] = [1,0],
                _ => panic!("Unexpected value!")
            }
        }
    }

    if is_game_of_life{
        set_restrictions( &mut lights);
    }
    return lights;
}

fn set_restrictions(lights:&mut Vec<Vec<[i32;2]>>){
    let msize:usize = MATRIX_SIZE as usize;
    
    lights[0][0] = [1,0];
    lights[0][msize-1] = [1,0];
    lights[msize-1][0] = [1,0];
    lights[msize-1][msize-1] =[1,0];
}

fn let_there_be_light(lights:&mut Vec<Vec<[i32;2]>>, steps:i32, is_game_of_life:bool){
    let msize:usize = (MATRIX_SIZE -1) as usize;
    let restricted = vec![(0,0),(msize,0),(0,msize),(msize,msize)];
    for _t in 0..steps{
        for r in 0..MATRIX_SIZE{
            for c in 0..MATRIX_SIZE{   

                if is_game_of_life{
                    let is_restricted = restricted.iter()
                    .position(|&e|e.0 == r as usize && e.1 == c as usize);
                    if is_restricted != None{
                        continue;
                    }  
                }

                let (on, _off) = scan_adjacent(lights, r, c);
                match lights[r as usize][c as usize][0]{
                    0 =>  {
                        if on == 3 {
                            lights[r as usize][c as usize][1] = 1;
                            continue;
                        }
                        lights[r as usize][c as usize][1] = lights[r as usize][c as usize][0]
                    },
                    1 => {
                        if on != 2 && on != 3 {                           
                            lights[r as usize][c as usize][1]= 0;
                            continue;
                        }
                        lights[r as usize][c as usize][1] = lights[r as usize][c as usize][0]
                    },
                    _ => ()
                }
            }
        }
        debug_lights(lights,is_game_of_life);
    }
}

fn scan_adjacent(lights:&mut Vec<Vec<[i32;2]>>, r:i32, c:i32)->(i32, i32){
    let directions = vec![(0,-1),(1,-1),(1,0),(1,1),(0,1),(-1,1),(-1,0),(-1,-1)];
    let (mut on, mut off) = (0,0);
    for d in directions.iter(){
        let dc =d.0 + c;
        let dr = d.1+ r;
        if dc < MATRIX_SIZE && dr < MATRIX_SIZE && dc >= 0 && dr >= 0{
            match lights[dr as usize][dc as usize][0]{
                0 => off += 1,
                1 => on +=1,
                _  => ()
            }
        }
    }
    return (on, off);
}

fn debug_lights(lights:&mut Vec<Vec<[i32;2]>>, is_game_of_life:bool){
    let tsleep = time::Duration::from_millis(5);
    let msize:usize = (MATRIX_SIZE -1) as usize;
    let restricted = vec![(0,0),(msize,0),(0,msize),(msize,msize)];
    //print!("{}[2J", 27 as char);
    for r in 0..MATRIX_SIZE as usize{
        for c in 0..MATRIX_SIZE as usize{

            if is_game_of_life{
                let is_restricted = restricted.iter()
                .position(|&e|e.0 == r && e.1 == c);
                if is_restricted == None{
                    lights[r][c].swap(1, 0);
                }
            }else{
                lights[r][c].swap(1, 0);
            }

            match lights[r][c][0]{
                0 => print!("{}", "⚯".black()),
                1 => print!("{}", "⊷".bright_cyan() ),
                _ => ()
            }
        }
        println!("{}","");
    }
    thread::sleep(tsleep);
}

fn count_lights(lights:&mut Vec<Vec<[i32;2]>>) -> i32{
    let mut count = 0;
    for r  in 0..MATRIX_SIZE as usize{
        for c in 0..MATRIX_SIZE as usize{
            match lights[r][c][0]{
                1 => count+=1,
                _ => ()
            }
        }
    }
    return count;
}

pub fn run(){
    let light_data = read_light_setup("inputs/day-18.txt");
    let mut lights = set_initial_light_config(&light_data, false);
    let_there_be_light(&mut lights, 100, false);
    let count =count_lights(&mut lights);
    println!("\n\nPART I: {}", count);


    println!("{}", "\n \n Resuming in 5 seconds...\n");
    let tsleep = time::Duration::from_secs(5);
    thread::sleep(tsleep);


    let mut lights = set_initial_light_config(&light_data, true);
    let_there_be_light(&mut lights, 100, true);
    let count =count_lights(&mut lights);
    println!("\n\n PART II: {}", count);
}
