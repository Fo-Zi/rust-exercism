use std::collections::{HashMap, HashSet};

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    // Parse the equation
    let (left, right) = input.split_once('=').unwrap();
    let left_words: Vec<&str> = left.split('+').map(|w| w.trim()).collect();
    let right_word = right.trim();

    let mut letters_counters: HashSet<char> = HashSet::new();
    let mut letters_values: HashMap<char, u8> = HashMap::new();

    // Populating the unique letters present at the input
    left_words.iter().for_each(|word| {
        word.chars().for_each(|x| {
            letters_counters.insert(x);
        })
    });
    right_word.chars().for_each(|c| {
        letters_counters.insert(c);
    });



    Some(letters_values)
}
