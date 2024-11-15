pub fn is_armstrong_number(num: u32) -> bool {
    let num_as_string = num.to_string(); 
    let nr_digits = num_as_string.len() as u32;

    num == num_as_string
        .chars()
        .map(|x| x.to_digit(10).unwrap().pow(nr_digits) )
        .sum()

}