/* Ejercicio 4: Verificador de acceso

Declara las siguientes variables:

- edad: u8 = 20
- tiene_permiso: bool = false
- es_vip: bool = true

Crea una variable puede_entrar que sea true solo si:

- La edad es mayor o igual a 18 Y tiene permiso, O es VIP.

Imprime puede_entrar. */

fn main() {
    let edad: u8 = 20;
    let tiene_permiso: bool = false;
    let es_vip: bool = true;

    let puede_entrar= (edad >= 18 && tiene_permiso) || es_vip;
    println!("¿Puede entrar?: {puede_entrar}");    
}
