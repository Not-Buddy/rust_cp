use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let mut iter = input.split_whitespace();
    let x : usize = iter.next().unwrap().parse().unwrap();
    input.clear();

    let mut flag = false;

    let mut a = 0;
    let mut b = 0;

    for i in 1..=x{
        for j in 1..=x{
            if i % j == 0 && (i*j) > x && (i/j) < x{
                a = i;
                b = j;
                flag = true;
                break;
            }
        }
    }

    if flag{
        println!("{} {}",a,b);
    }
    else {
        println!("-1");
    }

}