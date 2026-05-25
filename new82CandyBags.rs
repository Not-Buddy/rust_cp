use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let mut iter = input.split_whitespace();
    let n : usize = iter.next().unwrap().parse().unwrap();
    input.clear();

    let mut left = 1;
    let mut right = n*n;

    for _ in 0..n{
        for _ in 0..(n/2){
            println!("{} {}", left, right);
            left += 1;
            right -= 1;
        }
    }


}