/* EJERCICIO 4: Structs como parametros y valores de retorno

Reutiliza la struct Producto del ejercicio anterior.

Define una funcion valor_inventario que reciba un Producto y retorne un f64 con el valor total (precio por stock).

Define una funcion aplicar_descuento que reciba un Producto y un f64 con el porcentaje de descuento (por ejemplo 10.0), 
y retorne un Producto nuevo con el precio ajustado. El producto original no debe modificarse.

En main, crea un producto, imprime su valor de inventario, aplicale un descuento del 10 por ciento, 
e imprime el precio original y el precio con descuento para comprobar que funcionó. */
fn main() {
    let producto = Producto {
        codigo: 2,
        precio: 35.4,
        stock: 10,
        en_venta: false,
    };

    // Guardamos una copia del precio antes de entregar el producto
    let precio_original = producto.precio;

    // El producto pasa a ser dueno de la funcion (se "mueve")
    let con_descuento = aplicar_descuento(producto, 10.0);

    println!("Precio original: {}", precio_original);
    println!("Precio con descuento: {}", con_descuento.precio);
    println!("Valor de inventario final: {}", valor_inventario(con_descuento));

}

struct Producto {
    codigo: u32,
    precio: f64,
    stock: u32,
    en_venta: bool,
}

fn valor_inventario (producto:Producto) -> f64 {
    producto.precio * producto.stock as f64
}

fn aplicar_descuento (producto:Producto, porcentaje: f64) -> Producto {
    let nuevo_precio = producto.precio * (1.0 - porcentaje / 100.0);
    Producto {

    precio: nuevo_precio,
    codigo: producto.codigo,
    stock: producto.stock,
    en_venta: producto.en_venta,

    }
}
  
