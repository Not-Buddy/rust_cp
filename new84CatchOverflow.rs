use std::io::{self, BufRead};
use std::cmp;

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().map(|l| l.unwrap());
    
    let _n: usize = lines.next().unwrap().trim().parse().unwrap();
    
    let mut x: i64 = 0;
    let mut stack: Vec<i64> = vec![1]; 
    
    let limit: i64 = (1i64 << 32) - 1; 
    
    for line in lines {
        let parts: Vec<&str> = line.trim().split_whitespace().collect();
        
        match parts[0] {
            "add" => {
                x += stack.last().unwrap();
                if x > limit {
                    println!("OVERFLOW!!!");
                    return;
                }
            }
            "for" => {
                let loops: i64 = parts[1].parse().unwrap();
                let current_mult = *stack.last().unwrap();
                
                let next_mult = cmp::min(current_mult.saturating_mul(loops), limit + 1);
                stack.push(next_mult);
            }
            "end" => {
                stack.pop();
            }
            _ => {}
        }
    }
    
    println!("{}", x);
}