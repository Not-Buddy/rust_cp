use std::io;

fn main() {
    let mut input = String::new();
    
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let t: usize = input.trim().split_whitespace().next().unwrap().parse().unwrap();
    input.clear();

    for _ in 0..t{
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let mut iter = input.trim().split_whitespace();
        let n : i64 = iter.next().unwrap().parse().unwrap();
        let k : i64 = iter.next().unwrap().parse().unwrap();
        input.clear();

        if n >= k*k && (n % 2 == k % 2){
            println!("YES");
        }
        else{
            println!("NO");
        }

    }

    
}

