
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

    let mut c : Vec<i64> = vec![0 ; n];
    for i in 0..n{
        c[i] = a[i] - b[i];
    }
    c.sort_unstable();

    let mut good_topics : i64 = 0;
    let mut left = 0;
    let mut right = n - 1;

    while left < right{
        if c[left] + c[right] > 0{
            good_topics += (right - left) as i64;
            right -= 1;
        }
        else{
            left += 1;
        }
    }

    println!("{}",good_topics);
}