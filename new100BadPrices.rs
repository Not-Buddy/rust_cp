use std::io;

fn main(){
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let t : usize  = input.trim().split_whitespace().next().unwrap().parse().unwrap();
    input.clear();

    for _ in 0..t{

        io::stdin().read_line(&mut input).expect("Failed to read line");
        let _n : usize  = input.trim().split_whitespace().next().unwrap().parse().unwrap();
        input.clear();

        io::stdin().read_line(&mut input).expect("Failed to read line");
        let vec : Vec<i32> = input.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

        let mut bad_days = 0;
        let mut min_yet = std::i32::MAX;

        for &v in vec.iter().rev(){
            
            if v > min_yet {
                bad_days += 1;
            }
            else {
                min_yet = v;
            }
        }
        
        println!("{}",bad_days);
        
    }

}
