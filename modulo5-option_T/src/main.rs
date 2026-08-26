fn main() {
    let resultado_division = dividir(10, 0);
    
    match resultado_division {
        Some(valor) => println!("El resultado de la división es: {valor}"),
        None => println!("No se puede dividir por cero")
    };
}

// Option<i32> significa que si existe un valor, va a ser un i32
fn dividir(dividendo: i32, divisor: i32) -> Option<i32>{
    // Si no hay valor, devolverá un None
    if divisor == 0 {
        return Option::None;
    }
    // Si hay un valor, devolverá el resultado de la división
    let resultado = dividendo/divisor;
    Option::Some(resultado)
}
