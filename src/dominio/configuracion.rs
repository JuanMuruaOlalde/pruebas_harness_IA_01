//! Configuracion del edificio.

use std::time::Duration;

use super::errores::ErrorDeConfiguracion;
use super::planta::IdentificadorDePlanta;

/// Datos que definen un edificio y el comportamiento de sus ascensores.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ConfiguracionDelEdificio {
    pub planta_mas_baja: IdentificadorDePlanta,
    pub planta_mas_alta: IdentificadorDePlanta,
    pub numero_de_ascensores: u32,
    pub tiempo_de_desplazamiento: Duration,
    pub tiempo_de_arranque_y_de_parada: Duration,
}

impl ConfiguracionDelEdificio {
    /// Edificio estandar: plantas de la -2 a la 7, 3 ascensores, 2 s de desplazamiento y 4 s de arranque y parada.
    pub fn estandar() -> Self {
        Self {
            planta_mas_baja: IdentificadorDePlanta(-2),
            planta_mas_alta: IdentificadorDePlanta(7),
            numero_de_ascensores: 3,
            tiempo_de_desplazamiento: Duration::from_secs(2),
            tiempo_de_arranque_y_de_parada: Duration::from_secs(4),
        }
    }

    /// Numero de plantas del edificio, incluidas la mas baja y la mas alta.
    pub fn numero_de_plantas(&self) -> u32 {
        self.planta_mas_baja.distancia_a(self.planta_mas_alta) + 1
    }

    /// Indica si la planta esta dentro del rango de plantas del edificio.
    pub fn contiene_la_planta(&self, planta: IdentificadorDePlanta) -> bool {
        self.planta_mas_baja <= planta && planta <= self.planta_mas_alta
    }

    /// Duracion total de un desplazamiento, incluidos el arranque y la parada.
    pub fn duracion_de_un_desplazamiento(&self, distancia: u32) -> Duration {
        self.tiempo_de_desplazamiento * distancia + self.tiempo_de_arranque_y_de_parada
    }

    /// Comprueba que la configuracion es valida y, si no lo es, devuelve el primer motivo encontrado.
    pub fn validar(&self) -> Result<(), ErrorDeConfiguracion> {
        if self.planta_mas_baja > self.planta_mas_alta {
            return Err(ErrorDeConfiguracion::PlantaMasBajaPorEncimaDeLaMasAlta);
        }
        if !self.contiene_la_planta(IdentificadorDePlanta::PRINCIPAL) {
            return Err(ErrorDeConfiguracion::PlantaPrincipalFueraDelRangoDePlantas);
        }
        if self.numero_de_ascensores == 0 {
            return Err(ErrorDeConfiguracion::SinAscensores);
        }
        if self.tiempo_de_desplazamiento.is_zero() {
            return Err(ErrorDeConfiguracion::TiempoDeDesplazamientoCero);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn configuracion_estandar_tiene_10_plantas_desde_la_menos_2_hasta_la_7() {
        let configuracion = ConfiguracionDelEdificio::estandar();
        assert_eq!(configuracion.planta_mas_baja, IdentificadorDePlanta(-2));
        assert_eq!(configuracion.planta_mas_alta, IdentificadorDePlanta(7));
        assert_eq!(configuracion.numero_de_plantas(), 10);
    }

    #[test]
    fn configuracion_estandar_tiene_3_ascensores() {
        assert_eq!(ConfiguracionDelEdificio::estandar().numero_de_ascensores, 3);
    }

    #[test]
    fn configuracion_estandar_tiene_tiempo_de_desplazamiento_de_2_segundos_y_de_arranque_y_de_parada_de_4_segundos()
     {
        let configuracion = ConfiguracionDelEdificio::estandar();
        assert_eq!(
            configuracion.tiempo_de_desplazamiento,
            Duration::from_secs(2)
        );
        assert_eq!(
            configuracion.tiempo_de_arranque_y_de_parada,
            Duration::from_secs(4)
        );
    }

    #[test]
    fn duracion_de_un_desplazamiento_es_distancia_por_tiempo_de_desplazamiento_mas_tiempo_de_arranque_y_de_parada()
     {
        let configuracion = ConfiguracionDelEdificio::estandar();
        assert_eq!(
            configuracion.duracion_de_un_desplazamiento(3),
            Duration::from_secs(10)
        );
    }
}
