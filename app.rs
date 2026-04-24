fn main(){
    println!("Hola mundo");
    println!("Adios mundo");
    let x = 5;
    println!("El valor de x es: {}", x);
    let resultado = suma(3, 4);
    println!("El resultado de la suma es: {}", resultado);
}

fn suma(a: i32, b: i32) -> i32 {
    a + b
}   