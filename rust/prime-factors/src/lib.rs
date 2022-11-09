pub fn factors(mut n: u64) -> Vec<u64> {
    let mut facts = vec![];
    let mut numbers = 2..;

    while n > 1 {
        let x = numbers.next().unwrap();
        while n % x == 0 {
            n /= x;
            facts.push(x);
        }
    }

    facts
}