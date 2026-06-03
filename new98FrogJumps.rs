
use std::io;
use std::cmp::max;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let mut iter = input.split_whitespace();
    let t : usize = iter.next().unwrap().parse().unwrap();
    input.clear();


    for _ in 0..t{

        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let s = input.trim();

        let mut max_l = 0;
        let mut cur_l = 0;

        for c in s.chars(){
            if c == 'L'{
                cur_l += 1;
                max_l = max(max_l, cur_l);
            }
            else{
                cur_l = 0;
            }
        }

        println!("{}", max_l + 1);

    }

}