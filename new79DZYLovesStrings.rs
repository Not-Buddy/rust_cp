use std::io;
use std::collections::HashMap;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let mut iter = input.split_whitespace();
    let mut s : String = iter.next().unwrap().parse().unwrap();
    input.clear();

    io::stdin().read_line(&mut input).expect("Failed to read line");
    let k : usize = input.split_whitespace().next().unwrap().parse().unwrap();
    input.clear();

    io::stdin().read_line(&mut input).expect("Failed to read line");
    let vec : Vec<usize> = input.split_whitespace()
    .map(|s| s.parse().unwrap())
    .collect();

    let mut map : HashMap<char, usize> = HashMap::new();

    let mut ch_byte = b'a';
    for v in vec{
        map.insert(ch_byte as char, v);
        ch_byte += 1;
    }
    
    let (&max_char, &max_weight) = map.iter().max_by_key(|&(_, weight)| weight).unwrap();

    for _ in 0..k{
        s.push(max_char);
    }

    let mut score = 0;

    for (i, c) in s.chars().enumerate(){
        let index = i+1;
        let weight = map[&c];
        score += index * weight;
    }

    println!("{}", score);

}