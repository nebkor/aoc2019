fn main() {
    let mut ints: Vec<i32> = include_str!("input")
        .trim()
        .split(',')
        .map(|i| i.parse::<i32>().unwrap())
        .collect();

    ints[1] = 12;
    ints[2] = 2;
    for i in (0..ints.len()).step_by(4) {
        let op = ints[i];
        match op {
            99 => break,
            _ => {
                let x = ints[i + 1] as usize;
                let y = ints[i + 2] as usize;
                let r = ints[i + 3] as usize;
                ints[r] = match op {
                    1 => ints[x] + ints[y],
                    2 => ints[x] * ints[y],
                    _ => break,
                }
            }
        }
    }

    println!("ints[0]: {}", ints[0]);
}
