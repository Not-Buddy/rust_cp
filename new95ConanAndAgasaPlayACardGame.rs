use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let mut iter = input.split_whitespace();
    let n : usize = iter.next().unwrap().parse().unwrap();
    input.clear();

    io::stdin().read_line(&mut input).expect("Failed to read line");
    let vec : Vec<i64> = input.trim().split_whitespace()
    .map(|s| s.parse().unwrap())
    .collect();

    let highest_idx = vec.len();

    let max_ele = vec.iter().max().unwrap();



    // println!("highest_idx : {} max_ele : {}",highest_idx,max_ele);

    if *max_ele > highest_idx as i64{
        println!("Conan");
    }
    else {
        println!("Agasa");
    }
    
}