use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let mut iter = input.split_whitespace();
    let n : usize = iter.next().unwrap().parse().unwrap();
    let _of : String = iter.next().unwrap().parse().unwrap();
    let m_or_w : String = iter.next().unwrap().parse().unwrap();
    input.clear();
    
    if m_or_w == "week"{
        if n == 5 || n == 6 {
            println!("53");
        }
        else{
            println!("52");
        }
    }
    else{
        match n{
            31 => println!("7"),
            30 => println!("11"),
            _ => println!("12"),
        }
    }

}