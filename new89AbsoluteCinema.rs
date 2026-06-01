use std::io;
use std::cmp::{max,min};

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let mut iter = input.split_whitespace();
    let t : usize = iter.next().unwrap().parse().unwrap();
    input.clear();

    for _ in 0..t{

        io::stdin().read_line(&mut input).expect("Failed to read line");
        let mut iter = input.split_whitespace();
        let n : usize = iter.next().unwrap().parse().unwrap();
        input.clear();

        io::stdin().read_line(&mut input).expect("Failed to read line");
        let a : Vec<i64> = input.trim().split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
        input.clear();

        io::stdin().read_line(&mut input).expect("Failed to read line");
        let  b : Vec<i64> = input.trim().split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
        input.clear();

        let mut sum : i64 = 0;

        for i in 0..n{
            sum += max(a[i], b[i]);
        }

        let mut ans : i64 = 0;

        for i in 0..n{
            ans = max(ans, sum + min(a[i] , b[i]));
        }

        println!("{}",ans);
    }
}