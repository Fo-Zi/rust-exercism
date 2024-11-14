use std::arch::x86_64::_CMP_FALSE_OQ;

/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {

    let stripped_str = code.replace(char::is_whitespace, "");
    if stripped_str.chars().any(|x| !x.is_digit(10)) {
        return false;
    }
    
    if stripped_str.len() <= 1 {
        return false;
    }

    let doubled_input: Vec<u32> = stripped_str
        .chars()
        .rev()
        .enumerate()
        .map(   |(index,char)| {
            let mut num = char.to_digit(10).unwrap(); 
            if index % 2 == 1 { 
                num *=2;
                if num > 9 {
                    num -= 9;
                }
            }   
            num
        })      
        .collect();

    let sum: u64 = doubled_input.iter().map(|x| *x as u64 ).sum();
    if sum % 10 == 0 {
        return true;
    }else{
        return false;
    }

}
