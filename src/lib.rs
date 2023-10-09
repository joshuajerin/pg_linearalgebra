extern crate pgrx;
extern crate openblas_src;  // link to openblas
extern crate cblas_sys;  // cblas interface
extern crate ndarray;
extern crate serde;
extern crate serde_json;

use pgrx::*;
use ndarray::Array2;
use serde::{Deserialize, Deserializer};
use std::fmt;
use std::error::Error as StdError;
use std::str::FromStr;  // Import the FromStr trait
use serde_json::Error as SerdeJsonError; 

pg_module_magic!();

// Define a newtype wrapper around Array2<f64>
struct MyArray2(Array2<f64>);

// Define a custom error type:
#[derive(Debug)]
struct MyError(String);

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl StdError for MyError {}

impl serde::de::Error for MyError {
    fn custom<T: fmt::Display>(msg: T) -> Self {
        MyError(msg.to_string())
    }
}

// Implement the FromStr trait for MyArray2
impl FromStr for MyArray2 {
    type Err = MyError;  // Use MyError as the error type

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Parse the JSON string directly into a nested Vec<f64>
        let nested_vec: Vec<Vec<f64>> = serde_json::from_str(s)
            .map_err(|_| MyError("Failed to parse JSON".to_string()))?;
        
        // Convert the nested Vec<f64> into a flat Vec<f64> and get the dimensions
        let rows = nested_vec.len();
        let cols = nested_vec[0].len();
        let flat_vec: Vec<f64> = nested_vec.into_iter().flatten().collect();
        
        // Create the Array2<f64> from the flat Vec<f64>
        let array = Array2::from_shape_vec((rows, cols), flat_vec)
            .map_err(|_| MyError("Invalid matrix dimensions".to_string()))?;
        
        Ok(MyArray2(array))
    }
}

impl MyArray2 {
    fn from_str<'de, D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        // ... (rest of your existing code for from_str)
        let s = String::deserialize(deserializer)?;
        let vals: serde_json::Value = serde_json::from_str(&s).map_err(serde::de::Error::custom)?;

        if let Some(outer) = vals.as_array() {
            let mut rows = Vec::new();
            for val in outer {
                if let Some(inner) = val.as_array() {
                    let row: Vec<f64> = inner.iter().map(|v| v.as_f64().unwrap_or(0.0)).collect();
                    rows.push(row);
                } else {
                    return Err(serde::de::Error::custom("Expected inner array"));
                }
            }
            let array = Array2::from_shape_vec((rows.len(), rows[0].len()), rows.into_iter().flatten().collect())
                .map_err(|_| serde::de::Error::custom("Invalid matrix dimensions"))?;
            Ok(MyArray2(array))
        } else {
            Err(serde::de::Error::custom("Expected outer array"))
        }
    }
}

fn array_to_nested_vec(array: Array2<f64>) -> Vec<Vec<f64>> {
    array
        .map_axis(ndarray::Axis(0), |row| row.to_vec())
        .to_vec()
}

fn format_matrix(matrix: Vec<Vec<f64>>) -> String {
    println!("Entering format_matrix");  // Debugging statement
    let mut formatted = String::new();
    formatted.push_str("[\n");
    for row in matrix {
        formatted.push_str(" [");
        for value in row {
            formatted.push_str(&format!("{:.1}, ", value));  // Adjusts to 1 decimal place; adjust as needed.
        }
        // Remove trailing comma and space from the row
        let len = formatted.len();
        formatted.truncate(len - 2);
        formatted.push_str("],\n");
    }
    // Remove trailing comma and newline from the last row
    let len = formatted.len();
    formatted.truncate(len - 2);
    formatted.push_str("\n]");
    println!("Exiting format_matrix");  // Debugging statement
    formatted
}

#[pg_extern]
fn matrix_add(matrix1: &str, matrix2: &str, rows: i32, cols: i32) -> Result<String, &'static str> {
    if rows > 5 || cols > 5 {
        return Err("Matrix dimensions should not exceed 5");
    }
    // Parse the matrix strings into MyArray2 instances
    let mat1: MyArray2 = matrix1.parse().unwrap_or_else(|_| panic!("Failed to parse matrix1"));
    let mat2: MyArray2 = matrix2.parse().unwrap_or_else(|_| panic!("Failed to parse matrix2"));

    // Perform addition
    let result = mat1.0 + mat2.0;  // Access the inner Array2<f64> via mat1.0 and mat2.0

    // Convert result to nested Vec and then to JSON string
    let nested_vec = array_to_nested_vec(result);
    println!("Calling format_matrix from matrix_add");  // Debugging statement
    Ok(format_matrix(nested_vec))
}


#[pg_extern]
fn matrix_multiply(matrix1: &str, matrix2: &str, rows: i32, cols: i32) -> Result<String, &'static str> {
    if rows > 5 || cols > 5 {
        return Err("Matrix dimensions should not exceed 5");
    }
    let mat1: MyArray2 = matrix1.parse().unwrap_or_else(|_| panic!("Failed to parse matrix1"));
    let mat2: MyArray2 = matrix2.parse().unwrap_or_else(|_| panic!("Failed to parse matrix2"));

    // Perform multiplication
    let result = mat1.0.dot(&mat2.0);  // Access the inner Array2<f64> via mat1.0 and mat2.0

    // Convert result to nested Vec and then to JSON string
    let nested_vec = array_to_nested_vec(result);
    Ok(format_matrix(nested_vec))
}



