/* EJERCICIO 3: Tu primera struct

Define una struct llamada Producto con estos campos:

codigo: u32
precio: f64
stock: u32
en_venta: bool

En main, instancia dos productos distintos con valores inventados.

Imprime en pantalla cada uno de los campos del primer producto, accediendo a ellos con la notación de punto.

Calcula e imprime el valor total del inventario del segundo producto. */
fn main() {
    let producto_1 = Producto {
        codigo: 1,
        precio: 25.5,
        stock: 5,
        en_venta: true,
    };

    let producto_2 = Producto {
        codigo: 2,
        precio: 35.4,
        stock: 10,
        en_venta: false,
    };

    println!("El código del producto_1 es: {}", producto_1.codigo);
    println!("El precio del producto_1 es: {}", producto_1.precio);
    println!("El stock del producto_1 es: {}", producto_1.stock);
    println!("El producto_1 está en venta?: {}", producto_1.en_venta);

    // Como el precio es un f64 y el stock es un u32 hay que convertir este último a f64 para poder hacer la operación
    let producto_2_valor_total = producto_2.precio * producto_2.stock as f64;
    println!("El valor total del stock del producto_2 es: {producto_2_valor_total}");
}

struct Producto {
    codigo: u32,
    precio: f64,
    stock: u32,
    en_venta: bool,
}
