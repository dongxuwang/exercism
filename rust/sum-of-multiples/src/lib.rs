pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    (1..limit).filter(|x| factors.iter().filter(|&&y| y != 0u32).any(|y| x % y == 0)).sum()
}
