/* Ejercicio 4: Suma de números pares

Crea una variable mutable llamada suma_total inicializada en 0.

Usa un bucle for que recorra un rango de números del 1 al 20 (inclusive).

Dentro del bucle, usa un if y el operador módulo (%) para comprobar si el número es par. Si lo es, súmalo a suma_total.

Al final del programa, imprime el resultado. */
fn main() {
    let mut suma_total = 0;
    
    for numero in 1..=20{
        if numero % 2 == 0 {
            suma_total += numero;
        }
    }  
    println!("La suma total de los números pares del 1 al 20 es: {suma_total}");
}
