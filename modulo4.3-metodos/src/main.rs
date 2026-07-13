fn main() {
    // Creación de una instancia de la estructura Rectangulo
    let rectangulo1 = Rectangulo {
        ancho: 30,
        alto: 50,
    };

    // Visualización de los valores de los campos de la instancia rectangulo1
    println!("El ancho del rectángulo1 es: {}", rectangulo1.ancho);
    println!("El alto del rectángulo1 es: {}", rectangulo1.alto);

    // Cálculo del área utilizando el método area()
    let area = rectangulo1.area();

    // Visualización del área calculada
    println!("El área del rectángulo1 es: {}", area);
    println!("--------------------------------------------------------");

    // Creación de una instancia mutable de la estructura Rectangulo
    let mut rectangulo3 = Rectangulo::nuevo_rectangulo(20, 40);
    // Incremento del ancho del rectángulo3 utilizando el método incrementar_ancho()
    rectangulo3.incrementar_ancho();

    // Visualización de los valores de los campos de la instancia rectangulo3 después del incremento
    println!("El ancho del rectángulo3 es: {}", rectangulo3.ancho);
    println!("El alto del rectángulo3 es: {}", rectangulo3.alto);
    println!("El área del rectángulo3 es: {}", rectangulo3.area());
    println!("--------------------------------------------------------");

    // Creación de una nueva instancia de la estructura Rectangulo utilizando la función asociada nuevo_rectangulo()
    let rectangulo2 = Rectangulo::nuevo_rectangulo(20, 40);

    println!("El ancho del rectángulo2 es: {}", rectangulo2.ancho);
    println!("El alto del rectángulo2 es: {}", rectangulo2.alto);

    // Cálculo del área utilizando el método area() para la instancia rectangulo2
    let area2 = rectangulo2.area();

    println!("El área del rectángulo2 es: {}", area2);
}

// Definición de la estructura Rectangulo con dos campos: ancho y alto
struct Rectangulo {
    ancho: u32,
    alto: u32,
}

// Implementación de métodos y funciones asociadas para la estructura Rectangulo
impl Rectangulo {
    // Método para calcular el área del rectángulo
    // Un método tiene acceso a los campos de la estructura, por lo que puede usar &self
    fn area(&self) -> u32 {
        self.ancho * self.alto
    }

    // Método para incrementar el ancho del rectángulo
    fn incrementar_ancho(&mut self) {
        self.ancho = 30;
    }

    // Función asociada para crear una nueva instancia de Rectangulo
    // Una función asociada no tiene acceso a los campos de la estructura, por lo que no puede usar &self
    fn nuevo_rectangulo(ancho: u32, alto: u32) -> Self {
        Self { ancho, alto }
    }
}
