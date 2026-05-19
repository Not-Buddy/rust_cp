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

    for i in 0..vec.len() {
        while vec[i] % 2 == 0{
            vec[i] /= 2;
        }
        while vec[i] % 3 == 0{
            vec[i] /= 3;
        }
    }

    let mut possible = true;

    let id0 = vec[0];
    for &id in &vec{
        if id != id0{
            possible = false;
            break;
        }
    }

    if possible {
        println!("Yes");
    }
    else {
        println!("No");
    }


}