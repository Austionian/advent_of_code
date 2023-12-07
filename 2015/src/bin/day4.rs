fn part_one() -> usize {
    let mut res = 0;
    loop {
        let digest = md5::compute(format!("bgvyzdsv{res}"));
        if &digest[0..2] == &[0x00, 0x00] && &digest[2] <= &0x0F {
            break;
        }
        res += 1;
    }
    res
}

fn part_two() -> usize {
    let mut res = 0;
    loop {
        let digest = md5::compute(format!("bgvyzdsv{res}"));
        if &digest[0..3] == &[0x00, 0x00, 0x00] {
            break;
        }
        res += 1;
    }
    res
}

fn main() {
    println!("Part One: {}", part_one());
    println!("Part Two: {}", part_two());
}
