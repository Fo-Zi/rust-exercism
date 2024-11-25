use core::num;

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    InvalidInputBase,
    InvalidOutputBase,
    InvalidDigit(u32),
}

pub fn convert(number: &[u32], from_base: u32, to_base: u32) -> Result<Vec<u32>, Error> {

    if from_base == 0 || from_base == 1 {
        return Err(Error::InvalidInputBase);
    }

    if to_base == 0 || to_base == 1 {
        return Err(Error::InvalidOutputBase);
    }

    let mut number_in_output_base: Vec<u32> = Vec::new();
    let number_leading_zeros_free: Vec<u32> = get_leading_zeros_free_vec(number);

    if number.len() == 0 || number_leading_zeros_free.len() == 0 {
        number_in_output_base.push(0);
        return Ok(number_in_output_base);
    }

    for elem in number_leading_zeros_free.iter() {
        if *elem >= from_base {
            return Err(Error::InvalidDigit(*elem));
        }
    }

    let number_full = number_leading_zeros_free
        .iter()
        .rev()
        .enumerate()
        .map(|(index,x)| x * (from_base).pow(index as u32) )
        .sum::<u32>();

    let mut div_result = number_full;
    while div_result >= to_base {
        number_in_output_base.push(div_result % to_base);
        div_result = div_result / to_base;
    }
    if div_result % to_base != 0 {
        number_in_output_base.push(div_result % to_base);
    }
    number_in_output_base = number_in_output_base.into_iter().rev().collect();

    Ok(get_leading_zeros_free_vec(&number_in_output_base))

}


pub fn get_leading_zeros_free_vec(vec : &[u32]) -> Vec<u32> {
    vec
        .iter()
        .skip_while(|&&x| x == 0) 
        .copied()                 
        .collect()
}