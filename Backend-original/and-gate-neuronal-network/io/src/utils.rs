use gstd::prelude::*;

pub fn float_string_is_zero(num: &String) -> bool {
    let (integer, decimal) = integer_and_decimal(num).unwrap();
    is_zeros_string(&integer) && is_zeros_string(&decimal)
}

pub fn integer_and_decimal(num: &String) -> Result<(String, String), ()> {
    let index_dot = float_str_dot_position(num).unwrap();
    if index_dot == 0 {
        return Err(());
    }
    Ok((num[0..index_dot as usize].to_string(), num[(index_dot+1) as usize..].to_string()))
}

pub fn integer_of_float_str(num: &String) -> String {
    let dot_position = float_str_dot_position(num).unwrap();
    (&num[..dot_position as usize]).to_string()
}

pub fn float_string_without_dot(num: &String) -> Result<String, String> {
    let index_of_point = match float_str_dot_position(num) {
        Some(index) => index,
        None => return Err(String::from("The number is not a float")),
    };
    
    let mut integer = &num[..index_of_point as usize];
    if &integer[..1] == "0" {
        integer = &num[1..index_of_point as usize];
    }
    let decimal = &num[(index_of_point + 1) as usize..];
    
    Ok(format!("{}{}", integer, decimal))
}

pub fn zeros_string(size: u64) -> String {
    let mut zeros = String::new();
    for _i in 0..size {
        zeros.push('0');
    }
    zeros
}

pub fn size_of_decimal(num: &String) -> u64 {
    let mut count = 0;
    let mut dot_passed = false;
    for c in num.trim().chars() {
        if dot_passed {
            count += 1;
        }
        if c == '.' {
            dot_passed = true;
        }
    }
    count
}

pub fn size_of_integer_float(num: &String) -> u64 {
    let mut count = 0;
    for c in num.trim().chars() {
        if c == '.' {
            return count;
        }
        count += 1;
    }
    return count;
}

pub fn float_str_dot_position(num: &String) -> Option<usize> {
    num
        .chars()
        .position(|c| c == '.')
}

pub fn is_zeros_string(num: &String) -> bool {
    for c in num.chars() {
        if c != '0' {
            return false;
        }
    }
    return true;
}
