CREATE EXTENSION IF NOT EXISTS pg_linearalgebra;

CREATE OR REPLACE FUNCTION matrix_add(matrix1 text, matrix2 text, rows integer, cols integer)
RETURNS text
LANGUAGE c
AS 'libpg_linearAlgebra.dylib', 'matrix_add_wrapper';

CREATE OR REPLACE FUNCTION matrix_subtract(matrix1 text, matrix2 text, rows integer, cols integer)
RETURNS text
LANGUAGE c
AS 'libpg_linearAlgebra.dylib', 'matrix_subtract_wrapper';

CREATE OR REPLACE FUNCTION matrix_multiply(matrix1 text, matrix2 text, rows integer, cols integer)
RETURNS text
LANGUAGE c
AS 'libpg_linearAlgebra.dylib', 'matrix_multiply_wrapper';

CREATE OR REPLACE FUNCTION matrix_transpose(matrix text, rows integer, cols integer)
RETURNS text
LANGUAGE c
AS 'libpg_linearAlgebra.dylib', 'matrix_transpose_wrapper';

CREATE OR REPLACE FUNCTION matrix_svd(matrix text, rows integer, cols integer)
RETURNS text
LANGUAGE c
AS 'libpg_linearAlgebra.dylib', 'matrix_svd_wrapper';
