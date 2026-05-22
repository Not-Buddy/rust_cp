use std::io;

fn main(){
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let mut temp = input.split_whitespace();
    let _n : usize = temp.next().unwrap().parse().unwrap();
    input.clear();



    io::stdin().read_line(&mut input).unwrap();
    let  vec : Vec<i32> = input.split_whitespace()
    .map(|s| s.parse().unwrap())
    .collect();

    let mut energy = 0;
    let mut dollars = 0;
    let mut current_height = 0;

    for i in 0..vec.len(){
        let next_height = vec[i];

        energy += current_height - next_height;

        if energy < 0{
            dollars += -energy;
            energy = 0;
        }

        current_height = next_height;
    }

    println!("{}",dollars)

    
}