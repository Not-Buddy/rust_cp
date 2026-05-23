use std::io;

fn main(){
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let mut it = input.split_whitespace();
    let _n : usize  = it.next().unwrap().parse().unwrap();
    input.clear();

    io::stdin().read_line(&mut input).expect("Failed to read line");
    let s : String = input.trim().to_string().clone();
    input.clear();

    let vec : Vec<char> = s.chars().collect();
    let mut good_string : Vec<char> = Vec::new();

    for &ch in &vec{
        if good_string.len() % 2 != 0 && *good_string.last().unwrap() == ch {
            continue;
        } 
        good_string.push(ch);
    }

    if good_string.len() % 2 != 0{
        good_string.pop();
    }

    let defects = vec.len() - good_string.len();

    println!("{}",defects);

    let out_s : String = good_string.into_iter().collect();

    println!("{}", out_s);
}