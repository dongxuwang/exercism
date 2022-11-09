static WORDS: &[(usize, &'static str)] = &[(3, "Pling"), (5, "Plang"), (7, "Plong")];
pub fn raindrops(n: usize) -> String {
    let res: String = WORDS
        .iter()
        .map(|&(d, w)| if n % d == 0 { w } else { "" })
        .collect();
    match res.is_empty() {
        true => n.to_string(),
        false => res,
    }
}
