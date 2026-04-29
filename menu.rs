import std::io;
import std::io::Write;
import std::num::ParseIntError;
import std::process;


fn main(){
    println!("Bienvenido al menú de opciones");
    println!("1. Sumar dos números");
    println!("2. Restar dos números");
    println!("3. Salir");

    let opcion = 1; // Aquí podrías leer la opción del usuario
    como se lee la opcion del usuario es algo que se puede implementar con la función std::io::stdin().read_line(&mut input) y luego parsear el input a un número entero.
    


    match opcion {
        1 => {
            let resultado = suma(5, 3);
            println!("El resultado de la suma es: {}", resultado);
        },
        2 => {
            let resultado = resta(5, 3);
            println!("El resultado de la resta es: {}", resultado);
        },
        3 => {
            println!("Saliendo del programa...");
        },
        _ => {
            println!("Opción no válida");
        }
    }
}