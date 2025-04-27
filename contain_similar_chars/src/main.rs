use std::collections::HashMap;

fn contains_similar_chars(s1: &str, s2: &str) -> bool {
    let mut s1_hash :HashMap<char, i32>= HashMap::new();

    for ch in s1.chars(){
        *s1_hash.entry(ch).or_insert(0) += 1;
    }

    for ch in s2.chars() {
        if !(s1_hash.contains_key(&ch)){
            return false;
        }
    }
    
    true
}

fn main() {
    println!("{:?}", contains_similar_chars("listen", "silent"));
}