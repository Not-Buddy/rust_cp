use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let q: usize = input.trim().split_whitespace().next().unwrap().parse().unwrap();
    input.clear();

    for _ in 0..q {
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let mut iter = input.trim().split_whitespace();
        let n: i64 = iter.next().unwrap().parse().unwrap();
        let m: i64 = iter.next().unwrap().parse().unwrap();
        input.clear();

        let total_multiples = n / m;
        
        let full_cycles = total_multiples / 10;
        let remainder = total_multiples % 10;

        let mut cycle_sum: i64 = 0;
        for i in 1..=10 {
            cycle_sum += (m * i) % 10;
        }

        let mut final_answer = full_cycles * cycle_sum;

        for i in 1..=remainder {
            final_answer += (m * i) % 10;
        }

        println!("{}", final_answer);
    }
}

