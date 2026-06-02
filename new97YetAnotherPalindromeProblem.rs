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
        let n : usize = iter.next().unwrap().parse().unwrap();
        input.clear();

        io::stdin().read_line(&mut input).expect("Failed to read line");
        let vec : Vec<char> = input.trim().split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
        input.clear();

        let mut found = false;

        for i in 3..=n{
            for j in 0..=(n-i){
                let slice = &vec[j..(j + i)];
                if slice.iter().eq(slice.iter().rev()){
                    found = true;
                    break;
                }
            }
            if found{
                break;
            }
        }
        if found{
            println!("YES");
        }
        else {
            println!("NO");
        }
    }

}