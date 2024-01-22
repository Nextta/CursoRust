use std::mem;

// Esta función toma prestada una slice.
fn analyze_slice(slice: &[i32]){
    println!("First element of the slice: {}", slice[0]);
    println!("The slice has {} elements", slice.len());
}

fn main(){
    // Array de tamaño fijo (la firma de tipo es superflua).
    let xs: [i32; 5] = [1,2,3,4,5];

    // Todos los elementos pueden inicializarse con el mismo valor.
    let ys: [i32; 500] = [0;500];

    // La indexación comienza en 0.
    println!("First element of the array: {}", xs[0]);
    println!("Second element of the array: {}", xs[1]);

    // `len` devuelve la cuenta de elementos en el array.
    println!("Number of elements in array: {}", xs.len());

    // El array se asignan a la pila.
    println!("Array occupies {} bytes", mem::size_of_val(&xs));

    // Los arrays pueden ser prestados automáticamente como slices.
    println!("Borrow the whole array as a slice.");
    analyze_slice(&xs);

    // Los slices pueden apuntar a una sección de un array. 
    // Son de la forma [starting_index..ending_index]. 
    // `starting_index` es la primera posición en el slice. 
    // `ending_index` es una más que la última posición en el slice.
    println!("Borrow a section of the array as a slice.");
    analyze_slice(&ys[1..4]);

    // Ejemplo de slice vacío `&[]`:
    let empty_array: [u32; 0] = [];
    assert_eq!(&empty_array, &[]);
    assert_eq!(&empty_array, &[][..]); // Igual pero más verboso

    /*
    Se puede acceder a los arrays de forma segura usando `.get`, que devuelve una 
    `Opción`. Esto puede ser emparejado como se muestra a continuación, o utilizado con
    `.expect()` si desea que el programa salga con un bonito mensaje en lugar de 
    continuar felizmente.
    */
    for i in 0..xs.len() + 1 {
        match xs.get(i) {
            Some(xval) => println!("{}: {}", i, xval),
            None => println!("Slow down! {} is too far!", i)
        }
    }
}