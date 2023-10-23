# pg_linearalgebra

A PostgreSQL extension for basic linear algebra operations.

## Functions

### `mAdd(matrix1: &str, matrix2: &str, rows: i32, cols: i32) -> Result<String, &'static str>`
Adds two matrices.

#### Example
```sql
SELECT mAdd('[[1.0, 2.0], [3.0, 4.0]]', '[[5.0, 6.0], [7.0, 8.0]]', 2, 2);
```
<br>


### `mSubtract(matrix1: &str, matrix2: &str, rows: i32, cols: i32) -> Result<String, &'static str>`
Subtracts matrix2 from matrix1.

#### Example
```sql
SELECT mSubtract('[[1.0, 2.0], [3.0, 4.0]]', '[[5.0, 6.0], [7.0, 8.0]]', 2, 2);
```
<br>


### `mMultiply(matrix1: &str, matrix2: &str, rows: i32, cols: i32) -> Result<String, &'static str>`
Multiplies two matrices.

#### Example
```sql
SELECT mMultiply('[[1.0, 2.0], [3.0, 4.0]]', '[[5.0, 6.0], [7.0, 8.0]]', 2, 2);
```
<br>


### `mTranspose(matrix: &str, rows: i32, cols: i32) -> Result<String, &'static str>`
Transposes a matrix.

#### Example
```sql
SELECT mTranspose('[[1.0, 2.0], [3.0, 4.0]]', 2, 2);
```
<br>

### `mSvd(matrix: &str, rows: i32, cols: i32) -> Result<String, &'static str>`
Computes the Singular Value Decomposition of a matrix.

#### Example
```sql
SELECT mSvd('[[1.0, 2.0], [3.0, 4.0], [5.0, 6.0]]', 3, 2);
```
