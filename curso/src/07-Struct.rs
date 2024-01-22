// Un atributo para ocultar las advertencias de código no utilizado.
#![allow(dead_code)]

#[derive(Debug)]
struct Person {
    name: String,
    age: u8
}

// Una unidad struct
struct Unit;

// Una tupla struct
struct Pair(i32, f32);

// Una estructura con dos campos
struct Point {
    x: f32,
    y: f32
}

// Los struct pueden reutilizarse como campos de otro struct
struct Rectangle {
    /*
    Un rectángulo puede ser especificado por donde están las esquinas 
    superior izquierda e inferior derecha en el espacio.
    */
    top_left: Point,
    bottom_right: Point
}

fn main(){
    // Crear estructura con campo init abreviado
    let name = String::from("Peter");
    let age = 27;
    let peter = Person { name, age };

    // Imprimir estructura de depuración
    println!("{:?}", peter);

    // Instanciar un `Point`
    let point: Point = Point {x: 10.3, y: 0.4};

    // Acceder a los campos de Point
    println!("point coordinates: ({}, {})", point.x, point.y);

    /* 
    Crea un nuevo Point utilizando la sintaxis struct update para utilizar 
    los campos de nuestro otro Point.
    */
    let bottom_right = Point{ x: 5.2, ..point};

    // `bottom_right.y` será el mismo que `point.y` porque usamos ese campo de `point`.
    println!("second point: ({}, {})", bottom_right.x, bottom_right.y);

    // Desestructurar Point usando un enlace `let
    let Point {x: left_edge, y: top_edge} = point;

    let _rectangle = Rectangle {
        // la instanciación de la estructura también es una expresión
        top_left: Point { x: left_edge, y: top_edge },
        bottom_right: bottom_right,
    };

    // Instanciar una unidad struct
    let _unit = Unit;

    // Instanciar una tupla struct
    let pair = Pair(1, 0.1);

    // Acceder a los campos de una tupla struct
    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    // Desestructurar una tupla struct
    let Pair(integer, decimal) = pair;

    println!("pair contains {:?} and {:?}", integer, decimal);
}