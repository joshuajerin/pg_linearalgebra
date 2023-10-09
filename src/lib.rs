extern crate pgrx;
extern crate openblas_src;  // link to openblas
extern crate cblas_sys;  // cblas interface
extern crate serde;
extern crate serde_json;

use pgrx::*;
use cblas_sys::*;
use serde_json::Error as SerdeJsonError;
use std::ptr;

pg_module_magic!();

fn matrix_to_raw(matrix: Vec<Vec<f64>>) -> (*mut f64, i32, i32) {
    let rows = matrix.len();
    let cols = matrix[0].len();
    let flat: Vec<f64> = matrix.into_iter().flatten().collect();
    let raw_ptr = flat.as_ptr() as *mut f64;
    // Ensure the vector isn't deallocated
    std::mem::forget(flat);
    (raw_ptr, rows as i32, cols as i32)
}

fn raw_to_matrix(ptr: *mut f64, rows: i32, cols: i32) -> Vec<Vec<f64>> {
    let mut matrix = Vec::new();
    for i in 0..rows {
        let mut row = Vec::new();
        for j in 0..cols {
            unsafe {
                let value = *ptr.offset((i * cols + j) as isize);
                row.push(value);
            }
        }
        matrix.push(row);
    }
    unsafe { libc::free(ptr as *mut libc::c_void) };
    matrix
}

#[pg_extern]
fn matrix_add(matrix1: &str, matrix2: &str, rows: i32, cols: i32) -> Result<String, &'static str> {
    if rows > 5 || cols > 5 {
        return Err("Matrix dimensions should not exceed 5");
    }

    let mat1: Vec<Vec<f64>> = serde_json::from_str(matrix1).unwrap();
    let mat2: Vec<Vec<f64>> = serde_json::from_str(matrix2).unwrap();
    
    let (ptr1, rows1, cols1) = matrix_to_raw(mat1);
    let (ptr2, _, _) = matrix_to_raw(mat2);

    let mut result_vec = vec![0.0; (rows1 * cols1) as usize];
    let result_ptr = result_vec.as_mut_ptr();

    for i in 0..(rows1 * cols1) {
        unsafe {
            *result_ptr.offset(i as isize) = *ptr1.offset(i as isize) + *ptr2.offset(i as isize);
        }
    }

    unsafe { libc::free(ptr1 as *mut libc::c_void) };
    unsafe { libc::free(ptr2 as *mut libc::c_void) };

    let result_matrix = raw_to_matrix(result_ptr, rows1, cols1);

    Ok(serde_json::to_string(&result_matrix).unwrap())
}

#[pg_extern]
fn matrix_multiply(matrix1: &str, matrix2: &str, rows: i32, cols: i32) -> Result<String, &'static str> {
    if rows > 5 || cols > 5 {
        return Err("Matrix dimensions should not exceed 5");
    }

    let mat1: Vec<Vec<f64>> = serde_json::from_str(matrix1).unwrap();
    let mat2: Vec<Vec<f64>> = serde_json::from_str(matrix2).unwrap();

    let (ptr1, rows1, cols1) = matrix_to_raw(mat1);
    let (ptr2, _, cols2) = matrix_to_raw(mat2);

    let mut result_vec = vec![0.0; (rows1 * cols2) as usize];
    let result_ptr = result_vec.as_mut_ptr();

    unsafe {
        cblas_dgemm(
            CblasRowMajor,
            CblasNoTrans,
            CblasNoTrans,
            rows1,
            cols2,
            cols1,
            1.0,
            ptr1,
            cols1,
            ptr2,
            cols2,
            0.0,
            result_ptr,
            cols2,
        );
    }

    unsafe { libc::free(ptr1 as *mut libc::c_void) };
    unsafe { libc::free(ptr2 as *mut libc::c_void) };

    let result_matrix = raw_to_matrix(result_ptr, rows1, cols2);

    Ok(serde_json::to_string(&result_matrix).unwrap())
}
