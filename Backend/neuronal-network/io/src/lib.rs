#![no_std]
use gstd::{prelude::*, Vec};
use gmeta::{Metadata, In, Out, InOut};

pub mod network;
pub mod matrix;
pub mod exponentiation_str;
pub mod division_str;
pub mod multiplication_str;
pub mod sums_str;
pub mod subtraction_str;
pub mod utils;

use network::Network; 

pub struct ProgramMetadata;

impl Metadata for ProgramMetadata {
    type Init = In<NNMessageIn>;
    type Handle = InOut<NNMessageIn, NNMessageOut>;
    type Reply = ();
    type Others = ();
    type Signal = ();
    type State = Out<Network>;
}

#[derive(Encode, Decode, TypeInfo)]
pub enum NNMessageIn {
    SetLogicalXorNeuronalNetwork,
    PredictResultLogicalXor((BinaryLogic, BinaryLogic)),
    SetDefaultNeuronalNetwork,
    SetNewTrainedNeuronalNetwork(NNStructure),
    SetTrainedWeightsMatrix(Vec<Vec<Vec<String>>>),
    SetTrainedBiasMatrix(Vec<Vec<Vec<String>>>),
    //PredictResultOf(Vec<String>),
}

#[derive(Encode, Decode, TypeInfo)]
pub enum NNMessageOut {
    NeuronalNetworkCreated,
    DefaultNeuronalNetworkCreated,
    EstablishedNeuronalNetworkData,
    EstablishedWeightMatrix,
    EstablishedBiasMatrix,
    Prediction(Vec<String>),
    ErrorCreatingNeuronalNetwork(String),
    ErrorSettingTrainedBias(String),
    ErrorSettingTrainedWeights(String),
}

#[derive(Encode, Decode, TypeInfo)]
pub struct NNStructure {
    pub layers: Vec<u64>,
	pub weights: Vec<Vec<Vec<String>>>,
	pub biases: Vec<Vec<Vec<String>>>,
	pub learning_rate: String,
}

#[derive(Encode, Decode, TypeInfo)]
pub enum BinaryLogic {
    One,
    Zero
}

