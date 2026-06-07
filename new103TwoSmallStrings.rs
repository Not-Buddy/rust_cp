use std::io;

fn main() {
    let mut input = String::new();
    
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let n: usize = input.trim().split_whitespace().next().unwrap().parse().unwrap();
    input.clear();
    
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let s = input.trim().to_string();
    input.clear();
    
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let t = input.trim().to_string();
    input.clear();

    let permutations = ["abc", "acb", "bac", "bca", "cab", "cba"];
    
    for p in permutations.iter() {
        let chars: Vec<char> = p.chars().collect();
        
        let block = format!("{}{}{}", 
            chars[0].to_string().repeat(n),
            chars[1].to_string().repeat(n),
            chars[2].to_string().repeat(n)
        );
        
        if !block.contains(&s) && !block.contains(&t) {
            println!("YES");
            println!("{}", block);
            return;
        }
        
        let alt = p.repeat(n);
        
        if !alt.contains(&s) && !alt.contains(&t) {
            println!("YES");
            println!("{}", alt);
            return; 
        }
    }
}

