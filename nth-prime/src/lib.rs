pub fn nth(n: u32) -> u32 {
    match n {
        0 => 2,
        1 => 3,
        _ => get_nth_prime(n),
    }
}


pub fn get_nth_prime(nth: u32) -> u32{
    let mut prime_counter: u32 = 0;
    let mut current_num: u32 = 3;
    loop {
        let mut is_prime = true;
        for num in 2..current_num {
            if current_num % num == 0 {
                is_prime = false;
                break;
            }
        }

        if is_prime {
            prime_counter += 1;
            if prime_counter == nth {
                return current_num;
            }
        }

        current_num += 1;
    }
}