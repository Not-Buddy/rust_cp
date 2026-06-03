
use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let mut iter = input.split_whitespace();
    let n : usize = iter.next().unwrap().parse().unwrap();
    input.clear();

    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let a : Vec<i64> = input.trim().split_whitespace()
    .map(|s| s.parse().unwrap())
    .collect();

    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let b : Vec<i64> = input.trim().split_whitespace()
    .map(|s| s.parse().unwrap())
    .collect();

    let mut good_topic = 0;

    for i in 0..n{
        for j in 0..n{
            if i != j && (a[i] + a[j] > b[i] + b[j]){
                good_topic += 1;
            }
        }
    }

    println!("{}",good_topic/2);
}