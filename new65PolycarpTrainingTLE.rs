use std::io;

fn main(){
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let mut it = input.split_whitespace();
    let n : usize  = it.next().unwrap().parse().unwrap();
    input.clear();

    io::stdin().read_line(&mut input).expect("Failed to read line");
    let mut vec : Vec<usize> = input.split_whitespace()
    .map(|s| s.parse().unwrap())
    .collect();

    vec.sort();
    
    let mut days_solved = 0;

    for i in 0..n{
        let target = i + 1;
        if let Some(index) = vec.iter().position(|&x| x >= target){
            vec.remove(index);
            days_solved += 1;
        }
    }

    println!("{}", days_solved);

}