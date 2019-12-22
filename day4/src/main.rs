const LOW: u32 = 156218;
const HIGH: u32 = 652527 + 1;

fn main() {
    let mut num_pwords: u32 = 0;
    for guess in LOW..HIGH {
        let s = guess.to_string();
        let digits: Vec<u32> = s.chars().map(|d| d.to_digit(10).unwrap()).collect();
        if digits.windows(2).all(|w| !(w[1] < w[0])) && digits.windows(2).any(|w| w[0] == w[1]) {
            num_pwords += 1;
        }
    }
    println!("num passwords: {}", num_pwords);
}
