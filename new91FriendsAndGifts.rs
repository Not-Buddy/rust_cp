use std::io;

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

    let mut present : Vec<bool> = vec![false ; n + 1];

    for i in 0..n{
        if vec[i] == 0{
            missing_idx.push(i);
        }else{
            present[vec[i]] = true;
        }
    }
    
    let mut gifts : Vec<usize> = Vec::new();
    for i in 1..=n{
        if !present[i]{
            gifts.push(i);
        }
    }

    for i in 0..missing_idx.len(){
        vec[missing_idx[i]] = gifts[i];
    }

    for i in 0..missing_idx.len(){
        let id = missing_idx[i];

        if vec[id] == id + 1{
            let next_idx = missing_idx[(i+1) % missing_idx.len()];
            vec.swap(id,next_idx);
        }
    }

    for i in vec{
        print!("{} ",i);
    }

}