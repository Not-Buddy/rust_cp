use std::io;


fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let mut iter = input.split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let mut _m : usize = iter.next().unwrap().parse().unwrap();
    input.clear();

    let mut awake = 0;

    for _ in 0..n{
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let vec : Vec<i32> = input.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
        input.clear();
        
        for pair in vec.chunks(2){
            if pair.contains(&1){
                awake += 1;
            }
        }

    }

    println!("{}",awake);

}