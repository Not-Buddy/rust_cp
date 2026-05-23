use std::io;

fn main(){
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let mut it = input.split_whitespace();
    let _n : usize  = it.next().unwrap().parse().unwrap();
    input.clear();

    io::stdin().read_line(&mut input).expect("Failed to read line");
    let mut vec : Vec<i32> = input.split_whitespace()
    .map(|s| s.parse().unwrap())
    .collect();
    input.clear();

    vec.sort();
    vec.dedup(); 

    match vec.len() {
        1 => println!("0"),
        2 => {
            let diff = vec[1] - vec[0];
            if diff % 2 == 0 {
                println!("{}", diff / 2);
            } else {
                println!("{}", diff);
            }
        }
        3 => {
            let diff1 = vec[1] - vec[0];
            let diff2 = vec[2] - vec[1];
            if diff1 == diff2 {
                println!("{}", diff1);
            } else {
                println!("-1");
            }
        }
        _ => println!("-1"),
    }

    
}