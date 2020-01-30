use std::env;
use std::fs;
use std::path::Path;

#[derive(Copy,Clone,Debug)]
struct Ingredient<'a>{
    name:& 'a str,
    capacity:i32,
    durability:i32,
    texture:i32,
    flavor:i32,
    calories:i32
}

fn read_ingredients(filename:&str)->String{
    let fpath = Path::new(filename);
    let abspath = env::current_dir()
        .unwrap()
        .into_boxed_path()
        .join(fpath);
    let content = fs::read_to_string(abspath)
        .expect("Error reading ingredients file!");
    return content;
}

fn build_ingredients_table(contents:&str) -> Vec<Ingredient>{
    let mut ingredients:Vec<Ingredient>=Vec::new();
    for ing in contents.lines(){
        let fields:Vec<i32> = ing.split(|c:char| !c.is_numeric() && c != '-')
            .filter(|&x| !x.is_empty())
            .map(|v|v.parse().unwrap())
            .collect();
        let new_ing = Ingredient{
            name:ing.split(":").collect::<Vec<&str>>()[0],
            capacity:fields[0],
            durability:fields[1],
            flavor:fields[2],
            texture:fields[3],
            calories:fields[4]
        };
        ingredients.push(new_ing);
    }
    return ingredients;
}

fn make_recipe(ingredients:&mut Vec<Ingredient>, amounts:Vec<i32>) -> (i32, i32){
    let (mut capacity, mut durability, mut flavor, mut texture, mut calories) = (0,0,0,0,0);
    for(i, ing) in ingredients.iter().enumerate(){
        capacity += ing.capacity * amounts[i];
        durability += ing.durability * amounts[i];
        flavor += ing.flavor * amounts[i];
        texture += ing.texture * amounts[i];
        calories += ing.calories *amounts[i];
    }
    if capacity < 0{
        capacity=0;
    }
    if durability <0{
        durability=0;
    }
    if flavor < 0{
        flavor=0;
    }
    if texture <0{
        texture=0;
    }
    return (calories, capacity * durability * flavor * texture);
}

fn shake_it_n_make_it(mut ingredients:&mut Vec<Ingredient>, is_score_based:bool)-> i32{
    let mut max_score = 0;
    for i in 1..101{
        for j in 1..101{
            for k in 1..101{
                for l in 1..101{
                    if i+j+k+l == 100{
                        let amounts:Vec<i32> =vec![i,j,k,l];
                        let (calories, score) = make_recipe(&mut ingredients, amounts);
                        if is_score_based && max_score<score{
                            max_score= score;
                            continue;
                        }
                        if !is_score_based && calories == 500 && max_score < score{
                            max_score=score;
                        }
                    }
                }
            }
        }
    }
    return max_score;
}

pub fn run(){
    let content = read_ingredients("inputs/day-15.txt");
    let ingredients = build_ingredients_table(&content);
    let score_mode = shake_it_n_make_it(&mut ingredients.clone(), true);
    let calorie_mode  = shake_it_n_make_it(&mut ingredients.clone(), false);
    
    println!("\n-- Day 15: Science for Hungry People --");
    println!("\n🍔  Best Scored Recipe: {0: >31} \n🥗  Calorie Based Recipe: {1: >29}", score_mode, calorie_mode);
    println!("\n-- DONE --\n");
}