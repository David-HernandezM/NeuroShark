use gstd::prelude::*;

use crate::utils::{
    float_string_is_zero,  
    integer_and_decimal,
    is_zeros_string,
};

use crate::{
    multiplication_str::multiply_floats_strings,
    division_str::floats_division_strings,
    sums_str::add_floats_strings,
};


// POTENCIA DE EULER POR NUMERO ENTERO ------------------------------------------------------------------------------------------------------------------

pub fn float_string_euler_exponentiation(exponent: &String) -> String {
    let mut exponent_is_negative = false;
    let exponent: String = if &exponent[..1] == "-" {
        exponent_is_negative = true;
        (&exponent[1..]).to_string()
    } else {
        exponent.to_string()
    };
    
    if float_string_is_zero(&exponent) {
        return String::from("1.0");
    }
    let e = String::from("2.71828182845904");
    let (integer, decimal) = integer_and_decimal(&exponent).unwrap();
    let decimal = format!("0.{}", decimal);
    
    let result;
    if !is_zeros_string(&integer) {
        let integer: u64 = integer
        .parse()
        .expect("Errror parsing to u64");
        let partial_result_integer = exponent_float_string(&e, integer);
        let partial_result_decimal = aproximating_exponential(&decimal);
        result = multiply_floats_strings(&partial_result_integer, &partial_result_decimal)
            .unwrap();
    } else {
        result = aproximating_exponential(&decimal);
    }
    
    if exponent_is_negative {
        floats_division_strings(&String::from("1.0"), &result)
    } else {
        result
    }
}

// --------------------------------------------------------------------------------------------------------------------------------------------------


// Aproximating exponentials with Taylor Polynommials --------------------------------------------------------------------------------------------------

pub fn aproximating_exponential(num: &String) -> String {
    let mut exponent_is_negative = false;
    let num: String = if &num[..1] == "-" {
        exponent_is_negative = true;
        (&num[1..]).to_string()
    } else {
        num.to_string()
    };
     
    let exp1 = multiply_floats_strings(&num, &num).unwrap(); // num^2
    let exp2 = multiply_floats_strings(&exp1, &num).unwrap(); // num^3
    let exp3 = multiply_floats_strings(&exp2, &num).unwrap(); // num^4
    let exp4 = multiply_floats_strings(&exp3, &num).unwrap(); // num^5
    let exp5 = multiply_floats_strings(&exp4, &num).unwrap(); // num^6
    let exp6 = multiply_floats_strings(&exp5, &num).unwrap(); // num^7
    
    let x1 = String::from("1.0");
    let x2 = num;
    let x3 = floats_division_strings(&exp1, &String::from("2.0"));
    let x4 = floats_division_strings(&exp2, &String::from("6.0"));
    let x5 = floats_division_strings(&exp3, &String::from("24.0"));
    let x6 = floats_division_strings(&exp4, &String::from("120.0"));
    let x7 = floats_division_strings(&exp5, &String::from("720.0"));
    let x8 = floats_division_strings(&exp6, &String::from("5040.0"));
    let mut result = add_floats_strings(&x1, &x2);
    result = add_floats_strings(&result, &x3);
    result = add_floats_strings(&result, &x4);
    result = add_floats_strings(&result, &x5);
    result = add_floats_strings(&result, &x6);
    result = add_floats_strings(&result, &x7);
    result = add_floats_strings(&result, &x8);
    
    if exponent_is_negative {
        floats_division_strings(&String::from("1.0"), &result)
    } else {
        result
    }
}

// --------------------------------------------------------------------------------------------------------------------------------------------------


// POTENCIA DE NUMERO ENTERO ------------------------------------------------------------------------------------------------------------------

pub fn exponent_float_string(base: &String, exponent: u64) -> String {
    if exponent == 0 {
        return String::from("1.0");
    }
    if float_string_is_zero(base) {
        return String::from("0.0");
    }
    let mut result = base.to_string();
    for _i in 0..exponent-1 {
        result = multiply_floats_strings(&result, base).unwrap();
    }
    result
}

// --------------------------------------------------------------------------------------------------------------------------------------------------
