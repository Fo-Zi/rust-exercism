use std::{collections::HashSet,collections::HashMap};

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    //todo!("For the '{word}' word find anagrams among the following words: {possible_anagrams:?}");
    let mut word_letter_counts = HashMap::new();
    let lower_case_word = word.to_lowercase();
    for letter in lower_case_word.chars() {
        *word_letter_counts.entry(letter).or_insert(0) += 1;
    } 

    let mut matched_anagrams= HashSet::new();
    let hash_iter = possible_anagrams.iter();

    for possible_anagram in hash_iter {
        let mut possible_anagram_letter_counts = HashMap::new();
        let lower_case_possible_anagram = possible_anagram.to_lowercase();
        for letter in lower_case_possible_anagram.chars() {
            *possible_anagram_letter_counts.entry(letter).or_insert(0) += 1;
        } 

        let mut matched_anagram = true;
        for (letter,letter_count) in possible_anagram_letter_counts{
            if word_letter_counts.get(&letter) != Some(&letter_count){
                matched_anagram = false;
                break;
            }
        }
        if matched_anagram && *possible_anagram.to_lowercase() != word.to_lowercase(){
            let anagram: &str = possible_anagram;
            matched_anagrams.insert(anagram);
        }
        
    }

    matched_anagrams
}