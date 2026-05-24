use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let k : usize = input.split_whitespace().next().unwrap().parse().unwrap();


    let mut n = 0;
    let mut m = 0;
    for i in 5..=k{
        if k % i == 0{
            let j = k / i;
            if j >= 5{
                n = i;
                m = j;
                break;
            }
        }
    }

    if n == 0{
        println!("-1");
        return;
    }

    let vowels = ['a','e','i','o','u'];
    let mut output = String::new();

    for r in 0..n{
        for c in 0..m{
            output.push(vowels[(r+c) % 5]);
        }
    }

    println!("{}", output);


}