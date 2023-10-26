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
            	   String::from("-10.067806895028"),
            	   String::from("-10.069251807067"),
        	   ],
        	   vec![
            	   String::from("-9.382214114334"),
            	   String::from("-9.383488644351"),
        	   ],
        	   vec![
            	   String::from("-10.195950770812"),
            	   String::from("-10.195385621153"),
        	   ],
    	   ]),
    	   Matrix::from(vec![
        	   vec![
            	   String::from("-5.203677476184"),
            	   String::from("-4.022237746981"),
            	   String::from("8.569942138251"),
        	   ],
    	   ]),
	   ];
	   
	   let biases = vec![
    	   Matrix::from(vec![
        	   vec![
            	   String::from("4.457786935168"),
        	   ],
        	   vec![
            	   String::from("4.111367041253"),
        	   ],
        	   vec![
            	   String::from("15.282025368793"),
        	   ],
    	   ]),
    	   Matrix::from(vec![
        	   vec![
            	   String::from("-4.083288690267"),
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