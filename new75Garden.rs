use std::io;


fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let mut iter = input.split_whitespace();
    let _n : usize = iter.next().unwrap().parse().unwrap();
    let k : usize = iter.next().unwrap().parse().unwrap();
    input.clear();

    io::stdin().read_line(&mut input).expect("Failed to read line");
    let mut vec : Vec<usize> = input.split_whitespace()
    .map(|s| s.parse().unwrap())
    .collect();

    vec.sort();
    vec.reverse();
    
    for v in vec{
        if k % v == 0{
            println!("{}",k/v);
            break;
        }
    }
}