fn main(){

    // Este es un comentario de una sola línea
    println!("Hola mundo");
    println!("Adios mundo");

    /* Este es un comentario de varias líneas
    que puede abarcar varias líneas de texto. */


    let x = 5;
    println!("El valor de x es: {}", x);
    let resultado = suma(3, 4);
    println!("El resultado de la suma es: {}", resultado);

    saludar("Alice");
}

fn saludar(nombre: &str) {
    println!("Hola, {}!", nombre);
}

fn suma(a: i32, b: i32) -> i32 {
    a + b
}  
fn resta(a: i32, b: i32) -> i32 {
    a - b
}