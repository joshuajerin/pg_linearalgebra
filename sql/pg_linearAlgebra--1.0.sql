CREATE EXTENSION IF NOT EXISTS pg_linearalgebra;

CREATE OR REPLACE FUNCTION mAdd(matrix1 text, matrix2 text, rows integer, cols integer)
RETURNS text
LANGUAGE c
AS 'libpg_linearalgebra.dylib', 'matrix_add_wrapper';

CREATE OR REPLACE FUNCTION mSubtract(matrix1 text, matrix2 text, rows integer, cols integer)
RETURNS text
LANGUAGE c
AS 'libpg_linearalgebra.dylib', 'matrix_subtract_wrapper';

CREATE OR REPLACE FUNCTION mMultiply(matrix1 text, matrix2 text, rows integer, cols integer)
RETURNS text
LANGUAGE c
AS 'libpg_linearalgebra.dylib', 'matrix_multiply_wrapper';

CREATE OR REPLACE FUNCTION mTranspose(matrix text, rows integer, cols integer)
RETURNS text
LANGUAGE c
AS 'libpg_linearalgebra.dylib', 'matrix_transpose_wrapper';

CREATE OR REPLACE FUNCTION mSvd(matrix text, rows integer, cols integer)
RETURNS text
LANGUAGE c
AS 'libpg_linearalgebra.dylib', 'matrix_svd_wrapper';