/*

// POTENCIA DE EULER POR NUMERO DECIMAL --------------------------------------------------------------------------------------------------------------

fn float_string_euler_exponentiation(exponent: &String) -> String {
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
    let e = String::from("2.718281828459041");
    let (integer, decimal) = integer_and_decimal(&exponent).unwrap();
    let decimal = format!("0.{}", decimal);
    
    let result;
    if !is_zeros_string(&integer) {
        let integer: usize = integer
        .parse()
        .expect("Errror parsing to usize");
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

fn aproximating_exponential(num: &String) -> String {
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
    //println!("({})^2/2.0 = {}", num, x3);
    let x4 = floats_division_strings(&exp2, &String::from("6.0"));
    //println!("({})^3/6.0 = {}", num, x4);
    let x5 = floats_division_strings(&exp3, &String::from("24.0"));
    //println!("({})^4/24.0 = {}", num, x5);
    let x6 = floats_division_strings(&exp4, &String::from("120.0"));
    //println!("({})^5/120.0 = {}", num, x6);
    let x7 = floats_division_strings(&exp5, &String::from("720.0"));
    //println!("({})^7/720.0 = {}", num, x7);
    let x8 = floats_division_strings(&exp6, &String::from("5040.0"));
    //println!("({})^7/720.0 = {}", num, x7);
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

fn exponent_float_string(base: &String, exponent: usize) -> String {
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



// DIVISIONES ----------------------------------------------------------------------------------------------------------------------------------------

pub fn floats_division_strings(dividend: &String, divider: &String) -> String {
    let mut dividend_is_negative = false;
    let mut divider_is_negative = false;
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
        return String::from("ERROR");
    }
    
    let mut decimal1_is_zero = false;
    let mut decimal2_is_zero = false;
    
    if is_zeros_string(&decimal1) {
        decimal1_is_zero = true;
    }
    if is_zeros_string(&decimal2) {
        decimal2_is_zero = true;
    }
    
    let divider_number: usize;
    let dividend_number: String;
    if !decimal1_is_zero && !decimal2_is_zero {
        divider_number = format!("{}{}", integer2, decimal2)
            .parse()
            .expect("Error parsing");
        if decimal1.len() == decimal2.len() {
            dividend_number = dividend.to_string();
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
    } else if decimal1_is_zero && !decimal2_is_zero {
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
     
    let mut result = long_floats_division_strings(&dividend_number, divider_number)
        .unwrap_or_else(|_| String::from("Error!!"));
    let (integer_result, decimal_result) = integer_and_decimal(&result).unwrap();
    result = if decimal_result.len() > 14 {
        format!("{}.{}", integer_result, &decimal_result[..14])
    } else {
        result
    };
        
    if (!dividend_is_negative && divider_is_negative) || (dividend_is_negative && !divider_is_negative) {
        format!("-{}", result)
    } else {
        result
    }
}

fn long_floats_division_strings(dividend: &String, divider: usize) -> Result<String, ()> {
    if dividend.len() < 1 {
        return Err(());
    }
    
    let mut result = String::new();
    let mut num_bytes = dividend.as_bytes().iter();
    let mut num_of_decimals = 11;
    let mut temp = *num_bytes
        .next()
        .unwrap() as usize - 48;
    let mut add_dot = true;
    let mut count_to_add_zeros = 2;
    
    while temp < divider {
        match num_bytes.next() {
            Some(num) => {
                if *num != 46_u8 {
                    temp = temp * 10 + (*num as usize - 48);
                    if count_to_add_zeros == 0 {
                        result.push('0');
                    } else if count_to_add_zeros == 1 {
                        count_to_add_zeros = 0;
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
                    temp = (temp % divider) * 10 + (*num as usize - 48);
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
                if num_of_decimals == 0 || temp == 0 {
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

// --------------------------------------------------------------------------------------------------------------------------------------------------





// MULTIPLICACIONES ---------------------------------------------------------------------------------------------------------------------------------
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
    
    
    
    let decimal1_size = size_of_decimal(&num1); //float_str_dot_position(&num1).unwrap();
    let decimal2_size = size_of_decimal(&num2); //float_str_dot_position(&num2).unwrap();
    let decimal_total_size = decimal1_size + decimal2_size;
    //let point_result_ubication = index_of_point1 + index_of_point2;
    
    if decimal1_size == 0 || decimal2_size == 0 {
        return Err(());
    }

    let num1_without_point = float_string_without_dot(&num1)
        .unwrap();
    let num2_without_point = float_string_without_dot(&num2)
        .unwrap();
        
    let mut mult_res = multiply_numbers_in_strings(
        &num1_without_point,
        &num2_without_point
    );
    
    let mut result: String;
    let temp = mult_res.len() as isize - decimal_total_size as isize;
    //println!("Point ubication: {}", temp);
    if temp <= 0 {
        if (num1_is_negative && !num1_is_negative) || (!num1_is_negative && num1_is_negative) {
            result = String::from("-0.");
        } else {
            result = String::from("0.");
        }
        let zeros = zeros_string(temp.abs() as usize); //- mult_res.len());
        result.push_str(&zeros[..]);
        result.push_str(&mult_res[..]);
    } else {
        //println!("Result: {}", mult_res);
        //println!("Temp: {}", temp);
        let integer = &mult_res[..temp as usize];
        let integer = if &integer[..1] == "0" {
            &integer[1..]
        } else {
            integer
        };
        let mut decimal = &mult_res[temp as usize..];
        decimal = if decimal.len() > 15 {
            &decimal[..15]
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
    result = if decimal_result.len() > 14 {
        format!("{}.{}", integer_result, &decimal_result[..14])
    } else {
        result
    };
    
    Ok(result)
}

fn multiply_numbers_in_strings(num1: &String, num2: &String) -> String {
    let size1 = num1.len();
    let size2 = num2.len();
    let mut result = zeros_string(num1.len() + num2.len());
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
        .unwrap_or_else(|| usize::MAX);
    if start_position != usize::MAX {
        result
    } else {
        String::from("0")
    }
}
// --------------------------------------------------------------------------------------------------------------------------------------------------




// RESTAS -------------------------------------------------------------------------------------------------------------------------------------------
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
        .abs() as usize;
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
    
    let dot_index = result.len() - decimal_total_size;
    
    let integer = if dot_index != 0 {
        &result[..dot_index]
    } else {
        "0"
    };
    let decimal = &result[dot_index..];
    result = format!("{}.{}", integer, decimal);
    
    let (integer_result, decimal_result) = integer_and_decimal(&result).unwrap();
    result = if decimal_result.len() > 14 {
        format!("{}.{}", integer_result, &decimal_result[..14])
    } else {
        result
    };
    
    result
}

fn subtract_nums_strings(number1: &String, number2: &String) -> String {
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
        subtracting = format!("{}{}", zeros_string(difference), number2);
        minuend = number1.to_string();
    } else {
        let difference = number2.len() - number1.len();
        subtracting = format!("{}{}", zeros_string(difference), number1);
        minuend = number2.to_string();
        result_is_negative = true;
    }
    let mut result = zeros_string(minuend.len());
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

// --------------------------------------------------------------------------------------------------------------------------------------------------





// SUMAS ----------------------------------------------------------------------------------------------------------------------------------------------
pub fn add_floats_strings(num1: &String, num2: &String) -> String {
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
        .abs() as usize;
    let decimal_total_size;
    
    if decimal1.len() > decimal2.len() {
        num2_without_decimal.push_str(&zeros_string(difference)[..]);
        decimal_total_size = decimal1.len();
    } else {
        num1_without_decimal.push_str(&zeros_string(difference)[..]);
        decimal_total_size = decimal2.len();
    }
    
    let mut result;
    if !num1_is_negative && !num2_is_negative {
        result = add_nums_strings(&num1_without_decimal, &num2_without_decimal);
    } else if num1_is_negative && num2_is_negative {
        result = format!("-{}", add_nums_strings(&num1_without_decimal, &num2_without_decimal));
    } else if !num1_is_negative && num2_is_negative {
        result = subtract_nums_strings(&num1_without_decimal, &num2_without_decimal);
    } else {
        result = subtract_nums_strings(&num2_without_decimal, &num1_without_decimal);
    }
    
    let dot_index = result.len() - decimal_total_size;

    let integer = if dot_index != 0 {
        &result[..dot_index]
    } else {
        "0"
    };
    let decimal = &result[dot_index..];
    result = format!("{}.{}", integer, decimal);
    
    let (integer_result, decimal_result) = integer_and_decimal(&result).unwrap();
    result = if decimal_result.len() > 14 {
        format!("{}.{}", integer_result, &decimal_result[..14])
    } else {
        result
    };
    
    result
}

fn add_nums_strings(number1: &String, number2: &String) -> String {
    let num1;
    let num2;
    
    if number1.len() < number2.len() {
        let difference = zeros_string(number2.len() - number1.len());
        num1 = format!("{}{}", difference, number1);
        num2 = number2.to_string();
    } else {
        let difference = zeros_string(number1.len() - number2.len());
        num2 = format!("{}{}", difference, number2);
        num1 = number1.to_string();
    }
    
    let mut result = zeros_string(num1.len() + 1);
    let mut carry = 0;
    let num1 = num1.as_bytes();
    let num2 = num2.as_bytes();
    for i in (0..num1.len()).rev() {
        let digit1 = num1[i] - 48;
        let digit2 = num2[i] - 48;
        let mut sum = digit1 + digit2 + carry;
        carry = sum / 10;
        sum = sum % 10;
        result.replace_range(i+1..i+2, &sum.to_string()[..]);
    }
    if carry != 0 {
        result.replace_range(0..1, &carry.to_string()[..]);
    }
    if &result[..1] == "0" {
        result = result[1..].to_string();
    }
    result
}
// --------------------------------------------------------------------------------------------------------------------------------------------------


pub fn float_string_is_zero(num: &String) -> bool {
    let (integer, decimal) = integer_and_decimal(num).unwrap();
    is_zeros_string(&integer) && is_zeros_string(&decimal)
}

pub fn integer_and_decimal(num: &String) -> Result<(String, String), ()> {
    let index_dot = float_str_dot_position(num).unwrap();
    if index_dot == 0 {
        return Err(());
    }
    Ok((num[0..index_dot].to_string(), num[index_dot+1..].to_string()))
}

pub fn float_string_without_dot(num: &String) -> Result<String, String> {
    let index_of_point = match float_str_dot_position(num) {
        Some(index) => index,
        None => return Err(String::from("The number is not a float")),
    };
    
    let mut integer = &num[..index_of_point];
    if &integer[..1] == "0" {
        integer = &num[1..index_of_point];
    }
    let decimal = &num[index_of_point + 1..];
    
    Ok(format!("{}{}", integer, decimal))
}

pub fn zeros_string(size: usize) -> String {
    let mut zeros = String::new();
    for _i in 0..size {
        zeros.push('0');
    }
    zeros
}

pub fn size_of_decimal(num: &String) -> usize {
    let mut count = 0;
    let mut dot_passed = false;
    num
        .trim()
        .chars()
        .for_each(|c| {
            if dot_passed {
                count += 1;
            }
            if c == '.' {
                dot_passed = true;
            }
        });
    count
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


*/

