use gstd::prelude::*;

use crate::utils::{
    float_string_without_dot,
    integer_and_decimal,
    zeros_string,
    is_zeros_string
};

use crate::sums_str::add_nums_strings;

pub fn subtract_floats_strings(num1: &String, num2: &String) -> String {
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
    
    let mut num1_without_decimal = float_string_without_dot(&num1)
        .unwrap();
    let mut num2_without_decimal = float_string_without_dot(&num2)
        .unwrap();
        
    let (_, decimal1) = integer_and_decimal(&num1)
        .unwrap();
    let (_, decimal2) = integer_and_decimal(&num2)
        .unwrap();        
    let difference = (decimal1.len() as i64 - decimal2.len() as i64)
        .abs() as u64;
    let zeros = &zeros_string(difference)[..];
    let decimal_total_size;
    
    if decimal1.len() > decimal2.len() {
        decimal_total_size = decimal1.len();
        num2_without_decimal.push_str(zeros);
    } else {
        decimal_total_size = decimal2.len();
        num1_without_decimal.push_str(zeros);
    }
    
    let mut result;
    if !num1_is_negative && !num2_is_negative {
        result = subtract_nums_strings(&num1_without_decimal, &num2_without_decimal);
    } else if num1_is_negative && num2_is_negative {
        result = subtract_nums_strings(&num2_without_decimal, &num1_without_decimal);
    } else if !num1_is_negative && num2_is_negative {
        result = add_nums_strings(&num1_without_decimal, &num2_without_decimal);
    } else {
        result = format!("-{}", add_nums_strings(&num1_without_decimal, &num2_without_decimal));
    }
    
    if is_zeros_string(&result) {
        return String::from("0.0");
    }
    
    let mut modified_result: String;
    let sign = if &result[..1] == "-" {
        modified_result = result[1..].to_string();
        "-"
    } else {
        modified_result = result[..].to_string();
        ""
    };
    
    if modified_result.len() < 12 {
        let subtraction = 12 - modified_result.len(); 
        let zeros = zeros_string(subtraction as u64);
        modified_result = format!("{}{}", zeros, modified_result);
    }
    
    let dot_index = modified_result.len() - decimal_total_size;
    
    let integer = if dot_index != 0 {
        &modified_result[..dot_index]
    } else {
        "0"
    };
    
    let decimal = &modified_result[dot_index..];
    result = format!("{}{}.{}", sign, integer, decimal);
    
    let (integer_result, decimal_result) = integer_and_decimal(&result).unwrap();

    result = if decimal_result.len() > 12 {
        format!("{}.{}", integer_result, &decimal_result[..12])
    } else {
        result
    };
    
    result
}

pub fn subtract_nums_strings(number1: &String, number2: &String) -> String {
    let minuend;
    let subtracting;
    let mut result_is_negative = false;
    if number1.len() == number2.len() {
        let first_number = number1.as_bytes()[0] - 48;
        let second_number = number2.as_bytes()[0] - 48;
        if first_number < second_number {
            minuend = number2.to_string();
            subtracting = number1.to_string();
            result_is_negative = true;
        } else {
            minuend = number1.to_string();
            subtracting = number2.to_string();
        }
    } else if number1.len() > number2.len() { 
        let difference = number1.len() - number2.len();
        subtracting = format!("{}{}", zeros_string(difference as u64), number2);
        minuend = number1.to_string();
    } else {
        let difference = number2.len() - number1.len();
        subtracting = format!("{}{}", zeros_string(difference as u64), number1);
        minuend = number2.to_string();
        result_is_negative = true;
    }
    let mut result = zeros_string(minuend.len() as u64);
    let mut borrow = 0;
    let minuend = minuend.as_bytes();
    let subtracting = subtracting.as_bytes();
    for i in (0..minuend.len()).rev() {
        let digit1 = (minuend[i] - 48) as i64;
        let digit2 = (subtracting[i] - 48) as i64;
        let mut difference: i64 = digit1 - digit2 - borrow;
        if difference < 0 {
            difference += 10;
            borrow = 1;
        } else {
            borrow = 0;
        }
        result.replace_range(i..i+1, &difference.to_string()[..]);
    }
    if &result[..1] == "0" {
        result = result[1..].to_string();
    }
    if result_is_negative {
        format!("-{}", result) 
    } else {
        result
    }
}

