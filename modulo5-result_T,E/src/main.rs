fn main() {
    let resultado_division = dividir(10, 0);
    
    match resultado_division {
        Some(valor) => println!("El resultado de la división es: {valor}"),
        None => println!("No se puede dividir por cero")
    };
}

enum DividirError {
    DividirPorCero,
}

// Option<i32> significa que si existe un valor, va a ser un i32
fn dividir(dividendo: i32, divisor: i32) -> Result<i32, String>{
    
    if divisor == 0 {
        return Err(DividirError::DividirPorCero);
    }
    
    let resultado = dividendo/divisor;
    Ok(resultado)
}
