/*
Añade una función de transposición utilizando la función 
inversa como plantilla, que acepta una matriz como argumento 
y devuelve una matriz en la que se han intercambiado dos elementos. 
Por ejemplo:

println!("Matrix:\n{}", matrix);
println!("Transpose:\n{}", transpose(matrix));

Resultados en la salida:

Matrix:
( 1.1 1.2 )
( 2.1 2.2 )
Transpose:
( 1.1 2.1 )
( 1.2 2.2 )
*/

use std::fmt;

struct Matrix(f32,f32,f32,f32);

impl fmt::Display for Matrix{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({} {})\n({} {})", self.0, self.1, self.2, self.3)
    }
}

// Las tuplas se pueden utilizar como argumentos de funciones y como valores de retorno.
fn transpose(matrix: Matrix) -> Matrix{
    // `let` se puede utilizar para vincular los miembros de una tupla a variables.
    let rev = Matrix(matrix.0, matrix.2, matrix.1, matrix.3);

    rev // Return
}

fn main(){
    let matrix = Matrix(1.1,1.2,2.1,2.2);

    println!("Matrix:\n{}", matrix);
    println!("Transpose:\n{}", transpose(matrix));
}