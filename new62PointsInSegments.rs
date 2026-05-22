use std::io;

fn main(){
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let mut temp = input.split_whitespace();
    
    let n : usize = temp.next().unwrap().parse().unwrap();
    let m : usize = temp.next().unwrap().parse().unwrap();

    input.clear();



    let mut vec : Vec<bool> = vec![false; m+1];
    
    for _ in 0..n{
        io::stdin().read_line(&mut input).unwrap();
        let mut temp = input.split_whitespace();
        let mut l : usize = temp.next().unwrap().parse().unwrap();
        let mut r : usize = temp.next().unwrap().parse().unwrap();
        input.clear();

        for ii in l..=r{
            vec[ii] = true;
        }
    }

    let mut result = Vec::new();
    for i in 1..=m{
        if !vec[i]{
            result.push(i);
        }
    }

    println!("{}",result.len());

    let mut strung = String::new();

    for (i, &point) in result.iter().enumerate(){
        if i>0 {
            strung.push(' ');
        }
        strung.push_str(&point.to_string());
    }

    if !result.is_empty(){
        println!("{}", strung)
    }
    else {
        println!();
    }
}