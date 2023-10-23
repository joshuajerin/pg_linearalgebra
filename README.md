# pg_linearalgebra

A PostgreSQL extension for basic linear algebra operations.

## Functions

### `matrix_add(matrix1: &str, matrix2: &str, rows: i32, cols: i32) -> Result<String, &'static str>`
Adds two matrices.

#### Example
```sql
SELECT matrix_add('[[1.0, 2.0], [3.0, 4.0]]', '[[5.0, 6.0], [7.0, 8.0]]', 2, 2);
matrix_subtract(matrix1: &str, matrix2: &str, rows: i32, cols: i32) -> Result<String, &'static str>
Subtracts matrix2 from matrix1.

#### Example
sql
Copy code
SELECT matrix_subtract('[[1.0, 2.0], [3.0, 4.0]]', '[[5.0, 6.0], [7.0, 8.0]]', 2, 2);
matrix_multiply(matrix1: &str, matrix2: &str, rows: i32, cols: i32) -> Result<String, &'static str>
Multiplies two matrices.

#### Example
sql
Copy code
SELECT matrix_multiply('[[1.0, 2.0], [3.0, 4.0]]', '[[5.0, 6.0], [7.0, 8.0]]', 2, 2);
matrix_transpose(matrix: &str, rows: i32, cols: i32) -> Result<String, &'static str>
Transposes a matrix.

#### Example
sql
Copy code
SELECT matrix_transpose('[[1.0, 2.0], [3.0, 4.0]]', 2, 2);
matrix_svd(matrix: &str, rows: i32, cols: i32) -> Result<String, &'static str>
Computes the Singular Value Decomposition of a matrix.

#### Example
sql
Copy code
SELECT matrix_svd('[[1.0, 2.0], [3.0, 4.0], [5.0, 6.0]]', 3, 2); give all this in one markdown file
