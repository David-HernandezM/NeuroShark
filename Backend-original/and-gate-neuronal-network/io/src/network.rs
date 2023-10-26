use gstd::{prelude::*, Vec};
use crate::matrix::Matrix;
use crate::{
	division_str::floats_division_strings,
	multiplication_str::multiply_floats_strings,
	sums_str::add_floats_strings,
	exponentiation_str::float_string_euler_exponentiation
};

#[derive(Encode, Decode, TypeInfo)]
pub struct Network {
	pub layers: Vec<u64>,
	pub weights: Vec<Matrix>,
	pub biases: Vec<Matrix>,
	pub data: Vec<Matrix>,
	pub learning_rate: String,
}

impl Network {
	pub fn new() -> Network {
	    let learning_rate = String::from("0.5");
        let layers: Vec<u64> = vec![2, 3, 1];
        
        let weights = vec![
    		Matrix::from(vec![
        		vec![
            		String::from("9.83822265449708"),
            		String::from("-11.94074365115927"),
        		],
        		vec![
            		String::from("12.05750662273592"),
            		String::from("-9.99255229163805"),
        		],
        		vec![
            		String::from("7.39251807316976"),
            		String::from("7.49427173405531"),
        		],
    		]),
    		Matrix::from(vec![
        		vec![
            		String::from("8.20040221692707"),
            		String::from("-8.08491186961818"),
            		String::from("2.52342877357193"),
        		],
    		]),
		];

		let biases = vec![
    		Matrix::from(vec![
        		vec![
            		String::from("-4.31939271794314"),
        		],
        		vec![
            		String::from("4.35532533315800"),
        		],
        		vec![
            		String::from("-2.29736842379774"),
        		],
    		]),
    		Matrix::from(vec![
        		vec![
            		String::from("1.64582043745778"),
        		],
    		]),
		];
	   
        Self {
            layers,
            weights,
            biases,
            data: vec![],
            learning_rate
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