use std::sync::Arc;

use axum::{
    extract::State,
    http::{header, StatusCode},
    response::IntoResponse,
    Json,
};

use crate::modelos;

pub fn preparar_valor_match(termino: String) -> String {
    /* hay muchas cadenas que FTS considera "especiales" y que necesitan ser escapadas.
    La forma más fácil de hacerlo es añadir comillas DOBLES alrededor de la cadena que desea buscar. */
    format!(r###"'"{termino}"'"###)
}

pub async fn api(
    State(estado): State<Arc<modelos::Estado>>,
    Json(busqueda): Json<modelos::Busqueda>,
) -> impl IntoResponse {
    let coincidencias_termino = busqueda.termino.clone();
    // Contabilizar coincidencias
    let coincidencias = estado
        .conexion
        .call(|conexion| {
            let valor_match = preparar_valor_match(coincidencias_termino);
            let consulta_sql = &format!(
                r###"
                SELECT 
                   COUNT(*)
                FROM 
                    criticas 
                WHERE 
                    critica MATCH {valor_match}             
            "###
            );

            let coincidencias = conexion.query_row(&consulta_sql, [], |fila| {
                let coincidencias: i64;
                match fila.get(0) {
                    Err(error) => {
                        eprintln!("no se ha podido obtener el count(*) de la consulta {error}");
                        return Err(error);
                    }
                    Ok(ok) => coincidencias = ok,
                }
                Ok(coincidencias)
            });

            match coincidencias {
                Err(error) => Err(tokio_rusqlite::Error::Rusqlite(error)),
                Ok(ok) => Ok(ok),
            }
        })
        .await;

    if coincidencias.is_err() {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            [(header::CONTENT_TYPE, "plain/text")],
            format!(
                "error crítico en la consulta de las coincidencias: {}",
                coincidencias.err().unwrap()
            ),
        );
    }

    let filas = estado.conexion
        .call(move |conexion| {
            let valor_match = preparar_valor_match(busqueda.termino);
            let consulta_sql = &format!(r###"
                SELECT 
                    grupo, 
                    disco, 
                    url, 
                    snippet(criticas, 3, '<span class="coincidencia">', '</span>', '...', 50) as fragmento, 
                    puntuacion 
                FROM 
                    criticas 
                WHERE 
                    critica MATCH {valor_match} 
                ORDER BY 
                    grupo
                LIMIT 
                    25 
                OFFSET ?            
            "###);

            let mut declaracion_sql = conexion.prepare(&consulta_sql)?;

            let filas = declaracion_sql
                .query_map([busqueda.offset], |fila| {
                    Ok(modelos::Registro {
                        grupo: fila.get(0)?,
                        disco: fila.get(1)?,
                        url: fila.get(2)?,
                        fragmento: fila.get(3)?,
                        puntuacion: fila.get(4)?,
                    })
                })?;
            let resultados = filas.collect::<std::result::Result<Vec<modelos::Registro>, rusqlite::Error>>();
            match resultados {
                Err(error) => Err(tokio_rusqlite::Error::Rusqlite(error)),
                Ok(ok) => Ok(ok)
            }

        })
        .await;

    match filas {
        Err(error) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                [(header::CONTENT_TYPE, "plain/text")],
                format!("error crítico en la consulta a la base de datos: {error}"),
            )
        }
        Ok(ok) => {
            let respuesta = modelos::Respuesta {
                total: coincidencias.unwrap(),
                registros: ok,
            };
            match serde_json::to_string(&respuesta) {
                Err(error) => {
                    return (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        [(header::CONTENT_TYPE, "plain/text")],
                        format!("error en la serialización de la respuesta: {error}"),
                    )
                }
                Ok(ok) => {
                    return (
                        StatusCode::OK,
                        [(header::CONTENT_TYPE, "application/json")],
                        ok,
                    )
                }
            }
        }
    }
}
