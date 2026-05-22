use std::io;

fn main(){
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let mut temp = input.split_whitespace();
    let _n : usize = temp.next().unwrap().parse().unwrap();
    input.clear();



    io::stdin().read_line(&mut input).unwrap();
    let  dollars : i32 = input.split_whitespace()
    .map(|s| s.parse().unwrap())
    .max()
    .unwrap_or(0);

   
    println!("{}",dollars)

    
}