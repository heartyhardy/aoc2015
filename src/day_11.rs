
use std::collections::HashMap;

fn gen_new_password(old:Vec<u8>) ->String{
    let mut new_pass = increment(old);
    loop{
        new_pass = increment(new_pass);
        if has_only_valid(&new_pass) && has_incrementing_set(&new_pass) && has_pairs(&new_pass){
            return String::from_utf8(new_pass).unwrap()
        }
    }
}

fn increment(password:Vec<u8>)->Vec<u8>{
    let mut bytes = password.clone();
    for i in (0..bytes.len()).rev(){
        if bytes[i] >=57 && bytes[i] < 97{
            bytes[i] = 48;
        }else if bytes[i] >= 122{
            bytes[i] = 97;
        }else{
            bytes[i]+=1;
            break
        }
    }
    return bytes
}

fn has_only_valid(bytes:&Vec<u8>) -> bool{
    for i in 0..bytes.len(){
        if bytes[i] == 105 || bytes[i] == 108 || bytes[i] == 111{
            return false;
        }
    }
    return true;
}

fn has_incrementing_set(bytes:&Vec<u8>) ->bool{
    let mut is_inc = 0;
    for i in 1..bytes.len(){
        if bytes[i] == bytes[i-1]+1{
            is_inc+=1;
            if is_inc >=2{
                return true;
            }
        }else{
            is_inc = 0;
        }
    }
    return false;
}

fn has_pairs(bytes:&Vec<u8>) ->bool{
    let mut pairs:HashMap<u8,u8>=HashMap::new();
    for i in 1..bytes.len(){
        if bytes[i] == bytes[i-1] && !pairs.contains_key(&bytes[i]){
            pairs.insert(bytes[i], bytes[i]);
        }
    }
    if pairs.len() >= 2{
        return true;
    }
    return false;
}

pub fn run(){
    let old_password = String::from("hepxcrrq").into_bytes();
    let first= gen_new_password(old_password);
    let last = first.clone().into_bytes();
    let second =gen_new_password(last);

    println!("\n-- AoC 2015: -- Day 11: Corporate Policy --");
    println!("\n🛡️    First renewal: {0: >24} \n🛡️ 🛡️  Second renewal: {1: >23}", first, second);
    println!("\n-- DONE --\n");
}