fn main() 
{
    // Creación de una instancia de la estructura Rectangulo
    let rectangulo1 = Rectangulo 
    { 
        ancho: 30, alto: 50 
    };

    // Acceso a los campos de la estructura y visualización de sus valores
    println!("El ancho del rectángulo1 es: {}", rectangulo1.ancho);
    println!("El alto del rectángulo1 es: {}", rectangulo1.alto);

    // Llamada al método area() para calcular el área del rectángulo
    let area = rectangulo1.area();
    println!("El área del rectángulo1 es: {}", area);

    // Creación de otra instancia de la estructura Rectangulo utilizando la función asociada
    let rectangulo2 = Rectangulo::nueva(20, 40);
    let area2 = rectangulo2.area();
    println!("El área del segundo rectángulo2 es: {}", area2);
}

// Definición de la estructura Rectangulo
struct Rectangulo 
{
    ancho: u32,
    alto: u32
}

// Implementación de métodos para la estructura Rectangulo
impl Rectangulo 
{
    // Método para calcular el área del rectángulo
    fn area(&self) -> u32 
    {
        self.ancho * self.alto
    }   

    // Función asociada
    fn nueva(ancho: u32, alto: u32) -> Self
    {
        Self { ancho, alto }
    } 
}
