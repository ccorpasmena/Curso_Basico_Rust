fn main() {
    // Creación de una instancia de la estructura Usuario
    // La variable usuario_empresa es de tipo Usuario y se inicializa con valores específicos para cada campo de la estructura
    let usuario_empresa: Usuario = Usuario {
        // Usamos snake_case para los nombres de los campos de la estructura
        nombre_usuario: String::from("ccorpas"),
        email: String::from("ccorpas@empresa.com"),
        edad: 47,
        activo: true,
    };

    // Acceso a los campos de la estructura e imprime por pantalla la información
    println!(
        "El nombre del usuario es: {}",
        usuario_empresa.nombre_usuario
    );
    println!("El email del usuario es: {}", usuario_empresa.email);
    println!("La edad del usuario es: {}", usuario_empresa.edad);
    println!("El usuario está activo: {}", usuario_empresa.activo);
}

// Definición de la estructura Usuario usando PascalCase para nombrarla
struct Usuario {
    nombre_usuario: String,
    email: String,
    edad: u32,
    activo: bool,
}
