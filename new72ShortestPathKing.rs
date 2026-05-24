use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let s1 = input.trim().as_bytes();
    let mut x1 : i32 = (s1[0] - b'a') as i32;
    let mut y1 : i32 = (s1[1] - b'1') as i32;
    input.clear();

    io::stdin().read_line(&mut input).expect("Failed to read line");
    let s2 = input.trim().as_bytes();
    let x2 : i32 = (s2[0] - b'a') as i32;
    let y2 : i32 = (s2[1] - b'1') as i32;
    input.clear();
    
    let mut moves: Vec<String> = Vec::new();

    while x1 != x2 || y1 != y2{
        let mut c_move = String::new();

        if x1 < x2{
            c_move.push('R');
            x1+=1;
        } 
        else if x1 > x2{
            c_move.push('L');
            x1-=1;
        }

        if y1<y2{
            c_move.push('U');
            y1+=1;
        }
        else if y1>y2{
            c_move.push('D');
            y1-=1;
        }

        moves.push(c_move); 
    }
    println!("{}", moves.len());
    for m in moves {
        println!("{}", m);
    }

}