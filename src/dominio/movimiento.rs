//! Movimientos de ascensores: ordenes dadas a un ascensor, que se guardan en el historico.

use super::ascensor::IdentificadorDeAscensor;
use super::fecha_y_hora::FechaYHora;
use super::planta::IdentificadorDePlanta;

/// Por que se ha dado la orden a un ascensor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MotivoDeMovimiento {
    /// Una persona ha pulsado el boton de llamada de una planta.
    Llamada,
    /// Una persona ha pulsado un boton de la botonera de un ascensor.
    Botonera,
    /// El control de trafico ha llevado un ascensor en reposo a una planta de espera.
    Reposicionamiento,
}

/// Registro de una orden dada a un ascensor.
/// En un movimiento por llamada, la planta de destino es la planta donde se pulso el boton.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MovimientoDeAscensor {
    pub fecha_y_hora: FechaYHora,
    pub identificador_de_ascensor: IdentificadorDeAscensor,
    pub planta_de_origen: IdentificadorDePlanta,
    pub planta_de_destino: IdentificadorDePlanta,
    pub motivo: MotivoDeMovimiento,
}
