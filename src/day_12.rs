use std::env;
use std::fs;
use std::path::Path;
use serde_json;
use serde_json::{Value};

fn read_json_file(filename:&str) -> serde_json::Value{
    let filepath = Path::new(filename);
    let abspath = env::current_dir()
                .unwrap()
                .into_boxed_path()
                .join(&filepath);
    let contents = fs::read_to_string(&abspath)
        .expect("Error occured while reading file");
        let json:Value = serde_json::from_str(&contents).unwrap();
    return json;
}

fn sum_up_all(json: serde_json::Value, skip_reds:bool) -> i64{
    let mut sum = 0;
    match json{
        Value::Null=> return 0,
        Value::Bool(_) => return 0,
        Value::String(_) => return 0,
        Value::Number(n) => {
            sum = sum + n.as_i64().unwrap();
            return sum;
        },
        Value::Array(arr) => {
            for num in arr.iter(){
                sum = sum + sum_up_all(num.clone(),skip_reds);
            }
            return sum;
        },
        Value::Object(m) => {
            for (_,mv) in m.into_iter(){
                if skip_reds && mv.is_string() && mv.as_str().unwrap() == "red"{
                    return 0;
                }
                sum += sum_up_all(mv.clone(),skip_reds);
            }
            return sum;
        }        
    }
}

pub fn run(){
    let json = read_json_file("inputs/day-12.txt");
    let sum_all = sum_up_all(json.clone(),false);
    let sum_no_reds = sum_up_all(json.clone(),true);

    println!("\n-- AoC 2015: -- Day 12: JSAbacusFramework.io --");
    println!("\n➕     Sum up all: {0: >26} \n➕  🔴  Sum No REDS: {1: >24}", sum_all, sum_no_reds);
    println!("\n-- DONE --\n");
}