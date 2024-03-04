use gstd::{ prelude::*, Vec };
use crate::fraction::Fraction;

// #[derive(Encode, Decode, TypeInfo, Clone)]
// #[codec(crate = gstd::codec)]
// #[scale_info(crate = gstd::scale_info)]

#[derive(Clone)]
pub struct Matrix {
	pub rows: usize,
	pub cols: usize,
	pub data: Vec<Vec<Fraction>>,
}

impl Matrix {
	pub fn zeros(rows: usize, cols: usize) -> Matrix {
		// let mut cols_data = Vec::new();
		// let mut rows_data = Vec::new();

		// for _i in 0..cols {
		// 	cols_data.push(Fraction::new_from_int(0));
		// }
		
		// for i in 0..rows {
		// 	rows_data.push(cols_data.clone());
		// }

		let data = vec![vec![Fraction::new_from_int(0); cols]; rows];

		Matrix {
			rows,
			cols,
			data
		}
	}

	pub fn from(data: Vec<Vec<Fraction>>) -> Matrix {
		Matrix {
			rows: data.len(),
			cols: data[0].len(),
			data,
		}
	}

	pub fn multiply(&self, other: &Matrix) -> Matrix {
		if self.cols != other.rows {
			panic!("Attempted to multiply by matrix of incorrect dimensions");
		}

		let mut res = Matrix::zeros(self.rows, other.cols);

		for i in 0..self.rows {
			for j in 0..other.cols {
				// let mut sum = 0.0;
				let mut sum = Fraction::new_from_int(0);
				for k in 0..self.cols {
					let temp = self.data[i][k].mult(&other.data[k][j]);
					sum.add_self(&temp);
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

		for i in 0..self.rows {
			for j in 0..self.cols {
				res.data[i][j] = self.data[i][j].add(&other.data[i][j]);
			}
		}

		res
	}

	pub fn dot_multiply(&self, other: &Matrix) -> Matrix {
		if self.rows != other.rows || self.cols != other.cols {
			panic!("Attempted to dot multiply by matrix of incorrect dimensions");
		}

		let mut res = Matrix::zeros(self.rows, self.cols);

		for i in 0..self.rows {
			for j in 0..self.cols {
				res.data[i][j] = self.data[i][j].mult(&other.data[i][j]);
			}
		}

		res
	}

	pub fn subtract(&self, other: &Matrix) -> Matrix {
		if self.rows != other.rows || self.cols != other.cols {
			panic!("Attempted to subtract matrix of incorrect dimensions");
		}

		let mut res = Matrix::zeros(self.rows, self.cols);

		for i in 0..self.rows {
			for j in 0..self.cols {
				// res.data[i][j] = self.data[i][j] - other.data[i][j];
				res.data[i][j] = self.data[i][j].sub(&other.data[i][j]);
			}
		}

		res
	}

	pub fn map(&self, function: &dyn Fn(Fraction) -> Fraction) -> Matrix {
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

		for i in 0..self.rows {
			for j in 0..self.cols {
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
						data_str.push_str(&format!("            String::from(\"{:?}\"),\n", num)[..]);
					});
				data_str.push_str("        ],\n");
			});
		data_str.push_str("    ],\n");
		data_str
		
	}
}