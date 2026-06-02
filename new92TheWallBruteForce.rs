use std::io;

fn main(){
    let mut input : String = String::new();
    io::stdin().read_line(&mut input).expect("Error reading line");
    let mut vec : Vec<i64> = input.trim().split_whitespace()
    .map(|s| s.parse().unwrap())
    .collect();
    input.clear();

    let (mut x,mut y,mut a,mut b) = (vec[0],vec[1],vec[2],vec[3]);

    let mut red : Vec<i64> = Vec::new();
    let mut pink : Vec<i64> = Vec::new();

    for i in a..=b{
        if i % x == 0{
            red.push(i);
        }
        if i % y == 0{
            pink.push(i);
        }
    }

    let mut ans = 0;

    for &block in &red{
        if pink.contains(&block){
            ans += 1;
        }
    }

    println!("{}",ans);

}