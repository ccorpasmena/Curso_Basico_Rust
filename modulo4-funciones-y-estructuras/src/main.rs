// Módulo 4: Funciones y estructuras.

fn main() 
{
    // Si queremos sacar el resultado de la función definida abajo para usarlo después lo haremos así.
    // Definimos una variable que llame a la función.
    let resultado_suma = sumar(10, 20); 
    
    println!("El resultado de la suma es: {}", resultado_suma);

    let resultado_incrementado = incrementar_en_dos(resultado_suma);

    println!("El resultado de incrementar en 2 es: {}", resultado_incrementado);
}

fn incrementar_en_dos(num: i32) -> i32
{    
    num + 2
}

// Definimos la función aquí. 
// En los paréntesis van los parámetros que la función necesita.
fn sumar(a: i32, b:i32) -> i32
{
    println!("Sumando los valores a: {}, b: {}", a, b);
    
    let resultado = a + b;
        
    // No es necesario poner: return resultado;
    resultado
}
