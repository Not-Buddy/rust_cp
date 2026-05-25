use std::io;

fn solve(li : &i64, ri : &i64, di : &i64 ){
    if di < li{
        println!("{}",di);
    }
    else{
        let next_multiple = (ri / di) * di + di;
        println!("{}", next_multiple);
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let mut iter = input.split_whitespace();
    let q : usize = iter.next().unwrap().parse().unwrap();
    input.clear();

    for _ in 0..q{
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let vec : Vec<i64> = input.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
        input.clear();

        let (li, ri, di) = (vec[0], vec[1], vec[2]);

        solve(&li, &ri, &di);

    }
}