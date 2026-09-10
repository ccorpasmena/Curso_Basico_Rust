/* EJERCICIO 2: Funciones con lógica y retorno

Define una función es_par que reciba un u32 y retorne un bool indicando si el numero es par.

Define una función mayor_de_tres que reciba tres i32 y retorne el mayor de ellos, usando if / else if / else.

Define una función clasificar_temperatura que reciba un i32 y retorne un char según estas reglas:
'F' (frio) si es menor a 10
'T' (templado) si está entre 10 y 24
'C' (caluroso) si es 25 o mayor

En main, llama a las tres funciones con valores de tu elección e imprime los resultados. */
fn main() {
    let numero_par = es_par(11);
    println!("¿El número es par?: {numero_par}");

    let numero_mayor = mayor_de_tres(1, 6, 5);
    println!("El número mayor es el: {numero_mayor}");

    let letra_temperatura = clasificar_temperatura(20);
    println!("El valor de la temperatura es: {letra_temperatura}");
}

fn es_par(numero: u32) -> bool {
    numero % 2 == 0
}

fn mayor_de_tres(numero_1: i32, numero_2: i32, numero_3: i32) -> i32 {
    if numero_1 >= numero_2 && numero_1 >= numero_3 {
        numero_1
    } else if numero_2 >= numero_3 {
        numero_2
    } else {
        numero_3
    }
}

fn clasificar_temperatura(temperatura: i32) -> char {
    if temperatura < 10 {
        'F'
    } else if temperatura < 25 {
        'T'
    } else {
        'C'
    }
}