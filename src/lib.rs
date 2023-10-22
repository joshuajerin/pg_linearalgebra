    extern crate pgrx;
    extern crate ndarray;
    extern crate ndarray_linalg;

    use pgrx::*;
    use ndarray::*;
    use ndarray_linalg::*;


    pg_module_magic!();

    fn matrix_to_array(matrix: Vec<Vec<f64>>) -> Array2<f64> {
        Array2::from_shape_vec((matrix.len(), matrix[0].len()), matrix.into_iter().flatten().collect()).unwrap()
    }

#[pg_extern]
fn matrix_add(matrix1: &str, matrix2: &str, rows: i32, cols: i32) -> Result<String, &'static str> {
    if rows > 5 || cols > 5 {
        return Err("Matrix dimensions should not exceed 5");
    }

    let mat1: Vec<Vec<f64>> = serde_json::from_str(matrix1).unwrap();
    let mat2: Vec<Vec<f64>> = serde_json::from_str(matrix2).unwrap();

    let mut result_matrix = Vec::new();

    for i in 0..rows {
        let mut row = Vec::new();
        for j in 0..cols {
            let value = mat1[i as usize][j as usize] + mat2[i as usize][j as usize];
            row.push(value);
        }
        result_matrix.push(row);
    }

    Ok(format!(
        "[\n {} \n]",
        result_matrix
            .into_iter()
            .map(|row| format!("[{}]", row.into_iter().map(|val| val.to_string()).collect::<Vec<_>>().join(", ")))
            .collect::<Vec<_>>()
            .join(",\n ")
    ))
}

#[pg_extern]
fn matrix_subtract(matrix1: &str, matrix2: &str, rows: i32, cols: i32) -> Result<String, &'static str> {
    if rows > 5 || cols > 5 {
        return Err("Matrix dimensions should not exceed 5");
    }

    let mat1: Vec<Vec<f64>> = serde_json::from_str(matrix1).unwrap();
    let mat2: Vec<Vec<f64>> = serde_json::from_str(matrix2).unwrap();

    let mut result_matrix = Vec::new();

    for i in 0..rows {
        let mut row = Vec::new();
        for j in 0..cols {
            let value = mat1[i as usize][j as usize] - mat2[i as usize][j as usize];
            row.push(value);
        }
        result_matrix.push(row);
    }

    Ok(format!(
        "[\n {} \n]",
        result_matrix
            .into_iter()
            .map(|row| format!("[{}]", row.into_iter().map(|val| val.to_string()).collect::<Vec<_>>().join(", ")))
            .collect::<Vec<_>>()
            .join(",\n ")
    ))
}

#[pg_extern]
fn matrix_multiply(matrix1: &str, matrix2: &str, rows: i32, cols: i32) -> Result<String, &'static str> {
    if rows > 5 || cols > 5 {
        return Err("Matrix dimensions should not exceed 5");
    }

    let mat1: Vec<Vec<f64>> = serde_json::from_str(matrix1).unwrap();
    let mat2: Vec<Vec<f64>> = serde_json::from_str(matrix2).unwrap();

    let mut result_matrix = Vec::new();

    for i in 0..rows {
        let mut row = Vec::new();
        for j in 0..cols {
            let mut value = 0.0;
            for k in 0..cols {
                value += mat1[i as usize][k as usize] * mat2[k as usize][j as usize];
            }
            row.push(value);
        }
        result_matrix.push(row);
    }

    Ok(format!(
        "[\n {} \n]",
        result_matrix
            .into_iter()
            .map(|row| format!("[{}]", row.into_iter().map(|val| val.to_string()).collect::<Vec<_>>().join(", ")))
            .collect::<Vec<_>>()
            .join(",\n ")
    ))
}


#[pg_extern]
fn matrix_transpose(matrix: &str, rows: i32, cols: i32) -> Result<String, &'static str> {
    if rows > 5 || cols > 5 {
        return Err("Matrix dimensions should not exceed 5");
    }

    let mat: Vec<Vec<f64>> = serde_json::from_str(matrix).unwrap();
    let array = matrix_to_array(mat);

    let mut result_matrix = Vec::new();

    for i in 0..cols {
        let mut row = Vec::new();
        for j in 0..rows {
            let value = array[[j as usize, i as usize]];
            row.push(value);
        }
        result_matrix.push(row);
    }

    Ok(format!(
        "[\n {} \n]",
        result_matrix
            .into_iter()
            .map(|row| format!("[{}]", row.into_iter().map(|val| val.to_string()).collect::<Vec<_>>().join(", ")))
            .collect::<Vec<_>>()
            .join(",\n ")
    ))
}

#[pg_extern]
fn matrix_svd(matrix: &str, rows: i32, cols: i32) -> Result<String, &'static str> {
    if rows > 5 || cols > 5 {
        return Err("Matrix dimensions should not exceed 5");
    }
    
    let mat: Vec<Vec<f64>> = serde_json::from_str(matrix).unwrap();
    let a = Array2::from_shape_vec((rows as usize, cols as usize), mat.into_iter().flatten().collect()).unwrap();
    
    let (u, sigma, vt) = a.svd(true, true).unwrap();
    
    Ok(format!(
        "U: {:?} \n
         Sigma: {:?} \n
          V^T: {:?}",
        u, sigma, vt
    ))
}