use std::env;
use std::fs;
use std::path::Path;
use std::collections::HashMap;

const MATRIX_SIZE:usize = 8;

fn read_weights(filename:&str) -> String{
    let filepath = Path::new(filename);
    let abspath = env::current_dir()
        .unwrap()
        .into_boxed_path()
        .join(filepath);
    let mut content = fs::read_to_string(abspath)
        .expect("Error occured while reading file!");
    content = content
        .replace("happiness units by sitting next to", "")
        .replace("would", "")
        .replace(".", "");
    return content
}

fn build_weight_matrix(content:&str) ->  (Vec<Vec<i32>>,HashMap<&str,usize>){
    let mut keys:HashMap<&str,usize>=HashMap::new();
    let mut weights = vec![vec![0i32;MATRIX_SIZE];MATRIX_SIZE];
    let mut key:usize = 0;
    
    for line in content.lines(){
        let (mut l,mut r) = (0,0);
        let fields = line.split_whitespace().collect::<Vec<&str>>();

        if keys.contains_key(fields[0]){
            l = *keys.get(fields[0]).unwrap();
        }else{
            l=key;
            keys.entry(fields[0]).or_insert(l);
            key+=1;
        }
        if keys.contains_key(fields[3]) {
            r = *keys.get(fields[3]).unwrap();
        }else{
            r=key;
            keys.entry(fields[3]).or_insert(r);
            key+=1;
        }
        match fields[1]{
            "lose" => {
                weights[l][r] =fields[2].parse::<i32>().unwrap() * -1;
            },
            "gain" => {
                weights[l][r] =fields[2].parse::<i32>().unwrap();
            },
            &_ => panic!("Unintended Value!"),
        }
    }    

    return (weights, keys);
}

fn arrange_table(weights:Vec<Vec<i32>>, index:HashMap<&str,usize>, includes_you:bool)->i32{
    let mut keys:Vec<&str> = Vec::new();
    for k in index.keys(){
        keys.push(k)
    }   

    let next_p = |p:&mut Vec<usize>| {
        for i in (0..p.len()).rev(){
            if i == 0 || p[i] < p.len()-i-1{
                p[i]+=1;
                return
            }
            p[i] =0;
        }
    };
    let perm = |p:&mut Vec<usize>|{
        let mut res = keys.clone();
        for i in 0..p.len(){            
            res.swap(i, i+p[i]);
        }
        return res;
    };
    let calc_happiness = |perm:Vec<&str>|{
        let mut sum:i32 = 0;
        for i in 0..perm.len()-1{
            let ik = *index.get(perm[i]).unwrap();
            let iv = *index.get(perm[i+1]).unwrap();
            sum+= weights[ik][iv] + weights[iv][ik];
        }
        if !includes_you{
            let iz = *index.get(perm[0]).unwrap();
            let il = *index.get(perm[perm.len()-1]).unwrap();
            sum+=weights[iz][il] + weights[il][iz];
        }
        return sum
    };

    let mut max = 0;
    let mut permutations = vec![0;keys.len()];
    
    while permutations[0]<permutations.len()-1{
        next_p(&mut permutations);
        let sum= calc_happiness(perm(&mut permutations));
        if max < sum{
            max = sum;
        }
    }
    return max
}


pub fn run(){
    let content = read_weights("inputs/day-13.txt");
    let (weights, keys) = build_weight_matrix(&content);
    let max_happiness = arrange_table(weights.clone(), keys.clone(), false);
    let max_happiness_meh = arrange_table(weights.clone(), keys.clone(), true);

    println!("\n-- Day 13: Knights of the Dinner Table --");
    println!("\n😀  Max Happiness: {0: >29} \n😑  Max Happiness with Meh: {1: >20}", max_happiness, max_happiness_meh);
    println!("\n-- DONE --\n");
}