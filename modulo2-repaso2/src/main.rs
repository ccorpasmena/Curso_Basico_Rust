/* Ejercicio 2: Calculadora de área

Declara dos variables enteras: base = 12 y altura = 7.

Calcula e imprime:

El área de un rectángulo (base × altura)

El área de un triángulo con esa misma base y altura (base × altura / 2) */

fn main() {
    let base = 12;
    let altura = 7;

    let area_rectangulo = base * altura;
    println!("El área del rectángulo es: {area_rectangulo}");

    let area_triangulo = (base * altura)/2;
    println!("El área del triángulo es: {area_triangulo}");

}
