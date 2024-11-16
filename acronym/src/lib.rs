pub fn abbreviate(phrase: &str) -> String {
    let words = splitter(phrase);
    let mut acronym_letters: Vec<char> = Vec::new();
    for word in words.iter() {
        let mut consecutive_uppercase = false;
        for (index,ch) in word.chars().enumerate() {
            if ch.is_ascii_uppercase(){
                if !consecutive_uppercase{
                    acronym_letters.push(ch);
                    consecutive_uppercase = true;
                }
            }else if index == 0 && word.len() > 1 {
                acronym_letters.push(ch.to_ascii_uppercase());
                consecutive_uppercase = false;
            }else{
                consecutive_uppercase = false;
            }
        }
    } 
    let acronym_string : String = acronym_letters.into_iter().collect();
    acronym_string
}

pub fn splitter(phrase: &str) -> Vec<&str> {
    let non_letter_splitted: Vec<&str> = phrase
        .split(|c: char| !c.is_alphabetic() || c.is_whitespace() )
        .filter(|c| !c.is_empty())
        .collect();
    non_letter_splitted 
}