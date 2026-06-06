use std::io;
use std::cmp::min;

fn main(){
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let n : usize  = input.trim().split_whitespace().next().unwrap().parse().unwrap();
    input.clear();

        io::stdin().read_line(&mut input).expect("Failed to read line");
        let vec : Vec<i32> = input.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let mut count = 0;

    for v in &vec{
        if v % 2 != 0{
            count += 1;
        }
    }
    
    let eve_count = n - count;

    println!("{}", min(eve_count, count));

}
