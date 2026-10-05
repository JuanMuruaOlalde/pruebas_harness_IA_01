//! Ascensores y su posicion en el tiempo.

use std::time::Duration;

use super::configuracion::ConfiguracionDelEdificio;
use super::errores::ErrorDeSimulacion;
use super::planta::IdentificadorDePlanta;

/// Numero entero positivo que identifica un ascensor; se numeran desde 1.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct IdentificadorDeAscensor(pub u32);

/// Estado visible de un ascensor: parado u ocupado desplazandose.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EstadoDeAscensor {
    Parado,
    Desplazandose {
        planta_de_destino: IdentificadorDePlanta,
    },
}

/// Planta actual y estado de un ascensor en un instante dado.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PosicionDeAscensor {
    pub planta_actual: IdentificadorDePlanta,
    pub estado: EstadoDeAscensor,
}

/// Ascensor: entidad del agregado `SimuladorDeEdificio`, que es quien le da las ordenes.
/// Guarda su estado interno, del que se calcula su posicion en cualquier instante.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct Ascensor {
    identificador: IdentificadorDeAscensor,
    estado: EstadoInterno,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum EstadoInterno {
    Parado { planta: IdentificadorDePlanta },
    Desplazandose(Desplazamiento),
}

impl Ascensor {
    pub fn parado_en(
        identificador: IdentificadorDeAscensor,
        planta: IdentificadorDePlanta,
    ) -> Self {
        Self {
            identificador,
            estado: EstadoInterno::Parado { planta },
        }
    }

    pub fn identificador(&self) -> IdentificadorDeAscensor {
        self.identificador
    }

    pub fn posicion_en(
        &self,
        instante: Duration,
        configuracion: &ConfiguracionDelEdificio,
    ) -> PosicionDeAscensor {
        match self.estado {
            EstadoInterno::Parado { planta } => PosicionDeAscensor {
                planta_actual: planta,
                estado: EstadoDeAscensor::Parado,
            },
            EstadoInterno::Desplazandose(desplazamiento) => {
                desplazamiento.posicion_en(instante, configuracion)
            }
        }
    }

    /// Inicia, en el instante indicado, un desplazamiento desde la planta actual hasta la de destino.
    /// Si el destino es la planta en la que ya esta parado, se queda parado en ella sin consumir tiempo.
    /// Rechaza la orden si en ese instante el ascensor se esta desplazando.
    pub fn iniciar_desplazamiento(
        &mut self,
        planta_de_destino: IdentificadorDePlanta,
        instante_de_inicio: Duration,
        configuracion: &ConfiguracionDelEdificio,
    ) -> Result<(), ErrorDeSimulacion> {
        let posicion = self.posicion_en(instante_de_inicio, configuracion);
        if posicion.estado != EstadoDeAscensor::Parado {
            return Err(ErrorDeSimulacion::AscensorOcupado);
        }
        let planta_de_origen = posicion.planta_actual;
        self.estado = if planta_de_origen == planta_de_destino {
            EstadoInterno::Parado {
                planta: planta_de_origen,
            }
        } else {
            EstadoInterno::Desplazandose(Desplazamiento {
                planta_de_origen,
                planta_de_destino,
                instante_de_inicio,
            })
        };
        Ok(())
    }
}

/// Desplazamiento de un ascensor desde una planta de origen hasta una de destino distinta,
/// iniciado en un instante dado. Incluye el arranque y la parada.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Desplazamiento {
    planta_de_origen: IdentificadorDePlanta,
    planta_de_destino: IdentificadorDePlanta,
    instante_de_inicio: Duration,
}

impl Desplazamiento {
    /// Posicion en el instante indicado: desplazandose hasta cumplirse la duracion total
    /// del desplazamiento, y parado en la planta de destino a partir de entonces.
    fn posicion_en(
        &self,
        instante: Duration,
        configuracion: &ConfiguracionDelEdificio,
    ) -> PosicionDeAscensor {
        let tiempo_transcurrido = instante.saturating_sub(self.instante_de_inicio);
        let duracion_total = configuracion.duracion_de_un_desplazamiento(self.distancia());
        if tiempo_transcurrido >= duracion_total {
            return PosicionDeAscensor {
                planta_actual: self.planta_de_destino,
                estado: EstadoDeAscensor::Parado,
            };
        }
        PosicionDeAscensor {
            planta_actual: self.ultima_planta_alcanzada(tiempo_transcurrido, configuracion),
            estado: EstadoDeAscensor::Desplazandose {
                planta_de_destino: self.planta_de_destino,
            },
        }
    }

    /// Durante el arranque (la primera mitad del tiempo de arranque y de parada) sigue en la
    /// planta de origen; despues, alcanza una planta mas cada vez que se cumple el tiempo de
    /// desplazamiento, sin pasar de la planta de destino.
    fn ultima_planta_alcanzada(
        &self,
        tiempo_transcurrido: Duration,
        configuracion: &ConfiguracionDelEdificio,
    ) -> IdentificadorDePlanta {
        let tiempo_de_arranque = configuracion.tiempo_de_arranque_y_de_parada / 2;
        let tiempo_de_recorrido = tiempo_transcurrido.saturating_sub(tiempo_de_arranque);
        let plantas_completadas =
            tiempo_de_recorrido.as_nanos() / configuracion.tiempo_de_desplazamiento.as_nanos();
        let plantas_recorridas = u32::try_from(plantas_completadas)
            .unwrap_or(u32::MAX)
            .min(self.distancia());
        let IdentificadorDePlanta(numero_de_la_planta_de_origen) = self.planta_de_origen;
        if self.planta_de_destino > self.planta_de_origen {
            IdentificadorDePlanta(
                numero_de_la_planta_de_origen.saturating_add_unsigned(plantas_recorridas),
            )
        } else {
            IdentificadorDePlanta(
                numero_de_la_planta_de_origen.saturating_sub_unsigned(plantas_recorridas),
            )
        }
    }

    fn distancia(&self) -> u32 {
        self.planta_de_origen.distancia_a(self.planta_de_destino)
    }
}
