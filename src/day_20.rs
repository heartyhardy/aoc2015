
use std::f64;
use std::thread;


const MAX_GIFTS:i64 = 33100000;

fn get_divisors(n:i64) -> Vec<i64>{
    let mut i:i64 =1;
    let upper_bound = (n as f64).sqrt() as i64;
    let mut divisors:Vec<i64> = Vec::new();
    while i <= upper_bound {
        if n % i == 0{
            if n/i == i{
                divisors.push(i);
            }else{
                divisors.push(i);
                divisors.push(n/i);
            }
        }
        i+=1;
    }
    divisors.sort_by(|a,b| b.cmp(&a));
    return divisors;
}

fn get_gift_count_unlimited(divisors:Vec<i64>, gph:i64)->i64{
    let mut count =0;
    for d in 0..divisors.len(){
        count+= (divisors[d] as i64) * gph;
    }
    return count;
}

fn get_gift_count_limited(divisors:Vec<i64>, gph:i64,n:i64, maxn:i64)->i64{
    let mut count =0;
    for d in 0..divisors.len(){
        if divisors[d] * maxn < n{
            break;
        }
        count+= (divisors[d] as i64) * gph;
    }
    return count;
}

fn gifts_away_unlimited(gph:i64){
    let mut i:i64 =MAX_GIFTS/1000;
    loop{
        let divisors = get_divisors(i);
        let gift_count = get_gift_count_unlimited(divisors,10);

        if gift_count >= MAX_GIFTS{
            println!("\nPart I : House No: {}\n", i);
            break;
        }
        i+=1;
    }
}

fn gifts_away_limited(gph:i64){
    let mut i:i64 =MAX_GIFTS/1000;
    loop{
        let divisors = get_divisors(i);
        let gift_count = get_gift_count_limited(divisors, 11, i, 50);

        if gift_count >= MAX_GIFTS{
            println!("\nPart II - Limited: House No {}\n", i);
            break;
        }
        i+=1;
    }
}


pub fn run(){
    let part_i = thread::spawn(||{
        gifts_away_unlimited(10);
    });
    let part_ii = thread::spawn(||{
        gifts_away_limited(11);
    });
    part_i.join().unwrap();
    part_ii.join().unwrap();
}