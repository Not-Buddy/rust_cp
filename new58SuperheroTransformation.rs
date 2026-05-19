use std::io;

fn count_vowels(text: &str) -> i32 {

    let mut count : i32 = 0;
    for c in text.chars(){
        match c.to_ascii_lowercase() {
            'a' | 'e' | 'i' | 'o' | 'u' => count += 1,
            _ => (),
        }
    }
    count
}

fn count_consonants(text : &str) -> i32 {

    let mut count : i32 = 0;
    for c in text.chars() {
        let lower_c = c.to_ascii_lowercase();
        if lower_c.is_ascii_alphabetic() && !matches!(lower_c, 'a' | 'e' | 'i' | 'o' | 'u'){
            count += 1;
        }
    }
    count

}
fn main(){
    let mut s = String::new();
    io::stdin().read_line(&mut s).expect("Unable to read line");

    let mut t = String::new();
    io::stdin().read_line(&mut t).expect("Unable to read line2");

    let vowels1 = count_vowels(&s);
    let vowels2 = count_vowels(&t);

    let consonants1 = count_consonants(&s);
    let consonants2 = count_consonants(&t);

    if vowels1 == vowels2 && consonants1 == consonants2{
        println!("Yes");
    }
    else {
        println!("No");
    }

}