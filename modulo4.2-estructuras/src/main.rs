fn main() 
{
    // Creación de una instancia de la estructura Usuario
    let usuario_empresa: Usuario = Usuario 
    {
        nombre_usuario: String::from("ccorpas"),
        email: String::from("ccorpas@empresa.com"),
        edad: 47,
        activo: true
    };    

    // Acceso a los campos de la estructura y muestra de la información
    println!("El nombre del usuario es: {}", usuario_empresa.nombre_usuario);
    println!("El email del usuario es: {}", usuario_empresa.email);
    println!("La edad del usuario es: {}", usuario_empresa.edad);
    println!("El usuario está activo: {}", usuario_empresa.activo);
}

// Definición de la estructura Usuario
struct Usuario 
    {
        nombre_usuario: String, 
        email: String,               
        edad: u32,
        activo: bool
    }    
