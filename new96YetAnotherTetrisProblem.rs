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
        let _n : usize = iter.next().unwrap().parse().unwrap();
        input.clear();

        io::stdin().read_line(&mut input).expect("Failed to read line");
        let vec : Vec<i64> = input.trim().split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
        input.clear();

        let max_ele = vec.iter().max().unwrap();
        let mut flag = true;
        for v in &vec{
            let temp = max_ele - *v;
            // println!("temp : {}",temp);
            if temp % 2 == 0{
                continue
            }
            else {
                flag = false;
                break;
            }
        }

        if flag{
            println!("YES");
        }
        else {
            println!("NO");
        }


    }

}