//! Puerto del historico de movimientos (repositorio, en terminos de DDD).

use super::fecha_y_hora::FechaYHora;
use super::movimiento::MovimientoDeAscensor;

/// Motivos por los que falla el historico de movimientos.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ErrorDeHistoricoDeMovimientos {
    /// Fallo al leer o escribir en el almacenamiento (por ejemplo, un error de E/S).
    AccesoAlAlmacenamiento { detalle: String },
    /// Una linea del almacenamiento no se puede interpretar; las lineas se cuentan desde 1.
    LineaIlegible { numero_de_linea: usize },
}

/// Registro permanente de todos los movimientos de ascensores.
pub trait HistoricoDeMovimientos {
    /// Anade el movimiento al final del historico; nunca se borra nada.
    fn registrar(
        &mut self,
        movimiento: MovimientoDeAscensor,
    ) -> Result<(), ErrorDeHistoricoDeMovimientos>;

    /// Movimientos con fecha y hora igual o posterior a la indicada, en el orden en que se registraron.
    fn movimientos_desde(
        &self,
        fecha_y_hora: FechaYHora,
    ) -> Result<Vec<MovimientoDeAscensor>, ErrorDeHistoricoDeMovimientos>;
}
