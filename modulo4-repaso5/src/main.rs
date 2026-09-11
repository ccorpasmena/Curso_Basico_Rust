/* EJERCICIO 5: Metodos con impl

Define una struct Rectangulo con los campos = base: f64 y altura: f64.

Crea un bloque impl Rectangulo con los siguientes métodos:
area(&self) -> f64
perimetro(&self) -> f64
es_cuadrado(&self) -> bool, que retorne true si la base es igual a la altura.

Agrega al mismo bloque una función asociada nuevo(lado: f64) -> Rectangulo que cree un cuadrado (se llama con Rectangulo::nuevo(4.0), sin usar una instancia previa).

En main, crea un rectángulo por instanciación normal y un cuadrado usando Rectangulo::nuevo. Llama a los tres métodos en ambas instancias e imprime los resultados.

Reto adicional: agrega un método escalar(&mut self, factor: f64) que multiplique la base y la altura por el factor. */
fn main() {
    let rectangulo = Rectangulo {
        base: 30.0,
        altura: 50.0,
    };

    let area_rectangulo = rectangulo.area();
    println!("El área del rectangulo es: {area_rectangulo}");

    let perimetro_rectangulo = rectangulo.perimetro();
    println!("El perímetro del rectangulo es: {perimetro_rectangulo}");

    let rectangulo_es_cuadrado = rectangulo.es_cuadrado();
    println!("El rectangulo es cuadrado?: {rectangulo_es_cuadrado}");

    println!("------------------------------------------------------");

    let cuadrado = Rectangulo::nuevo(4.0);

    let area_cuadrado = cuadrado.area();
    println!("El área del cuadrado es: {area_cuadrado}");

    let perimetro_cuadrado = cuadrado.perimetro();
    println!("El perímetro del cuadrado es: {perimetro_cuadrado}");

    let cuadrado_es_cuadrado = cuadrado.es_cuadrado();
    println!("El cuadrado es cuadrado?: {cuadrado_es_cuadrado}");

    let mut escalable = Rectangulo {
        base: 2.0,
        altura: 3.0,
    };

    escalable.escalar(3.0);
    println!(
        "Despues de escalar, la base es: {} y la altura es: {}",
        escalable.base, escalable.altura
    );
}

struct Rectangulo {
    base: f64,
    altura: f64,
}

impl Rectangulo {
    // Metodo: usa &self para leer los datos de la instancia
    fn area(&self) -> f64 {
        self.base * self.altura
    }

    fn perimetro(&self) -> f64 {
        2.0 * (self.base + self.altura)
    }

    fn es_cuadrado(&self) -> bool {
        self.base == self.altura
    }

    // Funcion asociada: no lleva self, se llama con Rectangulo::nuevo(...)
    fn nuevo(lado: f64) -> Rectangulo {
        Rectangulo {
            base: lado,
            altura: lado,
        }
    }

    // Metodo mutable: &mut self permite modificar la instancia
    fn escalar(&mut self, factor: f64) {
        self.base *= factor;
        self.altura *= factor;
    }
}