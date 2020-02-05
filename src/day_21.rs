use std::cmp;
use std::env;
use std::fs;
use std::path::Path;

#[derive(Copy, Clone, Debug)]
enum GameObjectClass {
    Player = 1,
    Enemy = 2,
}

#[derive(PartialEq, Copy, Clone, Debug)]
enum ItemCategory {
    Weapon = 1,
    Armor = 2,
    Misc = 3,
}

#[derive(PartialEq, Copy, Clone, Debug)]
enum ItemClass {
    Damage = 1,
    Defense = 2,
}

#[derive(Copy, Clone, Debug)]
struct Item<'a> {
    key: &'a str,
    val: i32,
    cost: i32,
    class: ItemClass,
    category: ItemCategory,
}

#[derive(Copy, Clone, Debug)]
struct GameObject<'a> {
    key: &'a str,
    hp: i32,
    dmg: i32,
    armor: i32,
}

fn read_shop_contents(filename: &str) -> String {
    let fpath = Path::new(filename);
    let abspath = env::current_dir().unwrap().into_boxed_path().join(fpath);
    let contents = fs::read_to_string(abspath).expect("Error reading shop details");
    return contents;
}

fn make_items_list(items: &str) -> Vec<Item> {
    let mut shop_items: Vec<Item> = Vec::new();
    let mut current_category = ItemCategory::Weapon;
    for itm in items.lines() {
        let props = itm
            .split(|c: char| !c.is_alphanumeric())
            .filter(|b| !b.is_empty())
            .collect::<Vec<&str>>();
        if props.len() <= 0 {
            continue;
        }

        match props[0] {
            "Weapons" => {
                current_category = ItemCategory::Weapon;
                continue;
            }
            "Armor" => {
                current_category = ItemCategory::Armor;
                continue;
            }
            "Rings" => {
                current_category = ItemCategory::Misc;
                continue;
            }
            _ => match current_category {
                ItemCategory::Weapon => {
                    let v1: i32 = props[2].parse().unwrap();
                    let v2: i32 = props[3].parse().unwrap();
                    let new_category = current_category.clone();
                    let new_item = Item {
                        key: props[0],
                        val: cmp::max(v1, v2),
                        class: ItemClass::Damage,
                        cost: props[1].parse().unwrap(),
                        category: new_category,
                    };
                    shop_items.push(new_item);
                }
                ItemCategory::Armor => {
                    let v1: i32 = props[2].parse().unwrap();
                    let v2: i32 = props[3].parse().unwrap();
                    let new_category = current_category.clone();
                    let new_item = Item {
                        key: props[0],
                        val: cmp::max(v1, v2),
                        class: ItemClass::Defense,
                        cost: props[1].parse().unwrap(),
                        category: new_category,
                    };
                    shop_items.push(new_item);
                }
                ItemCategory::Misc => {
                    let v1: i32 = props[2].parse().unwrap();
                    let v2: i32 = props[3].parse().unwrap();
                    let new_category = current_category.clone();
                    let itm_class: ItemClass;
                    if v1 > v2 {
                        itm_class = ItemClass::Damage
                    } else {
                        itm_class = ItemClass::Defense;
                    }
                    let new_item = Item {
                        key: props[0],
                        val: cmp::max(v1, v2),
                        class: itm_class,
                        cost: props[1].parse().unwrap(),
                        category: new_category,
                    };
                    shop_items.push(new_item);
                }
            },
        }
    }
    shop_items.sort_by(|a, b| b.cost.cmp(&a.cost));
    return shop_items;
}

fn find_optimal_gear(items: Vec<Item>) {
    let weapons = items
        .clone()
        .into_iter()
        .filter(|a| a.category == ItemCategory::Weapon)
        .collect::<Vec<Item>>();
    let armor = items
        .clone()
        .into_iter()
        .filter(|a| a.category == ItemCategory::Armor)
        .collect::<Vec<Item>>();
    let mis_dmg = items
        .clone()
        .into_iter()
        .filter(|a| a.category == ItemCategory::Misc && a.class == ItemClass::Damage)
        .collect::<Vec<Item>>();
    let mis_def = items
        .clone()
        .into_iter()
        .filter(|a| a.category == ItemCategory::Misc && a.class == ItemClass::Defense)
        .collect::<Vec<Item>>();

    let mut player = GameObject {
        key: "player",
        hp: 100,
        dmg: 0,
        armor: 0,
    };
    let mut enemy = GameObject {
        key: "enemy",
        hp: 104,
        dmg: 8,
        armor: 1,
    };

    let (mut min_cost, mut max_cost) = (std::i32::MAX, std::i32::MIN);
    for w in 0..weapons.len() {
        for a in 0..armor.len() {
            for rl in 0..mis_dmg.len() {
                for rr in 0..mis_def.len() {
                    reset(&mut player, &mut enemy);
                    let mut equipped:Vec<Item> = Vec::new();
                    equipped.push(weapons[w]);
                    equipped.push(armor[a]);
                    equipped.push(mis_dmg[rl]);
                    equipped.push(mis_def[rr]);
                    let cost = get_gear_cost(equipped.clone());
                    equip_player(&mut player, equipped);
                    let winner = death_match(&mut player, &mut enemy);
                    match winner.0{
                        GameObjectClass::Player => {
                            if min_cost > cost{
                                min_cost = cost;
                            }
                        },
                        GameObjectClass::Enemy => {
                            if max_cost < cost{
                                max_cost = cost;
                            }
                        }
                    }
                }
            }
        }
    }
    println!("\nOptimal Cost: {} \nMax Cost: {}\n", min_cost, max_cost);
}

fn reset(player: &mut GameObject, enemy: &mut GameObject) {
    player.hp = 100;
    player.armor = 0;
    player.dmg = 0;
    enemy.hp = 104;
    enemy.dmg = 8;
    enemy.armor = 1;
}

fn equip_player(player: &mut GameObject, items: Vec<Item>) {
    for itm in items.iter() {
        match itm.category {
            ItemCategory::Weapon => {
                player.dmg += itm.val;
            }
            ItemCategory::Armor => {
                player.armor += itm.val;
            }
            ItemCategory::Misc => match itm.class {
                ItemClass::Damage => {
                    player.dmg += itm.val;
                }
                ItemClass::Defense => {
                    player.armor += itm.val;
                }
            },
        }
    }
}

fn get_gear_cost(items:Vec<Item>) ->i32{
    let mut cost = 0;
    for itm in items.iter(){
        cost+=itm.cost;
    }
    return cost;
}

fn death_match(player: &mut GameObject, enemy: &mut GameObject) -> (GameObjectClass, i32) {
    let mut turns = 1;
    loop {
        enemy.hp -= player.dmg - enemy.armor;
        if enemy.hp <= 0 {
            return (GameObjectClass::Player, turns);
        }
        player.hp -= enemy.dmg - player.armor;
        if player.hp <= 0 {
            return (GameObjectClass::Enemy, turns);
        }
        turns += 1;
    }
}

pub fn run() {
    let items_list = read_shop_contents("inputs/day-21.txt");
    let shop_items = make_items_list(&items_list);
    find_optimal_gear(shop_items);
}
