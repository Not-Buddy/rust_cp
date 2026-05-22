//n,x,y

use std::{any::type_name_of_val, io};

fn main(){
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let mut it = input.split_whitespace();
    let n : usize  = it.next().unwrap().parse().unwrap();
    let x : usize = it.next().unwrap().parse().unwrap();
    let y : usize = it.next().unwrap().parse().unwrap();
    input.clear();

    io::stdin().read_line(&mut input).expect("Failed to read line");
    let s = input.trim().as_bytes();

    // println!("{}", type_name_of_val(&s));


    let mut operations = 0;

    for i in (n-x)..n {
        if i == n - 1 - y{
            if s[i] != b'1'{
                operations += 1;
            }
        }
        else {
            if s[i] != b'0'{
                operations += 1;
            }
        }
    }
    println!("{}", operations);


    // println!("{} {}",b'0',b'1');
}