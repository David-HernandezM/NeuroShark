use gstd::{ prelude::*, Vec };
use crate::{
	multiplication_str::multiply_floats_strings,
	sums_str::add_floats_strings,
	subtraction_str::subtract_floats_strings
};

/*
use crate::{ 
	multiply_floats_strings,
	add_floats_strings,
	subtract_floats_strings
};
*/


#[derive(Encode, Decode, TypeInfo, Clone)]
pub struct Matrix {
	pub rows: u64,
	pub cols: u64,
	pub data: Vec<Vec<String>>,
}

impl Matrix {
	pub fn zeros(rows: u64, cols: u64) -> Matrix {
		Matrix {
			rows,
			cols,
			data: vec![vec![String::from("0.0"); cols as usize]; rows as usize],
		}
	}

	pub fn from(data: Vec<Vec<String>>) -> Matrix {
		Matrix {
			rows: data.len() as u64,
			cols: data[0].len() as u64,
			data,
		}
	}

	pub fn multiply(&self, other: &Matrix) -> Matrix {
		if self.cols != other.rows {
			panic!("Attempted to multiply by matrix of incorrect dimensions");
		}

		let mut res = Matrix::zeros(self.rows, other.cols);

		for i in 0..self.rows as usize {
			for j in 0..other.cols as usize {
				let mut sum = String::from("0.0");
				for k in 0..self.cols as usize {
					let multiply_result = multiply_floats_strings(
						&self.data[i][k],
						&other.data[k][j]
					).unwrap();
					sum = add_floats_strings(&sum, &multiply_result);
				}

				res.data[i][j] = sum;
			}
		}
		
		res
	}

	pub fn add(&self, other: &Matrix) -> Matrix {
		if self.rows != other.rows || self.cols != other.cols {
			panic!("Attempted to add matrix of incorrect dimensions");
		}

		let mut res = Matrix::zeros(self.rows, self.cols);

		for i in 0..self.rows as usize {
			for j in 0..self.cols as usize {
				res.data[i][j] = add_floats_strings(
					&self.data[i][j],
					&other.data[i][j]
				);
			}
		}
		
		res
	}

	pub fn dot_multiply(&self, other: &Matrix) -> Matrix {
		if self.rows != other.rows || self.cols != other.cols {
			panic!("Attempted to dot multiply by matrix of incorrect dimensions");
		}

		let mut res = Matrix::zeros(self.rows, self.cols);

		for i in 0..self.rows as usize {
			for j in 0..self.cols as usize {
				res.data[i][j] = multiply_floats_strings(
					&self.data[i][j], 
					&other.data[i][j]
				).unwrap();
			}
		}

		res
	}

	pub fn subtract(&self, other: &Matrix) -> Matrix {
		if self.rows != other.rows || self.cols != other.cols {
			panic!("Attempted to subtract matrix of incorrect dimensions");
		}

		let mut res = Matrix::zeros(self.rows, self.cols);

		for i in 0..self.rows as usize {
			for j in 0..self.cols as usize {
				res.data[i][j] = subtract_floats_strings(
					&self.data[i][j],
					&other.data[i][j]
				);
			}
		}

		res
	}

	pub fn map(&self, function: &dyn Fn(String) -> String) -> Matrix {
		Matrix::from(
			(self.data)
				.clone()
				.into_iter()
				.map(|row| row.into_iter().map(|value| function(value)).collect())
				.collect(),
		)
	}

	pub fn transpose(&self) -> Matrix {
		let mut res = Matrix::zeros(self.cols, self.rows);

		for i in 0..self.rows as usize {
			for j in 0..self.cols as usize {
				res.data[j][i] = self.data[i][j].clone();
			}
		}

		res
	}
	
	pub fn to_str(&self) -> String {
		let mut data_str = String::from("    vec![\n");
		self.data.iter()
			.for_each(|data| {
				data_str.push_str("        vec![\n");
				data.iter()
					.for_each(|num| {
						data_str.push_str(&format!("            String::from(\"{}\"),\n", num)[..]);
					});
				data_str.push_str("        ],\n");
			});
		data_str.push_str("    ],\n");
		data_str
		
	}
}