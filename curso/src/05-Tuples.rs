// Las tuplas se pueden utilizar como argumentos de funciones y como valores de retorno.
fn reverse(pair: (i32, bool)) -> (bool, i32){
    // `let` se puede utilizar para vincular los miembros de una tupla a variables.
    let (int_param, bool_param) = pair;

    (bool_param, int_param)
}

// La siguiente estructura es para la actividad.
#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

fn main(){
    // Una tupla con varios tipos diferentes.
    let long_tuple = (1u8, 2u16, 3u32, 4u64,
        -1i8, -2i16, -3i32, -4i64,
        0.1f32, 0.2f64,
        'a', true
    );
    
    // Los valores pueden extraerse de la tupla utilizando la indexación de tuplas.
    println!("Long tuple first value: {}", long_tuple.0);
    println!("Long tuple second value: {}", long_tuple.1);

    // Las tuplas pueden ser miembros de tuplas.
    let tuple_of_tuples = ((1u8, 2u16, 2u32), (4u64, -1i8), -2i16);

    // Las tuplas son imprimibles.
    println!("Tuple of tuples: {:?}", tuple_of_tuples);

    // Pero las tuplas largas (más de 12 elementos) no se pueden imprimir 
    //let tupla_demasiado_larga = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13); 
    //println!("Tupla demasiado larga: {:?}", tupla_demasiado_larga); 
    // TODO ^ Descomenta las 2 líneas anteriores para ver el error del compilador

    let pair = (1, true);
    println!("Pair is {:?}", pair);

    println!("The reverse pair is {:?}", reverse(pair));

    // Para crear tuplas de un elemento, la coma es necesaria para distinguirlas 
    // de un literal rodeado de paréntesis.
    println!("One element tuple: {:?}", (5u32,));
    println!("Just an integer: {:?}", (5u32));

    // Las tuplas se pueden desestructurar para crear enlaces.
    let tuple = (1, "hello", 4.5, true);

    let (a, b, c, d) = tuple;
    println!("{:?}, {:?}, {:?}, {:?}", a, b, c, d);

    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    println!("{:?}", matrix);
}