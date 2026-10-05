//! Errores del dominio.

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
