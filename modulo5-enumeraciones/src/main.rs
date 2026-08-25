fn main() {
    // Para llamar a la función escribimos su nombre y pasamos los parámetros requeridos
    realizar_operacion(TipoOperacion::Suma(5, 6));
    realizar_operacion(TipoOperacion::Resta { a: (8), b: (10) });
    realizar_operacion(TipoOperacion::Otra);
}

enum TipoOperacion {
    Suma(i32, i32),           // Usamos una tupla para usar 2 valores.
    Resta { a: i32, b: i32 }, // Usamos una estructura anónima (no tiene nombre).
    Otra,                     // Usamos una opción sin datos.
}

// Pasamos como parámetro de la función el tipo de dato (TipoOperacion) y lo guardamos en una variable (operacion).
fn realizar_operacion(operacion: TipoOperacion) {
    println!("Realizando operación:");
    // Tenemos que gestionar las 3 opciones
    match operacion {
        TipoOperacion::Suma(numero_a, numero_b) => {
            println!("sumando el numero a: {numero_a} y el numero b: {numero_b}")
        }
        TipoOperacion::Resta { a, b } => {
            println!("restando el numero a: {a} y el numero b: {b}")
        }
        TipoOperacion::Otra => {
            println!("otra operación desconocida")
        }
    }
}
