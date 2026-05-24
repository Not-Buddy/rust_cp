use std::io;

fn main(){
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let s = input.trim().as_bytes();
    
    let mut count = 0;
    let mut current_sum = 0;
    let mut current_len = 0;

    for &byte in s{
        let digit = (byte - b'0') as u32;
        current_sum += digit;
        current_len += 1;

        if digit % 3 == 0 || current_sum % 3 == 0 || current_len == 3{
            count +=1;
            current_sum = 0;
            current_len = 0;
        }
    }

    println!("{}",count);
    
}