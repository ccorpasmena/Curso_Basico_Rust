fn main() {
    let resultado_division = dividir(100, 20);

    match resultado_division {
        Ok(resultado) => println!("Operación completada. El resultado es: {resultado}"),
        Err(error) => match error {
            DividirError::DividirPorCero => println!("No se puede dividir entre cero."),
            DividirError::DivisorInvalido(divisor) => {
                println!("No se puede dividir entre el número {divisor}")
            }
        },
    };
}

enum DividirError {
    DividirPorCero,
    DivisorInvalido(i32),
}

// Option<i32> significa que si existe un valor, va a ser un i32
fn dividir(dividendo: i32, divisor: i32) -> Result<i32, DividirError> {
    if divisor == 0 {
        return Err(DividirError::DividirPorCero);
    }

    // Por reglas de negocio, por ejemplo no se puede dividir entre 10
    if divisor == 10 {
        return Err(DividirError::DivisorInvalido(divisor));
    }

    let resultado = dividendo / divisor;
    Ok(resultado)
}