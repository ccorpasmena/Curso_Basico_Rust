// Módulo 2: Fundamentos.

// Variables.
// Tipos de datos (primitivos y compuestos).
// Operadores (aritméticos, de comparación y lógicos).
// Mutabilidad e inmutabilidad.
// Sombreado

fn main() {
    // Para nombrar las variables usamos la nomenclatura snake_case
    let mi_variable = 8;
    println!("La variable vale: {mi_variable}");
    println!();

    // Tipos de datos primitivos
    let entero_sin_signo: u32 = 5; // Número siempre positivo
    println!("El número es: {entero_sin_signo}");

    let entero_con_signo = -18; // Número entero positivo o negativo
    println!("El número es: {entero_con_signo}");

    let precio_lata = 5.25; // Número flotante
    println!("El precio de la lata es: {precio_lata}");

    let mayor_edad = false; // Booleano
    println!("Mi hija es mayor de edad: {mayor_edad}");

    let letra_dni = 'D'; // Character
    println!("La letra de mi DNI es: {letra_dni}");
    println!();

    // Tipos de datos compuestos
    let mi_tupla = (-47, 74.5, true, "Pepe", 'w'); // Tuplas
    println!("El valor según el índice introducido es: {}", mi_tupla.3); // Se accede a los valores a través del índice con un punto

    let mi_array = ["Crecen", "Mónica", "Manuel", "Adrián"]; // Arrays
    println!("El valor según el índice introducido es: {}", mi_array[1]); // Se accede a los valores a través del índice con corchetes
    println!();

    // Operadores aritméticos
    let numero1 = 10;
    let numero2 = 5;

    println!("La suma es: {}", numero1 + numero2);
    println!("La resta es: {}", numero1 - numero2);
    println!("El producto es: {}", numero1 * numero2);
    println!("La división es: {}", numero1 / numero2);
    println!("El módulo es: {}", numero1 % numero2);
    println!();

    // Operadores de comparación
    println!("Los números son iguales?: {}", numero1 == numero2);
    println!("Los números son distintos?: {}", numero1 != numero2);
    println!(
        "El primer número es mayor o igual que el segundo?: {}",
        numero1 >= numero2
    );
    println!(
        "El primer número es menor o igual que el segundo?: {}",
        numero1 <= numero2
    );
    println!();

    // Operadores lógicos
    let frase1 = true;
    let frase2 = false;
    println!("El resultado del NOT en la frase1 es: {}", !frase1); // Si negamos la frase1 pasa a ser false
    println!("El resultado del NOT en la frase2 es: {}", !frase2); // Si negamos la frase2 pasa a ser true
    println!("El resultado del AND es: {}", frase1 && frase2); // Como una frase es falsa, el resultado es false
    println!("El resultado del OR es: {}", frase1 || frase2); // Como una frase es verdadera, el resultado es true
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
    println!("Ahora la a vale: {a}"); // La variable tiene el mismo nombre pero es de tipo String en este caso.
}
