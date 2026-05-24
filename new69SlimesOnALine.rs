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
        let n : usize  = it.next().unwrap().parse().unwrap();
        input.clear();

        io::stdin().read_line(&mut input).expect("Failed to read line");
        let vec : Vec<u32> = input.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
        input.clear();

        
        let mut temp : Vec<u32> = vec.clone();
        temp.dedup();

        if temp.len() == 1{
            println!("0");
            continue;
        }

        let count = vec.iter().max().unwrap();

        println!("{}",count/2);

    }
    
}