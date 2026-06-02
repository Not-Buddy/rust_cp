use std::io;

fn main(){
    let mut input : String = String::new();
    io::stdin().read_line(&mut input).expect("Error reading line");
    let vec : Vec<i64> = input.trim().split_whitespace()
    .map(|s| s.parse().unwrap())
    .collect();
    input.clear();

    let ( x, y, a, b) = (vec[0],vec[1],vec[2],vec[3]);

    let mut ans = 0;

    for i in a..=b{
        if i % x == 0 && i % y == 0{
            ans += 1;
        }

    }

    println!("{}",ans);

}