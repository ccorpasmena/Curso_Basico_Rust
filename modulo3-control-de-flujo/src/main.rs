// Módulo 3: control de flujo.

// Condicionales (if, else, else if)
// Coincidencias de patrones (match)
// Bucles (loop, while, for)

fn main() {
    // Condicionales (if-else, else if)
    // If else
    let edad = 17;
    if edad >= 18 {
        println!("Eres mayor de edad")
    } else {
        println!("Eres menor de edad")
    }
    println!();

    // Else if
    let nota = 7.7;
    if nota >= 9.0 {
        println!("Sobresaliente")
    } else if nota >= 7.0 {
        println!("Notable")
    } else if nota >= 6.0 {
        println!("Bien")
    } else if nota >= 5.0 {
        println!("Suficiente")
    } else
    // Solo se ejecuta si las condiciones anteriores son falsas
    {
        println!("Suspenso")
    }
    println!();

    // Match (es similar a la estructura switch de Java)
    let numero = 1;
    match numero {
        1 => println!("Es el uno"),
        2 => println!("Es el dos"),
        3 => println!("Es el tres"),
        _ => println!("Es otro número"), // El guión bajo es equivalente al default de Java.
    }
    println!();

    // Bucles (loop, while, for)
    // Loop (Este bucle es infinito hasta que se rompe con el break)
    let mut contador = 1;
    loop {
        println!("La variable vale: {}", contador);

        if contador == 5 {
            break;
        }
        contador += 1;
    }
    println!();

    // Bucle while
    let mut contador = 1;
    while contador <= 5 {
        println!("La variable vale: {}", contador);
        contador += 1;
    }
    println!();

    // Bucle for
    for i in 1..=5
    // Si ponemos el signo igual se incluye el último valor que pongamos.
    {
        println!("La variable vale: {}", i);
    }
    println!();

    let frutas = ["manzana", "uva", "naranja"];
    for fruta in frutas.iter() {
        println!("La fruta es: {}", fruta);
    }
    println!();

    // El break se puede usar en los 3 tipos de bucles que hemos visto.
    for i in 1..=10 {
        println!("La variable vale: {}", i);

        if i == 5 {
            break;
        }
    }
    println!();

    // El continue salta a la siguiente iteración del bucle.
    for numero in 1..=10 {
        if numero % 2 == 0 {
            continue;
        }
        println!("La variable vale: {}", numero)
    }
}
