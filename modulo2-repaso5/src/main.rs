/* Ejercicio 5: Ficha de personaje

Crea una tupla que represente un personaje de videojuego con:

Nombre (&str)
Nivel (u8)
Vida (f64)
¿Está vivo? (bool)

Luego:

- Desestructura la tupla en 4 variables separadas.
- Resta 25.5 de daño a la vida.
- Comprueba si la vida sigue siendo mayor que 0.

Imprime todos los datos. */
fn main() {
    let personaje: (&str, u8, f64, bool) = ("Crecen", 5, 100.0, true);

    // Desestructuramos la tupla en 4 variables separadas.
    // Ponemos vida como mut porque va a cambiar el valor de esa variable
    let (nombre, nivel, mut vida, esta_vivo) = personaje;
    
    println!("El nombre es: {nombre}");
    println!("El nivel es: {nivel}");
    println!("¿Está vivo?: {esta_vivo}");

    vida = vida - 25.5;
    let sigue_vivo = vida > 0.0;

    println!("La cantidad de vida restante es: {vida}");
    println!("¿Está vivo tras el daño recibido?: {sigue_vivo}");  
}
