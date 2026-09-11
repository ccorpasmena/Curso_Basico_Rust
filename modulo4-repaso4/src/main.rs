/* EJERCICIO 4: Structs como parametros y valores de retorno

Reutiliza la struct Producto del ejercicio anterior.

Define una función valor_inventario que reciba un Producto y retorne un f64 con el valor total (precio por stock).

Define una función aplicar_descuento que reciba un Producto y un f64 con el porcentaje de descuento (por ejemplo 10.0),
y retorne un Producto nuevo con el precio ajustado. El producto original no debe modificarse.

En main, crea un producto, imprime su valor de inventario, aplícale un descuento del 10 %,
e imprime el precio original y el precio con descuento para comprobar que funcionó. */
fn main() {
    let producto = Producto {
        codigo: 2,
        precio: 35.4,
        stock: 10,
        en_venta: false,
    };

    // Guardamos una copia del precio original
    let precio_original = producto.precio;

    // Aplicamos el descuento del 10%
    let precio_con_descuento = aplicar_descuento(producto, 10.0);

    println!("El precio original es: {precio_original}");
    println!(
        "El precio con descuento es: {}",
        precio_con_descuento.precio
    );
    println!(
        "El valor de inventario del producto con el descuento aplicado es: {}",
        valor_inventario(precio_con_descuento)
    );
}

struct Producto {
    codigo: u32,
    precio: f64,
    stock: u32,
    en_venta: bool,
}

fn valor_inventario(producto: Producto) -> f64 {
    producto.precio * producto.stock as f64
}

fn aplicar_descuento(producto: Producto, porcentaje: f64) -> Producto {
    let nuevo_precio = producto.precio * (1.0 - porcentaje / 100.0);
    Producto {
        precio: nuevo_precio,
        codigo: producto.codigo,
        stock: producto.stock,
        en_venta: producto.en_venta,
    }
}
