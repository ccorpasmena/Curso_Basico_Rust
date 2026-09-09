/* Ejercicio 2: El día de la semana

Declara una variable inmutable dia_numero con un valor del 1 al 7.

Usa una expresión match para imprimir el nombre del día correspondiente (1 = "Lunes", 2 = "Martes", etc.).

Si por error pones un número como 0 u 8, que el programa imprima "Día inválido". */
fn main() {
    let dia_numero = 3;

    match dia_numero {
        1 => println!("Lunes"),
        2 => println!("Martes"),
        3 => println!("Miércoles"),
        4 => println!("Jueves"),
        5 => println!("Viernes"),
        6 => println!("Sábado"),
        7 => println!("Domingo"),
        _ => println!("Día inválido")
    }
}
