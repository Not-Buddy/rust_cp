use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let mut iter = input.split_whitespace();
    let a : usize = iter.next().unwrap().parse().unwrap();
    let b : usize = iter.next().unwrap().parse().unwrap();
    input.clear();

    if a == 0 && b == 0{
        println!("NO");
    }
    else if a.abs_diff(b) <= 1{
        println!("YES");
    }
    else {
        println!("NO");
    }
}