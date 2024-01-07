use std::sync::Arc;

use axum::{
    routing::{get, post},
    Router,
};
use clap::Parser;
mod actualizaciones;
mod api;
mod conexion;
mod index;
mod modelos;
#[tokio::main]
async fn main() {
    let argumentos = modelos::Argumentos::parse();

    let conexion = conexion::conectar_base_datos().await;

    let estado_compartido = Arc::new(modelos::Estado { conexion });

    let app = Router::new()
        .route("/", get(index::index))
        .route("/api", post(api::api))
        .route("/actualizacion", get(actualizaciones::ultima))
        .with_state(estado_compartido);

    let direccion_local = format!("0.0.0.0:{}", argumentos.port);
    let manejador_web = tokio::net::TcpListener::bind(direccion_local)
        .await
        .unwrap();
    println!(
        "Entra en http://0.0.0.0:{} para acceder a Metallum Reviews Scan",
        argumentos.port
    );
    axum::serve(manejador_web, app).await.unwrap();
}
