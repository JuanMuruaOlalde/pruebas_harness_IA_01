//! Errores del dominio.

use std::fmt;

/// Motivos por los que una configuracion de edificio no es valida.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorDeConfiguracion {
    PlantaMasBajaPorEncimaDeLaMasAlta,
    /// Los ascensores empiezan en la planta principal, asi que ha de existir.
    PlantaPrincipalFueraDelRangoDePlantas,
    SinAscensores,
    /// Con un tiempo de desplazamiento cero no se podria calcular la planta actual de un ascensor.
    TiempoDeDesplazamientoCero,
}

/// Motivos por los que una operacion del simulador es rechazada.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorDeSimulacion {
    AscensorInexistente,
    PlantaInexistente,
    /// El ascensor se esta desplazando (incluidos el arranque y la parada) y no acepta nuevas ordenes.
    AscensorOcupado,
}

impl fmt::Display for ErrorDeConfiguracion {
    fn fmt(&self, formato: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mensaje = match self {
            Self::PlantaMasBajaPorEncimaDeLaMasAlta => {
                "la planta más baja está por encima de la más alta"
            }
            Self::PlantaPrincipalFueraDelRangoDePlantas => {
                "la planta principal no está entre las plantas del edificio"
            }
            Self::SinAscensores => "el edificio no tiene ascensores",
            Self::TiempoDeDesplazamientoCero => "el tiempo de desplazamiento no puede ser cero",
        };
        formato.write_str(mensaje)
    }
}

impl std::error::Error for ErrorDeConfiguracion {}

impl fmt::Display for ErrorDeSimulacion {
    fn fmt(&self, formato: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mensaje = match self {
            Self::AscensorInexistente => "el ascensor no existe en el edificio",
            Self::PlantaInexistente => "la planta no existe en el edificio",
            Self::AscensorOcupado => "el ascensor se está desplazando y no acepta nuevas órdenes",
        };
        formato.write_str(mensaje)
    }
}

impl std::error::Error for ErrorDeSimulacion {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn los_errores_de_configuracion_se_muestran_con_un_mensaje_legible_en_espanol() {
        let casos = [
            (
                ErrorDeConfiguracion::PlantaMasBajaPorEncimaDeLaMasAlta,
                "la planta más baja está por encima de la más alta",
            ),
            (
                ErrorDeConfiguracion::PlantaPrincipalFueraDelRangoDePlantas,
                "la planta principal no está entre las plantas del edificio",
            ),
            (
                ErrorDeConfiguracion::SinAscensores,
                "el edificio no tiene ascensores",
            ),
            (
                ErrorDeConfiguracion::TiempoDeDesplazamientoCero,
                "el tiempo de desplazamiento no puede ser cero",
            ),
        ];
        for (error, mensaje_esperado) in casos {
            assert_eq!(error.to_string(), mensaje_esperado);
        }
    }

    #[test]
    fn los_errores_de_simulacion_se_muestran_con_un_mensaje_legible_en_espanol() {
        let casos = [
            (
                ErrorDeSimulacion::AscensorInexistente,
                "el ascensor no existe en el edificio",
            ),
            (
                ErrorDeSimulacion::PlantaInexistente,
                "la planta no existe en el edificio",
            ),
            (
                ErrorDeSimulacion::AscensorOcupado,
                "el ascensor se está desplazando y no acepta nuevas órdenes",
            ),
        ];
        for (error, mensaje_esperado) in casos {
            assert_eq!(error.to_string(), mensaje_esperado);
        }
    }
}
