use aoc::timing;
use num_bigint::BigInt;

fn fib_bottom_up(n: usize) -> BigInt {
    if n == 1 || n == 2 {
        return 1.into();
    }
    let mut bottom_up: Vec<BigInt> = vec![0.into(); n + 1];
    bottom_up[1] = 1.into();
    bottom_up[2] = 1.into();
    for i in 3..n + 1 {
        bottom_up[i] = bottom_up[i - 2].clone() + bottom_up[i - 1].clone();
    }
    bottom_up[n].clone()
}

fn main() {
    timing!(println!("{}", fib_bottom_up(533)));
}
