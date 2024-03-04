use gstd::{prelude::*, Vec};

use super::matrix::Matrix;
use super::fraction::Fraction;

const DIVISOR: i128 = 1000000000000;

// #[derive(Encode, Decode, TypeInfo)]
// #[codec(crate = gstd::codec)]
// #[scale_info(crate = gstd::scale_info)]

pub struct Network {
	layers: Vec<usize>,
	weights: Vec<Matrix>,
	biases: Vec<Matrix>,
	data: Vec<Matrix>,
	// learning_rate: Fraction,
}

impl Network {
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

	pub fn new_xor_gate() -> Network {
		// let learning_rate = Fraction::new(1, 100);
		let layers = vec![2, 3, 1];

		let weights = vec![
			Matrix::from(vec![
				vec![
					Fraction::new(-637862520340/*5481*/, DIVISOR), 
					Fraction::new(-726105865742/*8923*/, DIVISOR)
				],
				vec![
					Fraction::new(-867868485952/*194*/, DIVISOR), 
					Fraction::new(-867657923674/*3467*/, DIVISOR)
				],
				vec![
					Fraction::new(1467306554884/*3083*/, DIVISOR), 
					Fraction::new(1462573787091/*9802*/, DIVISOR)
				],
			]),
			Matrix::from(vec![
				vec![
					Fraction::new(-1093371873764/*7682*/, DIVISOR), 
					Fraction::new(-1332154117654/*006*/, DIVISOR), 
					Fraction::new(-620115053970/*6846*/, DIVISOR), 
				]
			])
		];

		let biases = vec![
			Matrix::from(vec![
				vec![
					Fraction::new(637532916463/*3726*/, DIVISOR),
				],
				vec![
					Fraction::new(866931126590/*1329*/, DIVISOR),
				],
				vec![
					Fraction::new(1171211727170/*8187*/, DIVISOR),
				]
			]),
			Matrix::from(vec![
				vec![
					Fraction::new(2588727563216/*845*/, DIVISOR),
				]
			])
		];

		Network {
			layers,
			weights,
			biases,
			data: vec![],
			// learning_rate
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







