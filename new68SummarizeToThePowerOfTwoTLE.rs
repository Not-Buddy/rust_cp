use std::io;

fn main(){
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let mut it = input.split_whitespace();
    let _n : usize  = it.next().unwrap().parse().unwrap();
    input.clear();

    io::stdin().read_line(&mut input).expect("Failed to read line");
    let vec : Vec<u32> = input.split_whitespace()
    .map(|s| s.parse().unwrap())
    .collect();
    input.clear();

    if vec.len() == 0{
        println!("0");
    }

    let mut states : Vec<bool> = vec![false;vec.len()];
    for i in 0..vec.len(){
        for j in 0..vec.len(){
            if (vec[i]+vec[j]).is_power_of_two() && i!=j {
                states[i] = true;
                states[j] = true;
            }   
        }
    }

    let mut count = 0;
    for state in states{
        if state == false{
            count += 1;
        }
    }

    println!("{}",count);
    
}