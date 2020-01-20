use std::env;
use std::fs;
use std::path::Path;
use std::collections::HashMap;

fn read_instructions(filename:&str) ->String{
    let fpath = Path::new(filename);
    let abspath = env::current_dir()
        .unwrap()
        .into_boxed_path()
        .join(fpath);
    let content = fs::read_to_string(abspath)
        .expect("Error occurred while reading file!");
    return content
}

fn map_all_fn(content:&String)-> (HashMap<&str,Vec<&str>>, HashMap<&str,Vec<&str>>, HashMap<&str,u16>){
    let mut fns: HashMap<&str,Vec<&str>> =  HashMap::new();
    let mut vns: HashMap<&str,u16> = HashMap::new();
    let mut cns: HashMap<&str, Vec<&str>> = HashMap::new();

    for line in content.lines(){
        let fields:Vec<&str> = line.split(|c:char|! c.is_alphanumeric())
            .filter(|&x| !x.is_empty())
            .collect();
        let last = fields.last().unwrap();

        match fields.len(){
            2 => {
                match fields[0].parse::<u16>(){
                    Ok(v) => {
                        let ops:Vec<&str> = vec!["","","",fields[0],last];
                        fns.insert(last, ops);
                        vns.insert(last, v);
                    },
                    Err(_) =>{
                        let ops:Vec<&str> = vec!["EQ",fields[0],"","0",last];
                        let links:Vec<&str> = vec![last, fields[0]];
                        fns.insert(last, ops);
                        map_connections(last, &mut cns, &links);
                    }
                }
            },
            3 => {
                let ops:Vec<&str> = vec![fields[0],fields[1],"","0",last];
                let links:Vec<&str> = vec![last, fields[1]];
                fns.insert(last, ops);
                map_connections(last, &mut cns, &links);
            },
            4 => {
                let ops:Vec<&str> = vec![fields[1],fields[0],fields[2],"0",last];
                let links:Vec<&str> = vec![last, fields[0],fields[2]];
                fns.insert(last, ops);
                map_connections(last, &mut cns, &links);
            },
            _ => panic!("Unexpected field length!")
        }
    }
    return (fns,cns,vns)
}

fn map_connections<'a>(w: &'a str, cns:&mut HashMap<&'a str,Vec<&'a str>>, ops:&Vec<&'a str>){
    for op in ops.iter(){
        match op.parse::<u16>(){
            Ok(_) => return,
            Err(_) => {
                if cns.contains_key(op) && op.len() > 0{
                    cns.get_mut(op).unwrap().push(w);
                }else if !cns.contains_key(op) && op.len() >0{
                    let links:Vec<&str> =vec![w];
                    cns.insert(op, links);
                }
            }
        }
    }
}

fn try_solve_all<'a>(
    fns:&HashMap<&'a str,Vec<&'a str>>,
    cns:&HashMap<&'a str,Vec<&'a str>>,
    mut vns:&mut HashMap<&'a str, u16>
){
    let mut solved = false;
    loop{
        for (k,v) in cns.values().enumerate(){
            for c in v{
                solved &= try_solve(c, fns, &mut vns);
            }
        }
        if !solved{
            solved=true;
        }else{
            break;
        }
    }    
}

fn try_solve<'a>(
    w:&str,
    fns:&HashMap<&'a str,Vec<&'a str>>,
    vns:&mut HashMap<&'a str, u16>
)->bool{
    if fns.contains_key(w) {
        let ops = fns.get(w).unwrap();
        match ops[0]{
            "" => {
                let (_,lok) = try_fetch_value(ops[3], &vns);
                return lok;
            },
            "EQ" => {
                let (l, lok) = try_fetch_value(ops[1], &vns);
                if lok {
                    vns.entry(ops[4]).or_insert(l);
                }
                return lok;
            },
            "NOT" => {
                let (l, lok) = try_fetch_value(ops[1], &vns);
                if lok{
                    vns.entry(ops[4]).or_insert(!l);                    
                }
                return lok;
            },
            _ => {
                let (l, lok) = try_fetch_value(ops[1], &vns);
                let (r, rok) = try_fetch_value(ops[2], &vns);
                if lok && rok{
                    match ops[0]{
                        "AND" => {
                            vns.insert(ops[4],l &r);
                        },
                        "OR" => {
                            vns.insert(ops[4], l|r);
                        },
                        "LSHIFT" =>{
                            vns.insert(ops[4], l<<r);
                        },
                        "RSHIFT" =>{
                            vns.insert(ops[4], l>>r);
                        },
                        _ => panic!("Unexpected OP code!")
                    }
                    return true;
                }
            }            
        }
    }
    return false;
}

fn try_fetch_value<'a>(w: &'a str, vns: &HashMap<&'a str,u16>) -> (u16,bool){
    match w.parse::<u16>() {
        Ok(v) => {
            return (v, true);
        },
        Err(_)=>{
            if w.len() > 0{
                if vns.contains_key(w){
                    return (vns[w],true);
                }
                return (0, false);
            }
        }
    }
    return (0, false);
}

pub fn run(){
    let content = read_instructions("inputs/day-07.txt");
    let (fns,cns,mut vns) = map_all_fn(&content);
    try_solve_all(&fns, &cns, &mut vns);
    let wire_a = vns["a"];

    //Reset and override B from Previous A wire signal
    let (fns,cns,mut vns) = map_all_fn(&content);
    vns.insert("b", wire_a);
    try_solve_all(&fns, &cns, &mut vns);
    let wire_a_ovr = vns["a"];

    println!("\n-- AoC 2015: Day 7: Some Assembly Required --");
    println!("\n⚡  Wire A: {} \n⚡⚡ Wire A after override: {}", wire_a, wire_a_ovr );
    println!("\n-- DONE --\n");
}