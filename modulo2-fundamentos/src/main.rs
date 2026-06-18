fn main() 
{
    println!("Fundamentos: variables, mutabilidad e inmutabilidad, sombreado, tipos de datos y operadores");
    println!();

    // Mutabilidad e inmutabilidad   
    let mut x = 10;
    println!("Ahora la x vale: {}", x);
    x = 20;
    println!("Después la x vale: {}", x);
    println!();

    // Sombreado (shadowing)
    let a = 10;
    println!("Ahora la a vale: {}", a);

    let a = "Pepe";
    println!("Ahora la a vale: {}", a);

}
