-- contents of setup.sql
CREATE EXTENSION IF NOT EXISTS my_extension;

CREATE OR REPLACE FUNCTION matrix_add(matrix1 text, matrix2 text, rows integer, cols integer)
RETURNS text
LANGUAGE c
AS 'libmy_extension.dylib', 'matrix_add_wrapper';

CREATE OR REPLACE FUNCTION matrix_multiply(matrix1 text, matrix2 text, rows integer, cols integer)
RETURNS text
LANGUAGE c
AS 'libmy_extension.dylib', 'matrix_multiply_wrapper';
