fn main(){
    // Las variables pueden tener anotaciones de tipo.
    let logical: bool = true;
    let a_float: f64 = 1.0;
    let an_integer: i32 = 5;

    println!("{logical} {a_float} {an_integer}");

    // O se utilizará un valor por defecto.
    let default_float   = 3.0; // `f64`
    let default_integer = 7;   // `i32`
    println!("{default_float} {default_integer}");

    // Un tipo también puede inferirse a partir del contexto.
    let mut inferred_type = 12; // El tipo i64 se infiere de otra línea.
    println!("{inferred_type}");
    inferred_type = 4294967296i64;
    println!("{inferred_type}");

    // El valor de una variable mutable puede ser cambiado.
    let mut mutable: i32 = 12;
    println!("{mutable}");
    mutable = 21;
    println!("{mutable}");

    // Las variables se pueden sobrescribir con shadowing.
    let mutable: bool = true;
    println!("{mutable}");

    // Suma de números enteros
    println!("1 + 2 = {}", 1u32 + 2);

    // Resta de números enteros
    println!("1 - 2 = {}", 1i32 - 2);

    // Notación científica
    println!("1e4 is {}, -2.5e-3 is {}", 1e4, -2.5e-3);

    // Lógica booleana de cortocircuito
    println!("true AND false is {}", true && false);
    println!("true OR false is {}", true || false);
    println!("Not true is {}", !true);

    // Operaciones bit a bit
    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);

    // ¡Utiliza guiones bajos para mejorar la legibilidad!
    println!("One million is written as {}", 1_000_000u32);
}