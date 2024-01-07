use clap::Parser;
use tokio_rusqlite::Connection;

use serde::{Deserialize, Serialize};

/// Metallum reviews scan
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Argumentos {
    /// Puerto en el que se iniciará la aplicación web
    #[arg(short, long, default_value_t = 8080)]
    pub port: i64,
}

pub struct Estado {
    pub conexion: Connection,
}

#[derive(Deserialize)]
pub struct Busqueda {
    pub termino: String,
    pub offset: i64,
}
#[derive(Serialize)]
pub struct Registro {
    pub grupo: String,
    pub disco: String,
    pub url: String,
    pub fragmento: String,
    pub puntuacion: i8,
}

#[derive(Serialize)]
pub struct Respuesta {
    pub total: i64,
    pub registros: Vec<Registro>,
}

#[derive(Serialize)]
pub struct Actualizacion {
    pub version: String,
    pub fecha: i64,
}
