use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let mut iter = input.split_whitespace();
    let t : usize = iter.next().unwrap().parse().unwrap();
    input.clear();

    for _ in 0..t{
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let mut iter = input.split_whitespace();
        let mut n : i64 = iter.next().unwrap().parse().unwrap();
        let k : i64 = iter.next().unwrap().parse().unwrap();
        input.clear();

        let mut count = 0;
        while n != 0{
            let div = n % k;
            if div == 0{
                n = n / k;
                count += 1;
            }
            else {
                // n -= 1;
                // count += 1;
                let div = n % k;
                count += div;
                n -= div;
            }
        }
        println!("{}",count);
    }
}