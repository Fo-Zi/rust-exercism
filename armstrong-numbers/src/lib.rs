pub fn is_armstrong_number(num: u32) -> bool {
    
    let num_as_digits: Vec<u32> = num.
        to_string()
        .chars()
        .map(|x| x.to_digit(10).unwrap() as u32)
        .collect()
    ;

    let sum_of_pwr_digits: u32 = num_as_digits
        .iter()
        .map(|x| x.pow(num_as_digits.len() as u32) )
        .sum();

    sum_of_pwr_digits == num 

}
