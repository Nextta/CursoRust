fn main(){
    // Este es un ejemplo de comentario de línea.
    // Hay dos barras al principio de la línea.
    // Y nada escrito después de estos será leído por el compilador.

    // println!("Hello, world!");

    // Corre. ¿Lo ves? Ahora intenta borrar las dos barras, y ejecútalo de nuevo.

    /*
      Este es otro tipo de comentario, un comentario de bloque. En general, 
      los comentarios de línea son el estilo de comentario recomendado. 
      Pero los comentarios de bloque son extremadamente útiles para deshabilitar 
      temporalmente trozos de código. /* Los comentarios de bloque pueden ser /* anidados, */
      */ por lo que sólo se necesitan unas pocas pulsaciones para comentar todo en 
      esta función main(). /*/*/* ¡Pruébelo usted mismo! */*/*/
     */

    /*
    Nota: La columna anterior de `*` era enteramente por estilo. En realidad no es necesaria.
    */

    // Puede manipular expresiones más fácilmente con comentarios en bloque
    // que con los comentarios de línea. Pruebe a eliminar los delimitadores de comentario
    // para cambiar el resultado:

    let x = 5 + /* 90 + */ 5;
    println!("Es x 1 o 100? x = {}", x);
}