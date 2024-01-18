/*
Recapitulemos: Añada el rasgo fmt::Display a la estructura 
Matrix en el ejemplo anterior, de modo que si cambia de imprimir 
el formato de depuración {:?} al formato de visualización {}, 
verá la siguiente salida:

( 1.1 1.2 )
( 2.1 2.2 )
*/

// Importa (mediante `use`) el módulo `fmt` para que esté disponible.
use std::fmt;

/*
Define una estructura para la que se implementará `fmt::Display`. Esta es
una estructura tupla llamada `Matrix` que contiene 4 `f32`.
*/
struct Matrix(f32,f32,f32,f32);

/*
Para usar el marcador `{}`, el trait `fmt::Display` debe ser implementado
manualmente para el tipo.
*/
impl fmt::Display for Matrix{
    // Este rasgo requiere `fmt` con esta firma exacta.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        /*
        Escribe estrictamente los elementos en la salida suministrada
        flujo: `f`. Devuelve `fmt::Result` que indica si la operación
        tuvo éxito o falló. Tenga en cuenta que `write!` utiliza una sintaxis que
        es muy similar a `println!`.
        */
        write!(f, "({} {})\n({} {})", self.0, self.1, self.2, self.3)
    }
}

fn main(){
    let matrix = Matrix(1.1,1.2,2.1,2.2);
    println!("{}", matrix);
}