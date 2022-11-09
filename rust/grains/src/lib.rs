pub fn square(s: u32) -> u64 {
    match s {
        n if n > 64 || n < 1 => panic!("Square must be between 1 and 64"),
        _ => 2u64.pow(s - 1)
    }
}

pub fn total() -> u64 {
    (1..=64).map(square).sum()
}
