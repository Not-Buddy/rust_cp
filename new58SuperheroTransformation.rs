use std::io;

fn is_vowel(c : &char) -> bool {
    matches!(c ,'a' | 'e' | 'i' | 'o' | 'u')
}

fn main(){
    let mut t = String::new();
    io::stdin().read_line(&mut t).unwrap();
    let t = t.trim();

    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    let s = s.trim();

    if s.len() != t.len(){
        println!("No");
        return
    }

    let mut possible = true;

    for (c, k) in t.chars().zip(s.chars()) {
        if is_vowel(&c) != is_vowel(&k){
            possible = false;
            break;
        }
    }

    if possible {
        println!("Yes");
    }
    else {
        println!("No");
    }

}
