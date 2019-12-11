fn main() {
    let ints_og: Vec<i32> = include_str!("input")
        .trim()
        .split(',')
        .map(|i| i.parse::<i32>().unwrap())
        .collect();

    let mut ints = ints_og.clone();

    'outer: for noun in 0..100 {
        for verb in 0..100 {
            ints[1] = noun;
            ints[2] = verb;
            // later versions will require us to actually keep track of a program counter,
            // but for now, this is fine.
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
            if ints[0] == 19690720 {
                println!("100 * noun + verb = {}", 100 * noun + verb);
                break 'outer;
            } else {
                ints = ints_og.clone();
            }
        }
    }

    println!("ints[0]: {}", ints[0]);
}
