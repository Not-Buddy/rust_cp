use std::io;
use std::collections::HashMap;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let mut iter = input.split_whitespace();
    let t : usize = iter.next().unwrap().parse().unwrap();
    input.clear();


    for _ in 0..t{

        io::stdin().read_line(&mut input).expect("Failed to read line");
        let mut iter = input.split_whitespace();
        let _n : usize = iter.next().unwrap().parse().unwrap();
        input.clear();

        io::stdin().read_line(&mut input).expect("Failed to read line");
        let vec : Vec<i64> = input.trim().split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
        input.clear();

        let mut f_idx : HashMap<i64, usize> = HashMap::new();
        let mut found = false;

        for (i, &val) in vec.iter().enumerate(){

            if let Some(&idx) = f_idx.get(&val){
                if i - idx >= 2{
                    found = true;
                    break;
                }
            }
            else {
                f_idx.insert(val ,i);
            }

        }

        if found{
            println!("YES");
        }
        else {
            println!("NO");
        }
    }

}