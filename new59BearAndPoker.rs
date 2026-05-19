use std::io;

fn main(){
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let _n : i32 = input.split_whitespace().next().unwrap().parse().unwrap(); 
    input.clear();

    io::stdin().read_line(&mut input).unwrap();
    let mut vec : Vec<i32> = input.split_whitespace()
    .map(|s| s.parse().unwrap())
    .collect();

    vec.sort();

    let maximum = *vec.iter().max().unwrap();

    let mut possible = true;

    for i in &vec {
        if *i != maximum{
            if maximum % i == 0{
                continue;
            }
            else {
                possible = false;
                break;
            }
        }
    }

    if possible {
        println!("Yes");
    }
    else {
        println!("No");
    }


}