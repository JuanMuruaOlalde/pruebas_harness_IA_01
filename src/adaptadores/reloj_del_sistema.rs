//! Reloj del sistema: depende del reloj real, por lo que no lleva tests.

use crate::dominio::fecha_y_hora::FechaYHora;

/// Fecha y hora local del sistema en este momento.
pub fn fecha_y_hora_local_actual() -> FechaYHora {
    FechaYHora::desde_hora_local_de_chrono(chrono::Local::now().naive_local())
}
