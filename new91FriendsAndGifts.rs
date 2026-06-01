use std::io;
use std::cmp::{max,min};

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let mut iter = input.split_whitespace();
    let n : usize = iter.next().unwrap().parse().unwrap();
    input.clear();

    io::stdin().read_line(&mut input).expect("Failed to read line");
    let mut vec : Vec<usize> = input.split_whitespace()
    .map(|s| s.parse().unwrap())
    .collect();
    input.clear();
    
    let mut missing_idx : Vec<usize> = Vec::new(); 

    // println!("OG arrangement {:?}",vec);

    for i in 0..n{
        if vec[i] == 0{
            missing_idx.push(i);
        }
    }

    // println!("Missing Indexes {:?}", missing_idx);

    let mut gifts : Vec<usize> = Vec::new();
    
    for i in 1..=n{
        if vec.contains(&i) {
            continue;
        }
        else{
            gifts.push(i);
        }
    }

    // println!("remaining gifts {:?}", gifts);

    for id in missing_idx{
        let temp = * gifts.last().unwrap();
        gifts.pop();
        if id != temp{
            vec[id] = temp;
        }
    }

    // println!("ans {:?}", vec);

    for i in vec{
        print!("{} ", i);
    }

}