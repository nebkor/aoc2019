fn get_input() -> [Vec<String>; 2] {
    let wires: Vec<_> = include_str!("input").trim().split('\n').collect();
    let w1 = wires[0].split(',').map(|x| x.trim().to_owned()).collect();
    let w2 = wires[1].split(',').map(|x| x.trim().to_owned()).collect();
    [w1, w2]
}

fn main() {
    let input = get_input();
    dbg!(input);
}
