use std::env;
use std::fs;
use std::path::Path;
use std::collections::HashMap;

#[derive(Copy,Clone,Debug)]
struct AuntSue<'a>{
    id:i32,
    prop:& 'a str,
    count:i32
}

fn read_sue_list(filename:&str)->String{
    let fpath = Path::new(filename);
    let abspath = env::current_dir()
        .unwrap()
        .into_boxed_path()
        .join(fpath);
    let sues = fs::read_to_string(abspath)
        .expect("Error reading list!");
    return sues;
}

fn make_ticker_tape<'a>()->HashMap<& 'a str,i32>{
    let mut ticker_tape:HashMap<&str,i32> = HashMap::new();
    ticker_tape.insert("children", 3);
    ticker_tape.insert("cats", 7);
    ticker_tape.insert("samoyeds", 2);
    ticker_tape.insert("pomeranians", 3);
    ticker_tape.insert("akitas", 0);
    ticker_tape.insert("vizslas", 0);
    ticker_tape.insert("goldfish", 5);
    ticker_tape.insert("trees", 3);
    ticker_tape.insert("cars", 2);
    ticker_tape.insert("perfumes", 1);
    return ticker_tape;
}

fn make_sue_list(sues:&str) ->HashMap<i32,Vec<AuntSue>>{
    let mut aunt_sues:HashMap<i32,Vec<AuntSue>>=HashMap::new();
    for sue in sues.lines(){
        let props:Vec<&str> = sue.split(|c:char| !c.is_alphanumeric())
            .filter(|&x| !x.is_empty())
            .collect();

            for i in 1..props.len(){
                match props[i].parse::<i32>(){
                    Ok(_) => continue,
                    Err(_) => {
                        let sue_id = props[1].parse::<i32>().unwrap();
                        let prop_count =props[i+1].parse::<i32>().unwrap();
                        if !aunt_sues.contains_key(&sue_id) {
                           aunt_sues.insert(sue_id, Vec::new());
                        }
                        let new_aunt = AuntSue{
                            id:sue_id,
                            prop:props[i],
                            count:prop_count
                        };
                        aunt_sues.get_mut(&sue_id).unwrap().push(new_aunt);
                    }
                }
            }
    }
    return aunt_sues;
}

fn find_matching_sue(aunt_sues:HashMap<i32,Vec<AuntSue>> , ticker_tape:HashMap<&str,i32>, is_outdated:bool)->i32{
    let (mut last_index, mut match_id) = (0,0);
    for (i,props) in &aunt_sues{
        let mut match_index = 0;
        for prop in props.iter(){
            if is_outdated{
                if *ticker_tape.get(&prop.prop).unwrap() == prop.count{
                    match_index+=1;
                }
                continue;
            }
            match prop.prop{
                "trees" | "cats" => {
                    if *ticker_tape.get(&prop.prop).unwrap() < prop.count{
                        match_index+=1;
                    }
                },
                "pomeranians" | "goldfish" => {
                    if *ticker_tape.get(&prop.prop).unwrap() > prop.count{
                        match_index+=1;
                    }                        
                },
                _ => {
                    if *ticker_tape.get(&prop.prop).unwrap() == prop.count{
                        match_index+=1;
                    }
                }
            }
        }
        if last_index < match_index{
            last_index=match_index;
            match_id= *i;
            //println!("{} {} {}", match_index, last_index, match_id);
        }
    }
    return match_id;
}

pub fn run(){
    let sues = read_sue_list("inputs/day-16.txt");
    let ticker_tape = make_ticker_tape();
    let aunts = make_sue_list(&sues);
    let aunt_sue = find_matching_sue(aunts.clone(), ticker_tape.clone(),true);
    let real_aunt_sue = find_matching_sue(aunts.clone(), ticker_tape.clone(),false);
    
    println!("\n-- Day 16: Aunt Sue --");
    println!("\n👩\tAunt Sue: {0:<39} \n\n🙋‍♀️\tReal Aunt Sue: {1:<34}\n", aunt_sue,real_aunt_sue);
    println!("\n-- DONE --\n");
}