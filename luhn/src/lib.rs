use std::arch::x86_64::_CMP_FALSE_OQ;

/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    if code.len() <= 1 {
        false
    }

    let stripped_str = code.trim();
    if stripped_str.chars().any(|x| !x.is_alphanumeric()) {
        false
    }
    

}

pub fn double_every_2nd_digit(input_str: &str){
    input_str.chars().map(|(index,x)| )
}
