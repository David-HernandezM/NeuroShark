use gstd::prelude::*;

use crate::utils::{
    integer_and_decimal,
    zeros_string,
    float_string_is_zero,
    size_of_decimal,
    float_string_without_dot,
    is_zeros_string
};

pub fn multiply_floats_strings(num1: &String, num2: &String) -> Result<String, ()> {
    if float_string_is_zero(num1) || float_string_is_zero(num2) {
        return Ok(String::from("0.0"));
    }
    
    let mut num1_is_negative = false;
    let mut num2_is_negative = false;
    
    if &num1[..1] == "-" {
        num1_is_negative = true;
    }
    if &num2[..1] == "-" {
        num2_is_negative = true;
    }
    
    let num1: String = if num1_is_negative {
        num1[1..].to_string()
    } else {
        num1.to_string()
    };
    
    let num2: String = if num2_is_negative {
        num2[1..].to_string()
    } else {
        num2.to_string()
    };
    
    let decimal1_size = size_of_decimal(&num1);
    let decimal2_size = size_of_decimal(&num2); 
    let decimal_total_size = decimal1_size + decimal2_size;
    
    if decimal1_size == 0 || decimal2_size == 0 {
        return Err(());
    }

    let num1_without_point = float_string_without_dot(&num1)
        .unwrap();
    let num2_without_point = float_string_without_dot(&num2)
        .unwrap();
        
    let mult_res = multiply_numbers_in_strings(
        &num1_without_point,
        &num2_without_point
    );
    
    let mut result: String;
    let temp = mult_res.len() as isize - decimal_total_size as isize;
    if temp <= 0 {
        if (num1_is_negative && !num2_is_negative) || (!num1_is_negative && num2_is_negative) {
            result = String::from("-0.");
        } else {
            result = String::from("0.");
        }
        let zeros = zeros_string(temp.abs() as u64);
        result.push_str(&zeros[..]);
        result.push_str(&mult_res[..]);
    } else {
        let mut integer = &mult_res[..temp as usize];
        
        integer = if &integer[..1] == "0" {
            &integer[1..]
        } else {
            integer
        };
        
        integer = if !is_zeros_string(&integer.to_string()) {
            integer
        } else {
            "0"
        };
        
        let mut decimal = &mult_res[temp as usize..];
        
        decimal = if decimal.len() > 12 {
            &decimal[..12]
        } else {
            decimal
        };
        
        if (num1_is_negative && !num2_is_negative) || (!num1_is_negative && num2_is_negative) {
            result = format!("-{}.{}", integer, decimal);
        } else {
            result = format!("{}.{}", integer, decimal);
        }
        
    }
    
    let (integer_result, decimal_result) = integer_and_decimal(&result).unwrap();
    result = if decimal_result.len() > 12 {
        format!("{}.{}", integer_result, &decimal_result[..12])
    } else {
        result
    };
    
    Ok(result)
}

pub fn multiply_numbers_in_strings(num1: &String, num2: &String) -> String {
    let mut result = zeros_string((num1.len() + num2.len()) as u64);
    num1
        .trim()
        .as_bytes()
        .iter()
        .enumerate()
        .rev()
        .for_each(|c1| {
            let mut carry = 0;
            let digit1 = *c1.1 as u64 - 48;
            num2
                .trim()
                .as_bytes()
                .iter()
                .enumerate()
                .rev()
                .for_each(|c2| {
                    let digit2 = *c2.1 as u64 - 48;
                    let index = c1.0 + c2.0 + 1;
                    let num_at_index = result
                        .as_bytes()[index] as u64 - 48;
                    let product = (digit1 * digit2) + num_at_index + carry;
                    carry = product / 10;
                    result.replace_range(index..index+1, &(product % 10).to_string()[..]);
                });
                let num_at_index = result
                        .as_bytes()[c1.0] as u64 - 48;
                let add_carry = num_at_index + carry;
                result.replace_range((c1.0)..(c1.0)+1, &add_carry.to_string()[..]);
        });
    let start_position = result
        .chars()
        .position(|c| c != '0')
        .unwrap_or_else(|| u64::MAX as usize);
    if start_position as u64 != u64::MAX {
        result
    } else {
        String::from("0")
    }
}