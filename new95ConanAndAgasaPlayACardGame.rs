use std::io;
use std::collections::HashMap;
fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let mut iter = input.split_whitespace();
    let n : usize = iter.next().unwrap().parse().unwrap();
    input.clear();

    io::stdin().read_line(&mut input).expect("Failed to read line");
    let vec : Vec<i64> = input.trim().split_whitespace()
    .map(|s| s.parse().unwrap())
    .collect();

    let mut map : HashMap<i64,i64> = HashMap::new();
    for v in &vec{
        *map.entry(*v).or_insert(0) += 1;
    }

    let mut flag = false;

    for ele in &map{
        if ele.1 % 2 == 1{
            flag = true;
            break;
        }
    }

    if flag{
        println!("Conan");
    }
    else{
        println!("Agasa");
    }
    


}