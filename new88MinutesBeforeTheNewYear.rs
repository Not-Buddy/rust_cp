use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let mut iter = input.split_whitespace();
    let t : usize = iter.next().unwrap().parse().unwrap();
    input.clear();

    for _ in 0..t{
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let mut iter = input.split_whitespace();
        let h : usize = iter.next().unwrap().parse().unwrap();
        let m : usize = iter.next().unwrap().parse().unwrap();
        input.clear();

        let hours_left = 24 - h - 1;
        println!("{}", (hours_left * 60) + 60 - m);
    }
}