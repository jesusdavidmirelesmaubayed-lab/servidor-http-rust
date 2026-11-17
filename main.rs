use std::fs;
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time::Duration;

fn manejar_conexion(mut flujo: TcpStream) {
    let lector_buffer = BufReader::new(&mut flujo);

    let linea_peticion = lector_buffer
        .lines()
        .next()
        .unwrap()
        .unwrap();

    let (linea_estado, nombre_archivo) = match &linea_peticion[..] {
        "GET / HTTP/1.1" => {
            println!("-> Atendiendo ruta rápida (/)");
            ("HTTP/1.1 200 OK", "hola.html")
        }
        "GET /lento HTTP/1.1" => {
            println!("-> Iniciando tarea pesada en (/lento)... durmiendo 5 segs.");
            thread::sleep(Duration::from_secs(5));
            println!("-> ¡Tarea pesada (/lento) terminada!");
            ("HTTP/1.1 200 OK", "hola.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
    };

    let contenido_html = fs::read_to_string(nombre_archivo).unwrap();
    let longitud = contenido_html.len();

    let respuesta = format!(
        "{linea_estado}\r\nContent-Length: {longitud}\r\n\r\n{contenido_html}"
    );

    flujo.write_all(respuesta.as_bytes()).unwrap();
}

fn main() {
    let servidor = TcpListener::bind("127.0.0.1:7878")
        .expect("No se pudo iniciar el servidor en ese puerto");

    println!("Servidor MULTIHILO escuchando en http://127.0.0.1:7878 ...");

    for flujo in servidor.incoming() {
        let flujo = flujo.unwrap();

        // ¡AQUÍ ESTÁ LA MAGIA MULTIHILO!
        thread::spawn(move || {
            manejar_conexion(flujo);
        });
    }
}