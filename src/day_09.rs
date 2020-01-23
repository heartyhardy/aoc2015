use std::env;
use std::fs;
use std::path::Path;
use std::collections::HashMap;

const MATRIX_SIZE:usize = 8;

fn read_instructions(filename:&str) ->String{
    let fpath = Path::new(filename);
    let abspath = env::current_dir()
        .unwrap()
        .into_boxed_path()
        .join(&fpath);
    let contents = fs::read_to_string(&abspath)
        .expect("Error reading input file!");
    return contents
}

fn fill_matrix(contents:&str) -> Vec<Vec<usize>>{    
    let mut keys:HashMap<&str,usize> =HashMap::new();
    let mut matrix = vec![vec![0usize;MATRIX_SIZE];MATRIX_SIZE];

    let replaced = contents
                        .replace("to", " ")
                        .replace("=", "");
    let mut key:usize = 0;
    for line in replaced.lines(){
        let (l,r);
        let fields = line.split_whitespace().collect::<Vec<&str>>();

        if keys.contains_key(fields[0]){
            l = *keys.get(fields[0]).unwrap();
        }else{
            l=key;
            keys.entry(fields[0]).or_insert(l);
            key +=1;
        }
        if keys.contains_key(fields[1]){
            r = *keys.get(fields[1]).unwrap();
        }else{
            r = key;
            keys.entry(fields[1]).or_insert(r);
            key +=1;
        }
        
        matrix[l][r] = fields[2].parse().unwrap();
        matrix[r][l] =fields[2].parse().unwrap();
    }
    return matrix
}

fn find_min_distance(matrix:Vec<Vec<usize>>) -> usize{

    let mut shortest_path = std::usize::MAX;

    for i in 0..MATRIX_SIZE{
        let mut closed:HashMap<usize,bool> = HashMap::new();
        let (mut min_dis, mut min_node, mut current, mut distance) =
        (std::usize::MAX, 0usize, 0usize, 0usize);
        let mut min_changed = false;

        while !closed.contains_key(&current) {            
            closed.insert(current, true);
            for j in 0..MATRIX_SIZE{
                if min_dis > matrix[current][j] && matrix[current][j] != 0 && !closed.contains_key(&j){
                    min_dis = matrix[current][j];
                    min_node = j;
                    min_changed=true;
                }        
            }
            if min_changed {
                distance+=min_dis;
                current = min_node;
                min_node= 0;
                min_dis =std::usize::MAX;
                min_changed=false;
                continue;
            }
            break;
        }
        if shortest_path > distance{
            shortest_path= distance
        }
    }
    return shortest_path;
}


fn find_max_distance(matrix:Vec<Vec<usize>>) -> usize{

    let mut longest_path = std::usize::MIN;

    for i in 0..MATRIX_SIZE{
        let mut closed:HashMap<usize,bool> = HashMap::new();
        let (mut max_dis, mut max_node, mut current, mut distance) =
        (std::usize::MIN, 0usize, i, 0usize);
        let mut max_changed = false;

        while !closed.contains_key(&current) {            
            closed.insert(current, true);
            for j in 0..MATRIX_SIZE{
                if max_dis < matrix[current][j] && matrix[current][j] != 0 && !closed.contains_key(&j){
                    max_dis = matrix[current][j];
                    max_node = j;
                    max_changed=true;
                }        
            }
            if max_changed {
                distance+=max_dis;
                current = max_node;
                max_node= 0;
                max_dis =std::usize::MIN;
                max_changed=false;
                continue;
            }
            break;
        }
        if longest_path < distance{
            longest_path= distance
        }
    }
    return longest_path;
}

pub fn run(){
    let content = read_instructions("inputs/day-09.txt");
    let matrix_min = fill_matrix(&content);
    let matrix_max = matrix_min.clone();

    let min_distance = find_min_distance(matrix_min);
    let max_distance = find_max_distance(matrix_max);

    println!("\n-- AoC 2015: -- Day 9: All in a Single Night --");
    println!("\n🚀  Shortest Path: {} \n🚂  Longest Path: {}", min_distance, max_distance);
    println!("\n-- DONE --\n");
    
}