use md5;
fn gen_md5(key:&str, zeros:usize) -> i32{
    let mut i:i32 = 0;
    loop {
        let skey = format!("{}{}", key, i);
        let new_seed = md5::compute(skey);
        let base_10 = format!("{:x}",new_seed);
        let prefix = &base_10[..zeros];
        
        match prefix.to_string().parse::<i32>(){
            Ok(n) => {
                if n==0{
                    return i;
                }
                i+=1;
            },
            Err(_) => i+=1
        };
    }
}


pub fn run(){
    let five_zeros = gen_md5("ckczppom", 5);
    let six_zeros = gen_md5("ckczppom", 6);
    
    println!("\n-- AoC 2015: -- Day 4: The Ideal Stocking Stuffer --");
    println!("\nFive Zeros: {} \nSix Zeroes: {}", five_zeros, six_zeros);
    println!("\n-- DONE --\n");
}