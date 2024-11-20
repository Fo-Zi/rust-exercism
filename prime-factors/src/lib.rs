pub fn factors(n: u64) -> Vec<u64> {
    let mut divisors: Vec<u64> = Vec::new();
    let mut current_divisor = 2;
    let mut current_divisable = n;
    loop{
        if current_divisable == 1{
            break;
        }

        if current_divisable % current_divisor == 0 {
            divisors.push(current_divisor);
            current_divisable = current_divisable / current_divisor;
        }else{
            current_divisor = get_next_prime(current_divisor);
            if current_divisor > n {
                break;
            }
        }

    }
    divisors
}


pub fn get_next_prime(current_prime: u64) -> u64{
    let mut current_num :u64= current_prime + 1;
    while !is_prime(current_num){
        current_num+=1;
    }
    current_num
}

fn is_prime(n: u64) -> bool {
    match n {
        0 | 1 => false,                      // 0 and 1 are not prime
        2 | 3 => true,                       // 2 and 3 are prime
        n if n % 2 == 0 || n % 3 == 0 => false, // Even numbers and multiples of 3 are not prime
        _ => {
            // Use the 6k ± 1 rule for numbers > 3
            let mut i = 5;
            let limit = (n as f64).sqrt() as u64;
            while i <= limit {
                if n % i == 0 || n % (i + 2) == 0 {
                    return false;
                }
                i += 6;
            }
            true
        }
    }
}