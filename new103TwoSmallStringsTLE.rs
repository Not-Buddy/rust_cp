use std::io;

fn main() {
    let mut input = String::new();
    
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let n: usize = input.trim().split_whitespace().next().unwrap().parse().unwrap();
    input.clear();
    
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let s: Vec<char> = input.trim().chars().collect();
    input.clear();
    
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let t: Vec<char> = input.trim().chars().collect();
    input.clear();

    let mut chars: Vec<char> = "abc".repeat(n).chars().collect();

    let mut flag = true;
    
    while flag {

        flag = false;
        
        for i in 0..(chars.len() - 1) {
            let pair = [chars[i], chars[i + 1]];
            
            if pair == s[..] || pair == t[..] {
                chars.swap(i, i + 1);
                flag = true;
            }
        }
    }

    let output: String = chars.into_iter().collect();
    println!("YES");
    println!("{}", output);
}

