use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let k : usize = input.split_whitespace().next().unwrap().parse().unwrap();


    let mut flag = false;
    let mut n = 0;
    let mut m = 0;
    for i in 5..=k{
        for j in 5..=6{
            if  i * j == k{
                n = i;
                m = j;
                flag = true;
                break;
            }
        }
    }

    if flag{
        let mut vec : Vec<char> = vec!['a','e','i','o','u'];
        let mut output : Vec<Vec<char>> = Vec::new();

        for _ in 0..n{
            
            let mut temp : Vec<char> = Vec::new();
            for j in 0..m{
                temp.push(vec[j % 5]);
            }
            vec.rotate_left(1);
            output.push(temp);
        }

        for i in output{
            for j in i{
                print!("{}",j);
             }
        }        
    }
    else {
        println!("-1");
    }



}