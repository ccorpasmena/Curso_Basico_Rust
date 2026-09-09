/* Ejercicio 5: El Cajero Automático

Simula el saldo de una cuenta bancaria.

Declara let mut saldo = 100;.

Declara let mut transaccion = 1; (esto servirá para controlar qué operación hacemos).

Crea un bucle infinito con loop.

Dentro del bucle, usa match transaccion para simular 3 pasos:

Paso 1: Sumar 50 al saldo e imprimir el nuevo saldo.
Paso 2: Intentar retirar 30. Imprimir el nuevo saldo.
Paso 3: Intentar retirar 150. ¡Cuidado! Usa un if aquí para verificar si hay suficiente saldo. Si hay, réstalo; si no, imprime "Fondos insuficientes".
Comodín _: Rompe el bucle (break) para terminar el programa.

Después de cada paso, recuerda aumentar el valor de transaccion en 1. */
fn main() {
    let mut saldo = 100;
    let mut transaccion = 1;

    loop {
        match transaccion {
            1 => {
                saldo += 50;
                println!("Ingresamos 50 euros en la cuenta. El saldo actual es: {saldo}");
            }
            2 => {
                saldo -= 30;
                println!("Retiramos 30 euros de la cuenta. El saldo actual es: {saldo}");
            }
            3 => {
                if saldo >= 150 {
                    saldo -= 150;
                    println!("Retiramos 150 euros de la cuenta. El saldo actual es: {saldo}");
                } else {
                    println!("Fondos insuficientes");
                }
            }
            _ => {
                println!("Operaciones finalizadas. Saliendo del cajero ...");
                break;
            }
        }
        transaccion += 1;
    }
}
