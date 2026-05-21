use std::io;

fn solve(num: usize, vec: &Vec<i32>) {
    let mut max_diff = 0;
    
    for i in 0..num - 1 {
        if vec[i] > vec[i+1] {
            max_diff = max_diff.max(vec[i] - vec[i+1]);
        }
    }

    if max_diff == 0 {
        println!("YES");
        return;
    }

    let k = max_diff;
    
    let mut can_0 = true;
    let mut can_1 = true;

    for i in 1..num {
        let prev = vec[i-1];
        let curr = vec[i];

        let new_0 = (can_0 && prev <= curr) || (can_1 && curr - prev >= k);

        let new_1 = can_0 || (can_1 && prev <= curr);

        can_0 = new_0;
        can_1 = new_1;
    }

    if can_0 || can_1 {
        println!("YES");
    } else {
        println!("NO");
    }
}

fn main() {
    let mut input = String::new();
    
    io::stdin().read_line(&mut input).unwrap();
    let n: i32 = input.split_whitespace().next().unwrap().parse().unwrap(); 
    input.clear();

    for _ in 0..n {
        io::stdin().read_line(&mut input).unwrap();
        let num: i32 = input.split_whitespace().next().unwrap().parse().unwrap(); 
        input.clear();

        io::stdin().read_line(&mut input).unwrap();

        let mut vec : Vec<i32> = input.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

        input.clear();

        solve(num as usize, &vec);
        
    }
}