/*
pub const E: f64 = 2.71828182845904523536028747135266250f64; // 2.7182818284590451f64

pub fn pade_approximant_log(x: f64, log_base : f64) -> f64 {
    let x2 = x * x;
    let x3 = x2 * x;
    // 3rd order: 
    let numerator = (11.0/27.0) * x3 + x2 - x - (11.0/27.0);
    let denominator = log_base * (x3/9.0 + x2 + x + (1.0/9.0));
    let ln_x = numerator / denominator;
    ln_x
}

pub fn pow(n: f64, x: f64, accuracy: u64) -> f64 {
    let mut accuracy = accuracy;
    let mut result = 1.0;
    let num: f64;
    let is_negative = if x < 0.0 { 
        num = -x;
        true 
    } else {
        num = x;
        false 
    };
    let logn = pade_approximant_log(n, 1.0);
    while accuracy >= 1 {
        result = 1.0 + result * num * logn / (accuracy as f64);
        accuracy -= 1;
    }
    if is_negative {
        1.0 / result    
    } else {
        result
    }
}

fn float_to_string(num: f64, accuracy: u64) -> String {
    let number_without_decimal = num as u64;
    let mut decimal = num - number_without_decimal as f64;
    let mut accuracy = accuracy;
    while accuracy > 0 {
        decimal *= 10.0;
        accuracy -= 1;
    }
    let decimal = decimal as u64;
    format!("{}.{}", number_without_decimal, decimal)
}

fn convert_to_float(number: &String, size: u32) -> f64 {
    let mut number_without_decimal = String::new();
    let mut dot_index = 0;
    let mut i = 0;
    number
        .trim()
        .trim()
        .as_bytes()
        .iter()
        .for_each(|ascii| {
            if *ascii >= 48 && *ascii <= 57 {
                number_without_decimal.push(*ascii as char);
            } else if *ascii == 46 {
                dot_index = i;
            }
            i += 1;
        });
    let divisor = pow10(size as u64 - dot_index) as f64;
    let decimal = number_without_decimal.parse::<u64>().expect("error parsing") as f64;
    decimal / divisor
}

fn pow10(radix: u64) -> u64 {
    let mut r = 1;
    for _i in 0..radix {
        r *= 10;
    }
    r
}
*/


