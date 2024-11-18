pub fn square(s: u32) -> u64 {
    (2 as u64).pow(s-1) 
}

pub fn total() -> u64 {
    (1..=64).into_iter().map(|s| square(s)).sum()
}
