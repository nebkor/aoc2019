use std::collections::HashMap;

const LOW: u32 = 156218;
const HIGH: u32 = 652527 + 1;

fn main() {
    let mut num_pwords: u32 = 0;
    for guess in LOW..HIGH {
        let s = guess.to_string();
        let digits: Vec<u32> = s.chars().map(|d| d.to_digit(10).unwrap()).collect();
        if digits.windows(2).all(|w| !(w[1] < w[0])) {
            let mut counts: HashMap<u32, u32> = HashMap::new();
            for d in digits.iter() {
                *counts.entry(*d).or_insert(0) += 1;
            }
            if counts.values().any(|v| *v == 2) {
                num_pwords += 1;
            }
        }
    }
    println!("num passwords: {}", num_pwords);
}
