use std::io;
use std::collections::HashMap;

fn main(){
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let mut it = input.split_whitespace();
    let _n : usize  = it.next().unwrap().parse().unwrap();
    input.clear();

    io::stdin().read_line(&mut input).expect("Failed to read line");
    let vec : Vec<i64> = input.split_whitespace()
    .map(|s| s.parse().unwrap())
    .collect();
    input.clear();

    if vec.len() == 0{
        println!("0");
        return;
    }

    let mut freq : HashMap<i64 , usize> = HashMap::new();
    for &num in &vec {
        *freq.entry(num).or_insert(0) += 1;
    }

    let mut bad = 0;

    for &num in &vec {
        let mut flag = false;

        for p in 0..=30 {
            let target = (1 << p) - num;

            if let Some(&count) = freq.get(&target){
                if target == num && count > 1{
                    flag = true;
                    break;
                }
                else if target != num && count > 0{
                    flag = true;
                    break;
                }
            }
        }

        if !flag{
            bad += 1;
        }
    }
    

    println!("{}",bad);
    
}