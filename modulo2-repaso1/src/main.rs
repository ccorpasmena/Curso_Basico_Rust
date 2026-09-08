/* Ejercicio 1: Conversor de temperatura. 

Declara una variable fahrenheit con el valor 98.6. Conviértela a Celsius usando la fórmula:

C = (F - 32) × 5/9

Imprime el resultado. */

fn main() {
    let fahrenheit = 98.6;
    // Tenemos que poner los números con decimales para poder hacer las operaciones
    let celsius = (fahrenheit - 32.0) * 5.0/9.0;   
    println!("La conversión a Celsius es: {celsius}");
}
