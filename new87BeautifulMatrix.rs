use std::io;

fn main() {
    let mut input = String::new();
    input.clear();

    let mut matrix : Vec<Vec<i32>> = Vec::new();

    for _ in 0..5{
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let vec : Vec<i32> = input.trim().split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
        input.clear();
        matrix.push(vec);
    }

    let mut x = 0;
    let mut y = 0;

    for i in 0..5{
        for j in 0..5{
            if matrix[i][j] == 1{
                x = i;
                y = j;
                break;
            }
        }
    }

    let ans = x.abs_diff(2) + y.abs_diff(2);
    println!("{}",ans);

}