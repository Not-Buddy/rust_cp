use std::io;

fn solve(num: i32) {
    if num == 1 {
        println!("1");
        return;
    }

    let mut out = String::new();
    out.push_str("1");

    let limit = 2 * num;
    for i in 0..(num - 1) {
        out.push(' ');
        out.push_str(&(limit - i).to_string());
    }

    println!("{}", out);
}

fn main() {
    let mut input = String::new();
    
    io::stdin().read_line(&mut input).unwrap();
    let n: i32 = input.split_whitespace().next().unwrap().parse().unwrap(); 
    input.clear();

    for _ in 0..n {
        io::stdin().read_line(&mut input).unwrap();
        let num: i32 = input.split_whitespace().next().unwrap().parse().unwrap(); 
        input.clear();

        solve(num);
    }
}