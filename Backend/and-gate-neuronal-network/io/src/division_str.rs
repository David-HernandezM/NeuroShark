use gstd::prelude::*;

use crate::utils::{
    integer_and_decimal,
    is_zeros_string,
    zeros_string,
    size_of_integer_float,
    integer_of_float_str
};

use crate::multiplication_str::multiply_floats_strings;

pub fn long_floats_division_strings(dividend: &String, divider: u64) -> Result<String, ()> {
    if dividend.len() < 1 {
        return Err(());
    }
    
    let mut result = String::new();
    let mut num_bytes = dividend.as_bytes().iter();
    let mut num_of_decimals = 11;
    let mut temp = *num_bytes
        .next()
        .unwrap() as u64 - 48;
    let mut add_dot = true;
    let mut count_to_add_zeros = 2;
    
    while temp < divider {
        match num_bytes.next() {
            Some(num) => {
                if *num != 46_u8 {
                    temp = temp * 10 + (*num as u64 - 48);
                    if count_to_add_zeros == 0 {
                        result.push('0');
                        num_of_decimals -= 1;
                    } else if count_to_add_zeros == 1 {
                        count_to_add_zeros = 0;
                        num_of_decimals -= 1;
                    }
                } else {
                    add_dot = false;
                    result.push_str("0.");
                    count_to_add_zeros = 1;
                }
            },
            None => {
                temp *= 10;
                result.push('0');
            }
        }
    }
    
    let mut add_zero = false;
    let mut add_first_dot = false;
    loop {
        match num_bytes.next() {
            Some(num) => {
                if *num != 46_u8 {
                    let ans = temp / divider;
                    result.push((ans as u8 + 48) as char);
                    temp = (temp % divider) * 10 + (*num as u64 - 48);
                    if add_first_dot {
                        result.push('.');
                        add_first_dot = false;
                    }
                } else {
                    add_first_dot = true;
                    add_dot = false;
                }
            },
            None => {
                if temp >= divider {
                    let ans = temp / divider;
                    temp = temp % divider;
                    result.push((ans as u8 + 48) as char);
                    num_of_decimals -= 1;
                    add_zero = false;
                } else {
                    temp = (temp % divider) * 10;
                    if add_zero {
                        result.push('0');
                    }
                    if add_dot {
                        result.push('.');
                        add_dot = false;
                    }
                    add_zero = true;
                }
                if num_of_decimals <= 0 || temp == 0 {
                    break;
                }
            }
        }
    }
    if result.len() == 0 {
        return Ok("0.0".to_string());
    }
    Ok(result)
}

pub fn floats_division_strings(dividend: &String, divider: &String) -> String {
    let mut dividend_is_negative = false;
    let mut divider_is_negative = false;

    if &dividend[..1] == "-" {
        dividend_is_negative = true;
    }
    if &divider[..1] == "-" {
        divider_is_negative = true;
    }
    
    let mut dividend: String = if dividend_is_negative {
        dividend[1..].to_string()
    } else {
        dividend.to_string()
    };
    
    let mut divider: String = if divider_is_negative {
        divider[1..].to_string()
    } else {
        divider.to_string()
    };
    
    let (integer1, decimal1) = integer_and_decimal(&dividend).unwrap();
    let (integer2, decimal2) = integer_and_decimal(&divider).unwrap();
    
    if is_zeros_string(&integer1) && is_zeros_string(&decimal1) {
        return String::from("0.0");
    }
    
    if is_zeros_string(&integer2) && is_zeros_string(&decimal2) {
        return String::from("0.0");
    }
    
    let mut decimal1_is_zero = false;
    let mut decimal2_is_zero = false;
    
    if is_zeros_string(&decimal1) {
        decimal1_is_zero = true;
    }
    if is_zeros_string(&decimal2) {
        decimal2_is_zero = true;
    }
    
    while size_of_integer_float(&integer_of_float_str(&dividend)) > 4 
          || size_of_integer_float(&integer_of_float_str(&divider)) > 4 {
        if decimal1_is_zero && decimal2_is_zero {
            break;
        } else if !decimal1_is_zero {
            divider = multiply_floats_strings(&divider, &format!("0.{}",decimal1)).unwrap();
            dividend = multiply_floats_strings(&dividend, &format!("0.{}",decimal1)).unwrap();
        } else {
            divider = multiply_floats_strings(&divider, &format!("0.{}",decimal2)).unwrap();
            dividend = multiply_floats_strings(&dividend, &format!("0.{}",decimal2)).unwrap();
        }
    }
    
    let (integer1, decimal1) = integer_and_decimal(&dividend).unwrap();
    let (integer2, decimal2) = integer_and_decimal(&divider).unwrap();
    
    if !is_zeros_string(&decimal1) {
        decimal1_is_zero = false;
    } else {
        decimal1_is_zero = true;
    }
    if !is_zeros_string(&decimal2) {
        decimal2_is_zero = false;
    } else {
        decimal2_is_zero = true;
    }
    
    let divider_number: u64;
    let dividend_number: String;
    if !decimal1_is_zero && !decimal2_is_zero {
        divider_number = format!("{}{}", integer2, decimal2)
            .parse()
            .expect("Error parsing");
        if decimal1.len() == decimal2.len() {
            dividend_number = format!("{}{}.0", integer1, decimal1);
        } else if decimal1.len() < decimal2.len() {
            let zeros = zeros_string((decimal2.len() - decimal1.len()) as u64);
            dividend_number = format!("{}{}{}.0", integer1, decimal1, zeros);
        } else {
            let decimal_updated_dot = format!("{}.{}", 
                &decimal1[..decimal2.len()],
                &decimal1[decimal2.len()..],
            );
            dividend_number = format!("{}{}", integer1, decimal_updated_dot);
        }
    } else if decimal1_is_zero && !decimal2_is_zero {
        divider_number = format!("{}{}", integer2, decimal2)
            .parse()
            .expect("Error parsing");
        let zeros = zeros_string(decimal2.len() as u64);
        dividend_number = format!("{}{}.0", integer1, zeros);
    } else if !decimal1_is_zero && decimal2_is_zero {
        divider_number = integer2
            .parse()
            .expect("Error parsing");
        dividend_number = dividend.to_string();
    } else {
        divider_number = integer2
            .parse()
            .expect("Error parsing");
        dividend_number = format!("{}.0", integer1.to_string());
    }
    
    let mut result = long_floats_division_strings(&dividend_number, divider_number)
        .unwrap_or_else(|_| String::from("Error!!"));
        
    let (integer_result, decimal_result) = integer_and_decimal(&result).unwrap();
    result = if decimal_result.len() > 12 {
        format!("{}.{}", integer_result, &decimal_result[..12])
    } else {
        result
    };
        
    if (!dividend_is_negative && divider_is_negative) || (dividend_is_negative && !divider_is_negative) {
        format!("-{}", result)
    } else {
        result
    }
}









