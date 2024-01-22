fn main(){
    /*
        En general, el `{}` se sustituirá automáticamente por 
        cualquier argumento. Estos serán stringificados.
     */
    println!("{} days", 31);

    /*
        Se pueden utilizar argumentos posicionales. 
        Especificar un entero dentro de `{}` determina qué 
        argumento adicional será reemplazado. Los argumentos 
        comienzan en 0 inmediatamente después de la cadena de 
        formato.
    */
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    //Al igual que los argumentos con nombre.
    println!("{subject} {verb} {object}", 
        object = "the lazy dog",
        subject = "the quick brown fox",
        verb = "jumps over"
    );

    //Se puede invocar un formato diferente especificando el carácter de formato después de `:`.
    println!("Base 10:                     {}", 69420); //69420
    println!("Base 2 (Binary):             {:b}", 69420); //10000111100101100
    println!("Base 8 (Octal):              {:o}", 69420); //207454
    println!("Base 16 (Hexadecimal):       {:x}", 69420); //10f2c
    println!("Base 16 (Hexadecimal):       {:X}", 69420); //10F2C

    /*
        Puede justificar el texto a la derecha con un ancho especificado. Esto 
        producirá " 1". (Cuatro espacios en blanco y un "1", para un ancho total de 5).
    */
    println!("{number:>5}", number=1);

    // Puedes rellenar los números con ceros extra.
    println!("{number:0>5}", number=1); // 00001

    // y ajustar a la izquierda volteando el signo. Esto producirá "10000".
    println!("{number:0<5}", number=1); // 10000

    // Puede utilizar argumentos con nombre en el especificador de formato añadiendo un `$`.
    println!("{number:0>width$}", number=1, width=5);

    /*
        Para Rust 1.58 y superiores, puedes capturar directamente el argumento de una 
        variable circundante. Como en el caso anterior, esto producirá 
        " 1", 4 espacios en blanco y un "1".
    */
    let number: f64 = 1.0;
    let width: usize = 5;

    println!("{number:>width$}");


    // Ejercicio.
    /*
        Añade una llamada a la macro println! que imprima: Pi es aproximadamente 
        3.142 controlando el número de decimales mostrados. Para los propósitos 
        de este ejercicio, utilice let pi = 3.141592 como una estimación de pi. 
        (Sugerencia: puede que necesite consultar la documentación de std::fmt 
        para establecer el número de decimales a mostrar)
    */
    let pi: f64 = 3.141592;
    println!("Pi es aproximadamente {pi:.*}", 3);
}