use std::io;


fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let mut iter = input.split_whitespace();
    let t : usize = iter.next().unwrap().parse().unwrap();
    input.clear();


    for _ in 0..t{
        
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let vec : Vec<usize> = input.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
        input.clear();

        let (n,k) = (vec[0], vec[1]);

        let mut output : String = String::with_capacity(n);

        for i in 0..n{
            let ch = ((i % k) as u8 + b'a') as char;
            output.push(ch);
        }
        
        
        println!("{}", output);
    }
}