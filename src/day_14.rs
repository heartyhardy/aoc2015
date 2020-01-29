use std::env;
use std::fs;
use std::path::Path;

#[derive(Copy,Clone,Debug)]
struct Raindeer<'a>{
    name:&'a str,
    speed:i32,
    runtime:i32,
    sleep:i32,
    distance:i32,
    points:i32,
}

fn read_racer_stats(filename:&str)->String{
    let fpath=Path::new(filename);
    let abspath = env::current_dir()
        .unwrap()
        .into_boxed_path()
        .join(fpath);
    let content = fs::read_to_string(abspath)
        .expect("Error reading Racer stats file!");
    return mass_replace(&content)
}

fn mass_replace(content: &str)->String{
    return content.replace("can fly", "")
        .replace("km/s for", "")
        .replace("seconds, but then must rest for", "")
        .replace("seconds.", "")
        .trim()
        .to_string();
}

fn collect_racer_stats(stats: &str) -> Vec<Raindeer>{
    let mut racers:Vec<Raindeer> =Vec::new();
    for stat in stats.lines(){
        let fields:Vec<&str> = stat
            .split_ascii_whitespace()
            .collect();
        let racer = Raindeer{
            name:fields[0],
            speed: fields[1].parse().unwrap(),
            runtime: fields[2].parse().unwrap(),
            sleep:fields[3].parse().unwrap(),
            distance:0,
            points:0,
        };
        racers.push(racer);
    }
    return racers;
}

fn let_it_race_dep(racers:Vec<Raindeer>,dt:i32)->(String, i32){
    let (mut max, mut winner) = (0, String::new());
    for i in 0..racers.len(){
        let mut next = racers[i].runtime;
        let (mut distance,mut t) = (0,0);
    
        while t < dt{
            if t == next{
                t += racers[i].sleep;
                next = t+racers[i].runtime;
                continue;
            }
            distance+=racers[i].speed;
            t+=1;
        }
        if max < distance{
            max=distance;
            winner = String::from(racers[i].name.clone());
        }
    }
    return (winner, max);
}

fn let_it_race(mut deers:Vec<Raindeer>, dt:i32) -> Raindeer{
    for deer in deers.iter_mut(){
        let tseg = dt/(deer.sleep + deer.runtime);
        let trem = dt%(deer.sleep + deer.runtime);
        let di = deer.runtime * deer.speed * tseg;
        let dr = trem / deer.runtime;
        if dr>0{
            deer.distance = di + (1 * deer.speed * deer.runtime);
            continue;
        }
        deer.distance = di+(trem%deer.runtime)*deer.speed;
    }
    deers.sort_by(|a,b|b.distance.cmp(&a.distance));
    let winner = deers[0];
    return winner;
}

fn let_it_race_points(mut deers:Vec<Raindeer>, dt:i32)->Raindeer{
    for i in 1..dt+1{
        for deer in deers.iter_mut(){
            let tseg = i/(deer.sleep + deer.runtime);
            let trem = i%(deer.sleep + deer.runtime);
            let di = deer.runtime * deer.speed * tseg;
            let dr = trem / deer.runtime;
            if dr>0{
                deer.distance = di + (1 * deer.speed * deer.runtime);
                continue;
            }
            deer.distance = di+(trem%deer.runtime)*deer.speed;
        }
        deers.sort_by(|a,b|b.distance.cmp(&a.distance));        
        for d in 0..deers.len(){            
            if deers[d].distance !=  deers[0].distance{
                break;
            }
            deers[d].points=deers[d].points+1;
        }
    }
    deers.sort_by(|a,b|(b.distance+b.points).cmp(&(a.distance+a.points)));
    return deers[0];
}

pub fn run(){
    let stats = read_racer_stats("inputs/day-14.txt");
    let racers = collect_racer_stats(&stats);
    let winner = let_it_race(racers.clone(), 2503);
    let new_winner = let_it_race_points(racers.clone(), 2503);
    
    println!("\n-- Day 14: Reindeer Olympics --");
    println!("\n\t🦌  {0:<10} ran {1:>20} Km\n", winner.name, winner.distance);
    println!("\n\t🦌  {0:<10} earned {1:>20} Points\n", new_winner.name, new_winner.points);
    println!("\n-- DONE --\n");
}