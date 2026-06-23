fn main() 
{
    println!("Fundamentos: variables, mutabilidad e inmutabilidad, sombreado, tipos de datos y operadores");
    println!();

    // Para nombrar las variables usamos la nomenclatura snake_case
    let mi_variable = 8;
    println!("La variable vale: {mi_variable}");
    println!();

    // Tipos de datos
    let edad = 18;   // Número entero
    println!("Mi edad es: {edad}");
    println!();

    let precio_lata = 5.25;   // Número flotante
    println!("El precio de la lata es: {precio_lata}");
    println!();

    let mayor_edad = false;   // Booleano
    println!("Mi hija es mayor de edad: {mayor_edad}");
    println!();

    let letra_dni = 'D';   // Character
    println!("La letra de mi DNI es: {letra_dni}");
    println!();
    
    // Mutabilidad e inmutabilidad   
    let mut x = 10;
    println!("Ahora la x vale: {x}");
    x = 20;
    println!("Después la x vale: {x}");
    println!();

    // Sombreado (shadowing)
    let a = 10;
    println!("Ahora la a vale: {a}");

    let a = "Pepe";
    println!("Ahora la a vale: {a}");   // La variable tiene el mismo nombre pero es de tipo String en este caso.

}
