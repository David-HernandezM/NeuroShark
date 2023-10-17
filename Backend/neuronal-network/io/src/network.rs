use gstd::{prelude::*, Vec};
use crate::matrix::Matrix;
use crate::{
	division_str::floats_division_strings,
	multiplication_str::multiply_floats_strings,
	sums_str::add_floats_strings,
	exponentiation_str::float_string_euler_exponentiation
};

/*
use crate::{
    add_floats_strings,
    floats_division_strings,
    float_string_euler_exponentiation,
    subtract_floats_strings,
    multiply_floats_strings
};
*/

#[derive(Encode, Decode, TypeInfo)]
pub struct Network {
	pub layers: Vec<u64>,
	pub weights: Vec<Matrix>,
	pub biases: Vec<Matrix>,
	pub data: Vec<Matrix>,
	pub learning_rate: String,
}

impl Network {
	pub fn new_from_trained_data(
		layers: Vec<u64>,
		weights: Vec<Matrix>,
		biases: Vec<Matrix>,
		learning_rate: String
	) -> Network {
		Network {
			layers,
			weights,
			biases,
			data: vec![],
			learning_rate,
		}
	}
	
	pub fn new(
	    layers: Vec<u64>,
	    weights: Vec<Vec<Vec<String>>>,
	    biases: Vec<Vec<Vec<String>>>,
	    learning_rate: String,
	) -> Network {
	    let weights = weights
            .into_iter()
            .map(|bias| {
                Matrix::from(bias)
            })
            .collect();
        let biases = biases
            .into_iter()
            .map(|bias| {
                Matrix::from(bias)
            })
            .collect();
        Network {
            layers,
            weights,
            biases,
            data: vec![],
            learning_rate
        }
	}
	
	pub fn default_new() -> Network {
	    Network {
	       layers: vec![],
	       weights: vec![],
	       biases: vec![],
	       data: vec![],
	       learning_rate: String::from("0.0"),
	    }
	}

	pub fn feed_forward(&mut self, inputs: Vec<String>) -> Vec<String> {
		if inputs.len() as u64 != self.layers[0] {
			panic!("Invalid inputs length");
		}

		let mut current = Matrix::from(vec![inputs]).transpose();
		self.data = vec![current.clone()];

		for i in 0..self.layers.len() - 1 {
			current = self.weights[i]
				.multiply(&current)
				.add(&self.biases[i])
				.map(&|x| {
				    // sigmoid function
				    let exponent = multiply_floats_strings(
				        &String::from("-1.0"),
				        &x
				    ).unwrap();
				    let euler_exponent = float_string_euler_exponentiation(&exponent);
				    let euler_exponent_plus_one = add_floats_strings(
				        &String::from("1.0"),
				        &euler_exponent
				    );
				    let result = floats_division_strings(&String::from("1.0"), &euler_exponent_plus_one);
				    
				    result
				});
			self.data.push(current.clone());
		}

		current
			.transpose()
			.data[0]
			.to_owned()
	}
	
	pub fn to_str(&self) -> String {
		let mut data_str = String::from("Neuronal Network data:\n");
		data_str.push_str("Layers: \n");
		data_str.push_str(&format!("{:?}\n", self.layers)[..]);
		data_str.push_str("Weights:\nvec![\n");
		self.weights.iter()
			.for_each(|matrix| data_str.push_str(&(matrix.to_str())[..]));
		data_str.push_str("];\n");
		data_str.push_str("Biases:\n");
		data_str.push_str("vec![\n");
		self.biases.iter()
			.for_each(|matrix| data_str.push_str(&(matrix.to_str())[..]));
		data_str.push_str("];\n");
		
		data_str
	}
}




/*
pub fn float_to_string(value: f64) -> String {
    let mut number = value;
    let mut parsed_float = String::new();
    
    // If value is 0
    if number == 0.0 {
        return String::from("0.0");
    }
    
    // Check if value is negative
    if (number < 0.0) {
        number = (-number);
    }
    
    // Get integer
    let integer = number as u64;
    parsed_float.push_str(String::from(integer));
    
    loop {
        println!("{x}");
        x *= 10.0;
        if x - (x as u64) as f64 == 0.0 {
            break;
        }
    }
}


    // Manejo de la parte fraccional
    float fractionalPart = value - static_cast<float>(static_cast<int>(value));
    if (fractionalPart > 0) {
        result += '.';
        const int maxDecimalPlaces = 6; // Número máximo de decimales
        int decimalPlaces = 0;

        while (decimalPlaces < maxDecimalPlaces && fractionalPart > 0) {
            fractionalPart *= 10;
            result += static_cast<char>('0' + static_cast<int>(fractionalPart));
            fractionalPart -= static_cast<int>(fractionalPart);
            decimalPlaces++;
        }
    }

    // Agregar el signo si es negativo
    if (isNegative) {
        result = '-' + result;
    }

    return result;
}

int main() {
    float floatValue = 3.14159; // El valor float que deseas convertir a string

    std::string strValue = floatToString(floatValue);

    std::cout << "El valor como string es: " << strValue << std::endl;

    return 0;
}
*/





/* String to float
#include <iostream>
#include <string>
using namespace std;

int main() {
    string str = "3.14159"; // El string que deseas convertir a float
    
    float floatValue = 0.0f;
    int decimalPlaces = 0;
    bool isNegative = false;
    bool hasDecimal = false;

    for (char c : str) {
        if (c == '-') {
            isNegative = true;
        } else if (c == '.') {
            hasDecimal = true;
        } else if (c >= '0' && c <= '9') {
            if (hasDecimal) {
                floatValue = floatValue * 10 + (c - '0');
                decimalPlaces++;
            } else {
                floatValue = floatValue * 10 + (c - '0');
            }
        } else {
            // Carácter no válido
            cerr << "El string contiene caracteres no válidos." << std::endl;
            return 1;
        }
    }

    if (isNegative) {
        floatValue = -floatValue;
    }

    // Ajustar el valor para tener en cuenta la posición decimal
    for (int i = 0; i < decimalPlaces; i++) {
        floatValue /= 10.0f;
    }

    std::cout << "El valor float es: " << floatValue << std::endl;

    return 0;
}
*/