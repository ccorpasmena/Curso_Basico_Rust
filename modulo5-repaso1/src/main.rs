/* EJERCICIO 1: Estados de un juego (enum basico)

Define un enum llamado EstadoJuego con las variantes: Iniciando, Jugando, EnPausa y Terminado.

Escribe una funcion mostrar_mensaje(estado: EstadoJuego) que imprima un mensaje distinto para cada variante usando match.

Escribe una funcion esta_activo(estado: EstadoJuego) -> bool que retorne true solo si el estado es Jugando o EnPausa. Resuelvelo con un match agrupando variantes.

En main, crea una variable por cada estado y llama a ambas funciones con cada una. */
fn main() {
    let iniciando = EstadoJuego::Iniciando;
    let jugando = EstadoJuego::Jugando;
    let en_pausa = EstadoJuego::EnPausa;
    let terminado = EstadoJuego::Terminado;

    mostrar_mensaje(&iniciando);
    mostrar_mensaje(&jugando);
    mostrar_mensaje(&en_pausa);
    mostrar_mensaje(&terminado);

    println!("Iniciando activo: {}", esta_activo(&iniciando));
    println!("Jugando activo: {}", esta_activo(&jugando));
    println!("EnPausa activo: {}", esta_activo(&en_pausa));
    println!("Terminado activo: {}", esta_activo(&terminado));
}

enum EstadoJuego {
    Iniciando,
    Jugando,
    EnPausa,
    Terminado,
}

fn mostrar_mensaje(estado: &EstadoJuego) {
    match estado {
        EstadoJuego::Iniciando => println!("Cargando la partida ..."),
        EstadoJuego::Jugando => println!("La partida está en marcha."),
        EstadoJuego::EnPausa => println!("Partida pausada."),
        EstadoJuego::Terminado => println!("Fin de la partida."),
    }
}

fn esta_activo(estado: &EstadoJuego) -> bool {
    match estado {
        EstadoJuego::Jugando | EstadoJuego::EnPausa => true,
        _ => false,
    }
}