//! Puerto del historico de movimientos (repositorio, en terminos de DDD).

use std::fmt;

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

impl fmt::Display for ErrorDeHistoricoDeMovimientos {
    fn fmt(&self, formato: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::AccesoAlAlmacenamiento { detalle } => write!(
                formato,
                "no se ha podido acceder al histórico de movimientos: {detalle}"
            ),
            Self::LineaIlegible { numero_de_linea } => write!(
                formato,
                "la línea {numero_de_linea} del histórico de movimientos no se puede leer"
            ),
        }
    }
}

impl std::error::Error for ErrorDeHistoricoDeMovimientos {}

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn los_errores_del_historico_de_movimientos_se_muestran_con_un_mensaje_legible_que_incluye_sus_datos()
     {
        let acceso = ErrorDeHistoricoDeMovimientos::AccesoAlAlmacenamiento {
            detalle: "disco lleno".to_string(),
        };
        assert_eq!(
            acceso.to_string(),
            "no se ha podido acceder al histórico de movimientos: disco lleno"
        );
        let linea = ErrorDeHistoricoDeMovimientos::LineaIlegible { numero_de_linea: 7 };
        assert_eq!(
            linea.to_string(),
            "la línea 7 del histórico de movimientos no se puede leer"
        );
    }
}
