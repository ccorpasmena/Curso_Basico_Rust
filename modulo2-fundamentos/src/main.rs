fn main() 
{
    // Fundamentos: variables, tipos de datos (primitivos y compuestos), operadores, mutabilidad e inmutabilidad y sombreado.
    
    // Para nombrar las variables usamos la nomenclatura snake_case
    let mi_variable = 8;
    println!("La variable vale: {mi_variable}");
    println!();

    // Tipos de datos primitivos
    let entero_sin_signo: u32 = 5;   // Número siempre positivo
    println!("El número es: {entero_sin_signo}");
    println!();

    let entero_con_signo = -18;   // Número entero positivo o negativo
    println!("El número es: {entero_con_signo}");
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

    // Tipos de datos compuestos
    let mi_tupla= (-47, 74.5, true, "Pepe", 'w');   // Tuplas
    println!("El valor según el índice introducido es: {}", mi_tupla.3);   // Se accede a los valores a través del índice con un punto
    println!();

    let mi_array = ["Crecen", "Mónica", "Manuel", "Adrián"];   // Arrays
    println!("El valor según el índice introducido es: {}", mi_array[1]);   // Se accede a los valores a través del índice con corchetes
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
