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
        let n : i32 = iter.next().unwrap().parse().unwrap();
        let k : i32 = iter.next().unwrap().parse().unwrap();
        input.clear();

        let min_giv = (n/k)*k;
        // println!("min_giv : {}",min_giv);

        let left_candy = n - min_giv;
        // println!("left candy : {}", left_candy);

        let min_add = min(k/2, left_candy);

        let ans = min_giv + min_add;
        println!("{}",ans);


    }
}