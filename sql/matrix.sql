-- contents of setup.sql
CREATE OR REPLACE FUNCTION matrix_add(matrix1 text, matrix2 text)
RETURNS text
LANGUAGE c
AS 'libmy_extension.dylib', 'matrix_add_wrapper';

CREATE OR REPLACE FUNCTION matrix_multiply(matrix1 text, matrix2 text)
RETURNS text
LANGUAGE c
AS 'libmy_extension.dylib', 'matrix_multiply_wrapper';

