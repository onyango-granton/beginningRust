use std::collections::HashMap;

fn contains_all_alphabets(s : &str) -> bool {
    let s_vector = &s.chars().collect::<Vec<char>>();
    let mut s_hash : HashMap<char, i32> = HashMap::new();

    for ch in s_vector {
        if ch.is_ascii_alphabetic(){
            *s_hash.entry(ch.to_ascii_lowercase()).or_insert(0) += 1;
        }
    }

    s_hash.len() == 26
}

fn main() {
    let s = String::from("The quick brown fox jumps over the lazy dog");
    println!("{:?} {:?} contains each alphabetical letter", contains_all_alphabets(&s), &s);
}