use std::sync::Arc;

use crate::modelos;
use axum::{extract::State, Json};
use serde_json::{json, Value};

pub async fn ultima(State(estado): State<Arc<modelos::Estado>>) -> Json<Value> {
    let actualizacion = estado
        .conexion
        .call(|conexion| {
            let sql_consulta_actualizacion =
                "SELECT version, fecha from actualizaciones WHERE id=1";
            let fecha = conexion.query_row(sql_consulta_actualizacion, [], |fila| {
                let version: String = fila.get(0)?;
                let fecha: i64 = fila.get(1)?;
                return Ok(modelos::Actualizacion { version, fecha });
            });
            match fecha {
                Err(error) => {
                    eprintln!("error recuperando la última fecha de actualizacion: {error}");
                    return Err(tokio_rusqlite::Error::Rusqlite(error));
                }
                Ok(ok) => return Ok(ok),
            }
        })
        .await;
    match actualizacion {
        Err(_) => Json(json!({"version": 0, "actualizacion": 0})),
        Ok(ok) => Json(json!({"version": ok.version, "actualizacion": ok.fecha})),
    }
}