/*
pub fn floats_division_strings(dividend: &String, divider: &String) -> String {
    let mut dividend_is_negative = false;
    let mut divider_is_negative = false;
    println!("DIVIDENDDDDDDDDD = {}", dividend);
    println!("DIVIDEEEEEEEEEER = {}", divider);
    
    
    /*
    if size_of_integer_float(&divider) > 6 {
        return String::from("0.0");
    }
    */
    
    
    
    if &dividend[..1] == "-" {
        dividend_is_negative = true;
    }
    if &divider[..1] == "-" {
        divider_is_negative = true;
    }
    
    let dividend: String = if dividend_is_negative {
        dividend[1..].to_string()
    } else {
        dividend.to_string()
    };
    
    let divider: String = if divider_is_negative {
        divider[1..].to_string()
    } else {
        divider.to_string()
    };
    
    let (mut integer1, mut decimal1) = integer_and_decimal(&dividend).unwrap();
    let (integer2, mut decimal2) = integer_and_decimal(&divider).unwrap();
    
    if is_zeros_string(&integer1) && is_zeros_string(&decimal1) {
        return String::from("0.0");
    }
    
    if is_zeros_string(&integer2) && is_zeros_string(&decimal2) {
        return String::from("0.0");
    }
    
    let mut decimal1_is_zero = false;
    let mut decimal2_is_zero = false;
    
    if is_zeros_string(&decimal1) {
        decimal1_is_zero = true;
    }
    if is_zeros_string(&decimal2) {
        decimal2_is_zero = true;
    }
    
    let divider_number: u64;
    let dividend_number: String;
    if !decimal1_is_zero && !decimal2_is_zero {
        divider_number = format!("{}{}", integer2, decimal2)
            .parse()
            .expect("Error parsing");
        if decimal1.len() == decimal2.len() {
            //dividend_number = dividend.to_string();
            dividend_number = format!("{}{}.0", integer1, decimal1);
        } else if decimal1.len() < decimal2.len() {
            let zeros = zeros_string(decimal2.len() - decimal1.len());
            dividend_number = format!("{}{}{}.0", integer1, decimal1, zeros);
        } else {
            let decimal_updated_dot = format!("{}.{}", 
                &decimal1[..decimal2.len()],
                &decimal1[decimal2.len()..],
            );
            dividend_number = format!("{}{}", integer1, decimal_updated_dot);
        }
        //println!("PROBABLY HERE IS THE PROBLEM::::::     {}", dividend_number);
    } else if decimal1_is_zero && !decimal2_is_zero {
        println!("Posible error: ");
        println!("Integer2: {}", integer2);
        println!("decimal2: {}", decimal2);
        divider_number = format!("{}{}", integer2, decimal2)
            .parse()
            .expect("Error parsing");
        let zeros = zeros_string(decimal2.len());
        dividend_number = format!("{}{}.0", integer1, zeros);
    } else if !decimal1_is_zero && decimal2_is_zero {
        divider_number = integer2
            .parse()
            .expect("Error parsing");
        dividend_number = dividend.to_string();
    } else {
        divider_number = integer2
            .parse()
            .expect("Error parsing");
        dividend_number = format!("{}.0", integer1.to_string());
    }
    
    //println!("\n A dividir --------------------------------------------------");
    //println!("{} / {}", dividend_number, divider_number);
    
    let mut result = long_floats_division_strings(&dividend_number, divider_number)
        .unwrap_or_else(|_| String::from("Error!!"));
        
    let (integer_result, decimal_result) = integer_and_decimal(&result).unwrap();
    result = if decimal_result.len() > 12 {
        format!("{}.{}", integer_result, &decimal_result[..12])
    } else {
        result
    };
        
    if (!dividend_is_negative && divider_is_negative) || (dividend_is_negative && !divider_is_negative) {
        format!("-{}", result)
    } else {
        result
    }
}
*/