/* Ejercicio 3: Cuenta regresiva para el despegue

Declara una variable mutable cuenta inicializada en 10.

Usa un bucle while para que, mientras cuenta sea mayor que 0, imprima el número actual y luego le reste 1.

Cuando el bucle termine, imprime "¡Despegue!". */
fn main() {
    let mut cuenta = 10;
    
    while cuenta > 0 {
        println!("La cuenta atrás: {cuenta}");
        cuenta -= 1;
    }
    println!("¡Despegue!");
}
