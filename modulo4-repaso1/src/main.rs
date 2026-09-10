/* EJERCICIO 1: Primeras funciones

Define una funcion llamada area_rectangulo que reciba dos parametros de tipo f64 (base y altura) y retorne el area como f64.

Define una funcion llamada imprimir_encabezado que no reciba parametros, no retorne nada, y simplemente imprima un mensaje fijo como "Calculadora de areas".

En main, llama primero a imprimir_encabezado y luego usa area_rectangulo para calcular el area de un rectangulo de 5.0 por 3.0, imprimiendo el resultado. */
fn main() {
    imprimir_encabezado();
    let area_calculada = area_rectangulo(5.0, 3.0);
    println!("El área del rectángulo es: {area_calculada}");
}

fn area_rectangulo(base: f64, altura: f64) -> f64 {
    base * altura
}

fn imprimir_encabezado() {
    println!("Calculadora de áreas")
}
