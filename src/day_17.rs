use std::env;
use std::fs;
use std::path::Path;

const EGGNOG_LITRES:i32 = 150;

fn read_container_data(filename:&str)->String{
    let fpath = Path::new(filename);
    let abspath = env::current_dir()
        .unwrap()
        .into_boxed_path()
        .join(fpath);
    let contents = fs::read_to_string(abspath)
        .expect("Error reading container data!");
    return contents;
}

fn find_combinations(container_data:&str)->i32{
    let mut containers:Vec<i32> = container_data
        .lines()
        .map(|x| x.parse().unwrap())
        .collect();
    containers.sort_by(|a,b| b.cmp(a));

    let size:i32 = containers.len() as i32;
    let count = rec_count(&mut containers, EGGNOG_LITRES, size, 0);
    return count;
}

fn find_min_combinations(container_data:&str)->i32{
    let mut containers:Vec<i32> = container_data
        .lines()
        .map(|x| x.parse().unwrap())
        .collect();
    containers.sort_by(|a,b| b.cmp(a));

    let min_cmb = rmin(&mut containers, EGGNOG_LITRES);
    return min_cmb;
}

fn rec_count(containers:&mut Vec<i32>,target:i32, size:i32, i:i32)->i32{
    if size<0{
        return 0;
    }else if target==0{
        return 1;
    }else if i == containers.len() as i32 || target < 0{
        return 0;
    }else {
        return rec_count(containers, target, size, i+1) +
            rec_count(containers, target-containers[i as usize], size-1, i+1);
    }
}

fn rmin(containers:&mut Vec<i32>,target:i32)->i32{
    let (mut i, mut min) = (1, -1);
    while min <= 0{
        min = rec_count(containers, target, i, 0);
        i+=1;
    }
    return min;
}


pub fn run(){
    let containers = read_container_data("inputs/day-17.txt");
    let count = find_combinations(&containers);
    let min_count = find_min_combinations(&containers);

    println!("\n-- Day 17: No Such Thing as Too Much --");
    println!("\n✳\tAll Combinations: {0:<39} \n\n✴\tMin Combinations: {1:<34}\n", count, min_count);
    println!("\n-- DONE --\n");
}