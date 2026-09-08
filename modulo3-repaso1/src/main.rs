/* Ejercicio 1: Clasificador de Notas

Declara una variable inmutable llamada nota con un valor entero entre 0 y 100 (por ejemplo, 85).

Usa condicionales para imprimir la calificación en letras según esta escala:

90 a 100: "A"
80 a 89: "B"
70 a 79: "C"
60 a 69: "D"
Menos de 60: "Reprobado" */
fn main() {
    let nota = 85;

    if nota >= 90{
        println!("A")
    }else if nota >= 80 {
        println!("B")
    }else if nota >= 70 {
        println!("C")
    }else if nota >= 60 {
        println!("D")
    }else {
        println!("Reprobado")
    }
}
