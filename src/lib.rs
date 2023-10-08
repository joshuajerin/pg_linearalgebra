extern crate pgrx;
extern crate openblas_src;  // link to openblas
extern crate cblas_sys;  // cblas interface
extern crate ndarray;

use pgrx::*;
use ndarray::Array2;
use std::str::FromStr;
use std::num::ParseFloatError;

pg_module_magic!();

// Define a newtype wrapper around Array2<f64>
struct MyArray2(Array2<f64>);

// Implement the FromStr trait for your newtype
impl FromStr for MyArray2 {
    type Err = ParseFloatError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let rows: Result<Vec<Vec<f64>>, ParseFloatError> = s.lines()
            .map(|line| {
                line.split_whitespace()
                    .map(|num| num.parse::<f64>())
                    .collect::<Result<Vec<f64>, ParseFloatError>>()
            })
            .collect::<Result<Vec<Vec<f64>>, ParseFloatError>>();
        match rows {
            Ok(rows) => {
                let flattened: Vec<f64> = rows.into_iter().flatten().collect();
                Ok(MyArray2(Array2::from_shape_vec((flattened.len() / 2, 2), flattened).unwrap()))
            },
            Err(e) => Err(e),
        }
    }
}

#[pg_extern]
fn matrix_add(matrix1: &str, matrix2: &str) -> String {
    let mat1: MyArray2 = matrix1.parse().unwrap_or_else(|_| panic!("Failed to parse matrix1"));
    let mat2: MyArray2 = matrix2.parse().unwrap_or_else(|_| panic!("Failed to parse matrix2"));

    // Perform addition
    let result = mat1.0 + mat2.0;  // Access the inner Array2<f64> via mat1.0 and mat2.0

    // Convert result to string
    result.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(",")
}

#[pg_extern]
fn matrix_multiply(matrix1: &str, matrix2: &str) -> String {
    let mat1: MyArray2 = matrix1.parse().unwrap_or_else(|_| panic!("Failed to parse matrix1"));
    let mat2: MyArray2 = matrix2.parse().unwrap_or_else(|_| panic!("Failed to parse matrix2"));

    // Perform multiplication
    let result = mat1.0.dot(&mat2.0);  // Access the inner Array2<f64> via mat1.0 and mat2.0

    // Convert result to string
    result.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(",")
}
