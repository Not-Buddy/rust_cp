use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let mut vec : Vec<i32> = input.trim().split('+')
    .map(|s| s.parse().unwrap())
    .collect();

    input.clear();

    vec.sort();
    for i in 0..vec.len(){
        if i == vec.len()-1{
            print!("{}",vec[i]);
        }
        else {
            print!("{}+",vec[i]);
        }
    }
}