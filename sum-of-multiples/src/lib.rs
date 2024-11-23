use std::collections::HashSet;

pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut multiples_vec: Vec<u32> = Vec::new();    
    for factor in factors{
        if *factor > 0 {
            (1..).into_iter().take_while(|x| *x * factor < limit).for_each(|y| multiples_vec.push(y * factor));
        }
    }
    let unique_multiples: HashSet<_> = multiples_vec.into_iter().collect();
    unique_multiples.iter().sum()
}
