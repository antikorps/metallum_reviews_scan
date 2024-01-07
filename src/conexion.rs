use std::env::current_exe;

use tokio_rusqlite::Connection;

pub async fn conectar_base_datos() -> Connection {
    let ruta_ejecutable = current_exe().expect("no se ha podido obtener la ruta del ejecutable");
    let ruta_raiz = ruta_ejecutable
        .parent()
        .expect("no se ha podido obtener el directorio del ejecutable");
    let ruta_base_datos = ruta_raiz.join("metallum_reviews.sqlite3");

    return Connection::open(ruta_base_datos)
        .await
        .expect("no se ha podido realizar la conexión con la base de datos sqlite");
}
