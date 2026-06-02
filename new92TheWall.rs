use std::io;

fn gcd(mut a : i64, mut b : i64) -> i64{
    while b != 0{
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

fn lcm(a : i64, b : i64) -> i64{
    (a / gcd(a,b)) * b
}

fn main(){
    let mut input : String = String::new();
    io::stdin().read_line(&mut input).expect("Error reading line");
    let vec : Vec<i64> = input.trim().split_whitespace()
    .map(|s| s.parse().unwrap())
    .collect();
    input.clear();

    let (x, y, a, b) = (vec[0],vec[1],vec[2],vec[3]);

    let l = lcm(x,y);

    let ans = (b/l)-((a-1)/l);
    
    println!("{}",ans);

}