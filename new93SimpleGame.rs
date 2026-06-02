use std::io;

fn main(){
    let mut input : String = String::new();
    io::stdin().read_line(&mut input).expect("Error reading line");
    let vec : Vec<i64> = input.trim().split_whitespace()
    .map(|s| s.parse().unwrap())
    .collect();
    input.clear();

    let (n, m) = (vec[0],vec[1]);

    let left = m;
    let right = n-m;

    let ans;
    if left < right{
        ans = m + 1;
    }
    else{
        ans = m - 1;
    }

    println!("{}",ans);

}