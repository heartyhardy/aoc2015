
fn gen_next_seq(seq:Vec<usize>, steps:i32) ->Vec<usize>{    
    if steps <= 0 {
        return seq.clone();
    }

    let mut next:Vec<usize> = Vec::new();
    let mut i = 0;

    while i < seq.len() {
        let mut n:usize = 1;
        for j in i..seq.len()-1{
            if seq[j] == seq[j+1]{
                n+=1;
                continue;
            }
            break;
        }
        let v = seq[i];
        next.push(n);
        next.push(v);
        i=i+ n;
    }
    return gen_next_seq(next, steps-1);
}

pub fn run(){
   let start="1113122113";
   let seq:Vec<usize> = start
                        .chars()
                        .map(|c|c.to_string())
                        .map(|s|s.parse::<usize>().unwrap())
                        .collect();
    let run40 = gen_next_seq(seq, 40);
    let run40_len = run40.len();
    let run50 = gen_next_seq(run40, 10);
    
    println!("\n-- AoC 2015: -- Day 10: Elves Look, Elves Say --");
    println!("\n After 4️⃣ 0️⃣  iterations: {0: >10} \n After 5️⃣ 0️⃣  iterations: {1: >10}", run40_len, run50.len());
    println!("\n-- DONE --\n");
                        
}