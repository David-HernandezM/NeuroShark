use gstd::{prelude::*, Vec};

use super::matrix::Matrix;
use super::fraction::Fraction;

pub struct Network {
	pub layers: Vec<usize>,
	pub weights: Vec<Matrix>,
	pub biases: Vec<Matrix>,
	pub data: Vec<Matrix>,
	// learning_rate: Fraction,
}

impl Network {
	pub fn new_doodle_recognition(layers: Vec<u64>) -> Network {
		// let layers = vec![784, 64, 3];

		let layers: Vec<usize> = layers.into_iter()
			.map(|data| data as usize)
			.collect();

		let mut weights = Vec::new();
		let mut biases = Vec::new();

		for _i in 0..layers.len()-1 {
			weights.push(Matrix::default_matrix());
			biases.push(Matrix::default_matrix());
		}

		Network {
			layers,
			weights,
			biases,
			data: vec![],
		}
	}
	
	pub fn new_from(
		layers: Vec<usize>,
		// learning_rate: Fraction,
		biases: Vec<Matrix>,
		weights: Vec<Matrix>,
	) -> Network {
		Network {
			layers,
			weights,
			biases,
			data: vec![],
			// learning_rate,
		}
	}

	pub fn feed_forward(&mut self, inputs: Vec<Fraction>) -> Vec<Fraction> {
		if inputs.len() != self.layers[0] {
			panic!("Invalid inputs length");
		}

		let mut current = Matrix::from(vec![inputs]).transpose();
		self.data = vec![current.clone()];

		for i in 0..self.layers.len() - 1 {
			current = self.weights[i]
				.multiply(&current)
				.add(&self.biases[i])
				.map(&|fraction| {
					let zero = Fraction::new_from_int(0);
					if fraction.gt(&zero) {
						fraction
					} else {
						zero
					}
				});
				// .map(self.activation.function);
			self.data.push(current.clone());
		}

		current.transpose().data[0].to_owned()
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







