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

    let size:i32 = (containers.len()-1) as i32;
    let count = rcount(&mut containers, EGGNOG_LITRES, size);
    return count;
}

fn rcount(containers:&mut Vec<i32>,target:i32,i:i32)->i32{
    if target == 0{
        return 1;
    }else if target < 0{
        return 0;
    }else if i < 0{
        return 0;
    }else if target < containers[i as usize]{
        return rcount(containers, target, i-1);
    }else{
        return rcount(containers, target-containers[i as usize], i-1)+
            rcount(containers, target, i-1)
    }
}

fn find_min_combinations(container_data:&str)->i32{
    let mut containers:Vec<i32> = container_data
        .lines()
        .map(|x| x.parse().unwrap())
        .collect();
    //containers.sort_by(|a,b| b.cmp(a));

    let size:i32 = containers.len() as i32;
    let min_cmb = rmin(&mut containers, size, EGGNOG_LITRES);
    return min_cmb;
}

fn rmin(containers:&mut Vec<i32>,size:i32,target:i32)->i32{
    if target == 0{
        return 0
    }
    let mut res = std::i32::MAX;

    for i in 0..size as usize{
        if containers[i] <= target{
            let min_sub = rmin(containers, size, target - containers[i]);
            
            if min_sub != std::i32::MAX && (min_sub +1) < res {
                res = min_sub+1;
            }
        }
    }
    return res;
}


pub fn run(){
    let containers = read_container_data("inputs/day-17.txt");
    let count = find_combinations(&containers);
    println!("{}", count );

    let min_count = find_min_combinations(&containers);
    println!("{}", min_count);
}