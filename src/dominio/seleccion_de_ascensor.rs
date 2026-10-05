//! Seleccion del ascensor que atiende una llamada.

use super::ascensor::{EstadoDeAscensor, IdentificadorDeAscensor, PosicionDeAscensor};
use super::planta::IdentificadorDePlanta;

/// Ascensor libre (parado) mas cercano a la planta de la llamada; a igual distancia, el de menor
/// identificador. Devuelve `None` si no hay ningun ascensor libre.
pub fn ascensor_libre_mas_cercano(
    posiciones: &[(IdentificadorDeAscensor, PosicionDeAscensor)],
    planta_de_la_llamada: IdentificadorDePlanta,
) -> Option<IdentificadorDeAscensor> {
    posiciones
        .iter()
        .filter(|(_, posicion)| posicion.estado == EstadoDeAscensor::Parado)
        .min_by_key(|(identificador, posicion)| {
            (
                posicion.planta_actual.distancia_a(planta_de_la_llamada),
                *identificador,
            )
        })
        .map(|(identificador, _)| *identificador)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parado_en(identificador: u32, planta: i32) -> (IdentificadorDeAscensor, PosicionDeAscensor) {
        (
            IdentificadorDeAscensor(identificador),
            PosicionDeAscensor {
                planta_actual: IdentificadorDePlanta(planta),
                estado: EstadoDeAscensor::Parado,
            },
        )
    }

    fn desplazandose_en(
        identificador: u32,
        planta: i32,
        destino: i32,
    ) -> (IdentificadorDeAscensor, PosicionDeAscensor) {
        (
            IdentificadorDeAscensor(identificador),
            PosicionDeAscensor {
                planta_actual: IdentificadorDePlanta(planta),
                estado: EstadoDeAscensor::Desplazandose {
                    planta_de_destino: IdentificadorDePlanta(destino),
                },
            },
        )
    }

    fn planta(numero: i32) -> IdentificadorDePlanta {
        IdentificadorDePlanta(numero)
    }

    #[test]
    fn sin_ascensores_parados_no_hay_ascensor_libre_mas_cercano() {
        assert_eq!(ascensor_libre_mas_cercano(&[], planta(3)), None);
        let todos_desplazandose = [desplazandose_en(1, 0, 5), desplazandose_en(2, 2, -1)];
        assert_eq!(
            ascensor_libre_mas_cercano(&todos_desplazandose, planta(3)),
            None
        );
    }

    #[test]
    fn el_ascensor_libre_mas_cercano_es_el_parado_a_menor_distancia_de_la_planta_de_la_llamada() {
        let posiciones = [parado_en(1, 0), parado_en(2, 6), parado_en(3, -2)];
        assert_eq!(
            ascensor_libre_mas_cercano(&posiciones, planta(5)),
            Some(IdentificadorDeAscensor(2))
        );
    }

    #[test]
    fn un_ascensor_parado_en_la_planta_de_la_llamada_es_el_libre_mas_cercano() {
        let posiciones = [parado_en(1, 4), parado_en(2, 5), parado_en(3, 6)];
        assert_eq!(
            ascensor_libre_mas_cercano(&posiciones, planta(5)),
            Some(IdentificadorDeAscensor(2))
        );
    }

    #[test]
    fn los_ascensores_que_se_estan_desplazando_no_cuentan_como_libres_aunque_esten_mas_cerca() {
        let posiciones = [desplazandose_en(1, 4, 5), parado_en(2, -2), parado_en(3, 0)];
        assert_eq!(
            ascensor_libre_mas_cercano(&posiciones, planta(5)),
            Some(IdentificadorDeAscensor(3))
        );
    }

    #[test]
    fn a_igual_distancia_se_escoge_el_ascensor_libre_de_menor_identificador() {
        let posiciones = [parado_en(3, 2), parado_en(2, 4), parado_en(1, 7)];
        assert_eq!(
            ascensor_libre_mas_cercano(&posiciones, planta(3)),
            Some(IdentificadorDeAscensor(2))
        );
    }
}
