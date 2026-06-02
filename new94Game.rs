use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let mut iter = input.split_whitespace();
    let n : usize = iter.next().unwrap().parse().unwrap();
    input.clear();

    if n % 2 == 1{
        println!("1");
    }
    else {
        println!("2");
    }
    
}