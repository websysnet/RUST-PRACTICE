
mod operaciones;
use std::io::{self, Write};
use operaciones::{suma, resta};

fn leer_entero(prompt: &str) -> i32 {
    loop {
        print!("{}", prompt);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Error al leer la entrada");

        match input.trim().parse() {
            Ok(num) => return num,
            Err(_) => println!("Entrada no válida. Por favor, ingrese un número."),
        }
    }
}

fn main(){
    println!("Bienvenido al menú de opciones");
    println!("1. Sumar dos números");
    println!("2. Restar dos números");
    println!("3. Salir");

    let opcion: i32 = leer_entero("Ingrese su opción: ");

    


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