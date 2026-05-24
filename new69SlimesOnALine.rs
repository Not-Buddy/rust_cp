use std::io;

fn main(){
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let mut it = input.split_whitespace();
    let t : usize  = it.next().unwrap().parse().unwrap();
    input.clear();

    for _ in 0..t{

        io::stdin().read_line(&mut input).expect("Failed to read line");
        let mut it = input.split_whitespace();
        let _n : usize  = it.next().unwrap().parse().unwrap();
        input.clear();

        io::stdin().read_line(&mut input).expect("Failed to read line");
        let vec : Vec<u32> = input.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
        input.clear();

        let max = *vec.iter().max().unwrap();
        let min = *vec.iter().min().unwrap();

        println!("{}",(max - min + 1) / 2);

    }
    
}