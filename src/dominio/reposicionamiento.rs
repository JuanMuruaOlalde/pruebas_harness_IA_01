//! Optimizacion de la posicion de los ascensores libres segun la demanda historica.

use std::cmp::Reverse;
use std::collections::HashMap;

use super::ascensor::IdentificadorDeAscensor;
use super::configuracion::ConfiguracionDelEdificio;
use super::fecha_y_hora::FechaYHora;
use super::movimiento::{MotivoDeMovimiento, MovimientoDeAscensor};
use super::planta::IdentificadorDePlanta;

/// Orden de llevar un ascensor en reposo a una planta de espera.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OrdenDeReposicionamiento {
    pub identificador_de_ascensor: IdentificadorDeAscensor,
    pub planta_de_espera: IdentificadorDePlanta,
}

/// Plantas con demanda (llamadas del mismo dia de la semana y de la misma franja horaria que la
/// fecha y hora actual), de mas a menos demanda. A igual demanda, primero la mas cercana a la
/// planta principal y, si tambien empatan, la de menor identificador. Se ignoran las plantas que
/// no existen en el edificio. No filtra por antiguedad: eso lo hace la consulta al historico.
pub fn plantas_de_espera_preferentes(
    movimientos: &[MovimientoDeAscensor],
    fecha_y_hora_actual: FechaYHora,
    configuracion_del_edificio: &ConfiguracionDelEdificio,
) -> Vec<IdentificadorDePlanta> {
    let demanda_por_planta =
        demanda_por_planta(movimientos, fecha_y_hora_actual, configuracion_del_edificio);
    let mut plantas_con_demanda: Vec<IdentificadorDePlanta> =
        demanda_por_planta.keys().copied().collect();
    plantas_con_demanda.sort_by_key(|planta| {
        (
            Reverse(demanda_por_planta[planta]),
            planta.distancia_a(IdentificadorDePlanta::PRINCIPAL),
            *planta,
        )
    });
    plantas_con_demanda
}

/// Numero de llamadas hechas en cada planta del edificio el mismo dia de la semana y en la misma
/// franja horaria que la fecha y hora actual. Las plantas sin llamadas no aparecen.
fn demanda_por_planta(
    movimientos: &[MovimientoDeAscensor],
    fecha_y_hora_actual: FechaYHora,
    configuracion_del_edificio: &ConfiguracionDelEdificio,
) -> HashMap<IdentificadorDePlanta, usize> {
    let mut demanda_por_planta = HashMap::new();
    let llamadas_que_cuentan_como_demanda = movimientos.iter().filter(|movimiento| {
        movimiento.motivo == MotivoDeMovimiento::Llamada
            && son_del_mismo_dia_de_la_semana_y_franja_horaria(
                movimiento.fecha_y_hora,
                fecha_y_hora_actual,
            )
            && configuracion_del_edificio.contiene_la_planta(movimiento.planta_de_destino)
    });
    for llamada in llamadas_que_cuentan_como_demanda {
        *demanda_por_planta
            .entry(llamada.planta_de_destino)
            .or_insert(0) += 1;
    }
    demanda_por_planta
}

fn son_del_mismo_dia_de_la_semana_y_franja_horaria(una: FechaYHora, otra: FechaYHora) -> bool {
    una.dia_de_la_semana() == otra.dia_de_la_semana() && una.hora_del_dia() == otra.hora_del_dia()
}

/// Reparte los ascensores en reposo entre las plantas de espera preferentes que no cubre ya otro
/// ascensor. Devuelve solo las ordenes que mueven un ascensor, ordenadas por identificador.
pub fn plan_de_reposicionamiento(
    plantas_de_espera_preferentes: &[IdentificadorDePlanta],
    ascensores_en_reposo: &[(IdentificadorDeAscensor, IdentificadorDePlanta)],
    plantas_cubiertas_por_otros_ascensores: &[IdentificadorDePlanta],
) -> Vec<OrdenDeReposicionamiento> {
    // Las de mas prioridad que no cubre otro ascensor, tantas como ascensores en reposo haya.
    let plantas_a_cubrir: Vec<IdentificadorDePlanta> = plantas_de_espera_preferentes
        .iter()
        .filter(|planta| !plantas_cubiertas_por_otros_ascensores.contains(planta))
        .take(ascensores_en_reposo.len())
        .copied()
        .collect();

    // Ordenados por identificador para que, a igualdad, se escoja siempre el de menor identificador.
    let mut ascensores_disponibles = ascensores_en_reposo.to_vec();
    ascensores_disponibles.sort_by_key(|(identificador, _)| *identificador);

    // Cada ascensor que ya esta en una planta a cubrir se queda en ella.
    let mut plantas_sin_ascensor = Vec::new();
    for planta_a_cubrir in plantas_a_cubrir {
        match indice_del_primer_ascensor_en_la_planta(&ascensores_disponibles, planta_a_cubrir) {
            Some(indice_del_ascensor_que_se_queda) => {
                ascensores_disponibles.remove(indice_del_ascensor_que_se_queda);
            }
            None => plantas_sin_ascensor.push(planta_a_cubrir),
        }
    }

    // Las plantas que quedan, por orden de prioridad, van al ascensor disponible mas cercano.
    let mut ordenes = Vec::new();
    for planta_de_espera in plantas_sin_ascensor {
        let Some(indice_del_ascensor_mas_cercano) =
            indice_del_ascensor_mas_cercano(&ascensores_disponibles, planta_de_espera)
        else {
            break;
        };
        let (identificador_de_ascensor, _) =
            ascensores_disponibles.remove(indice_del_ascensor_mas_cercano);
        ordenes.push(OrdenDeReposicionamiento {
            identificador_de_ascensor,
            planta_de_espera,
        });
    }
    ordenes.sort_by_key(|orden| orden.identificador_de_ascensor);
    ordenes
}

fn indice_del_primer_ascensor_en_la_planta(
    ascensores: &[(IdentificadorDeAscensor, IdentificadorDePlanta)],
    planta: IdentificadorDePlanta,
) -> Option<usize> {
    ascensores
        .iter()
        .position(|(_, planta_actual)| *planta_actual == planta)
}

/// A igual distancia, el de menor identificador.
fn indice_del_ascensor_mas_cercano(
    ascensores: &[(IdentificadorDeAscensor, IdentificadorDePlanta)],
    planta: IdentificadorDePlanta,
) -> Option<usize> {
    ascensores
        .iter()
        .enumerate()
        .min_by_key(|(_, (identificador, planta_actual))| {
            (planta_actual.distancia_a(planta), *identificador)
        })
        .map(|(indice, _)| indice)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn planta(numero: i32) -> IdentificadorDePlanta {
        IdentificadorDePlanta(numero)
    }

    fn ascensor(numero: u32) -> IdentificadorDeAscensor {
        IdentificadorDeAscensor(numero)
    }

    fn lunes_a_las_8() -> FechaYHora {
        FechaYHora::nueva(2026, 10, 5, 8, 0, 0).unwrap()
    }

    fn movimiento(
        fecha_y_hora: FechaYHora,
        planta_de_origen: i32,
        planta_de_destino: i32,
        motivo: MotivoDeMovimiento,
    ) -> MovimientoDeAscensor {
        MovimientoDeAscensor {
            fecha_y_hora,
            identificador_de_ascensor: ascensor(1),
            planta_de_origen: planta(planta_de_origen),
            planta_de_destino: planta(planta_de_destino),
            motivo,
        }
    }

    /// Llamada hecha el lunes anterior (28 de septiembre) a las 08:30.
    fn llamada_en(planta_de_destino: i32) -> MovimientoDeAscensor {
        let lunes_anterior = FechaYHora::nueva(2026, 9, 28, 8, 30, 0).unwrap();
        movimiento(
            lunes_anterior,
            0,
            planta_de_destino,
            MotivoDeMovimiento::Llamada,
        )
    }

    fn preferentes(movimientos: &[MovimientoDeAscensor]) -> Vec<IdentificadorDePlanta> {
        plantas_de_espera_preferentes(
            movimientos,
            lunes_a_las_8(),
            &ConfiguracionDelEdificio::estandar(),
        )
    }

    fn plantas(numeros: &[i32]) -> Vec<IdentificadorDePlanta> {
        numeros.iter().map(|numero| planta(*numero)).collect()
    }

    fn en_reposo(pares: &[(u32, i32)]) -> Vec<(IdentificadorDeAscensor, IdentificadorDePlanta)> {
        pares
            .iter()
            .map(|(identificador, numero)| (ascensor(*identificador), planta(*numero)))
            .collect()
    }

    fn orden(identificador: u32, planta_de_espera: i32) -> OrdenDeReposicionamiento {
        OrdenDeReposicionamiento {
            identificador_de_ascensor: ascensor(identificador),
            planta_de_espera: planta(planta_de_espera),
        }
    }

    // Plantas de espera preferentes

    #[test]
    fn sin_movimientos_no_hay_plantas_de_espera_preferentes() {
        assert_eq!(preferentes(&[]), vec![]);
    }

    #[test]
    fn solo_cuentan_como_demanda_los_movimientos_por_llamada() {
        let fecha = FechaYHora::nueva(2026, 9, 28, 8, 30, 0).unwrap();
        let movimientos = [
            movimiento(fecha, 0, 5, MotivoDeMovimiento::Botonera),
            movimiento(fecha, 0, 6, MotivoDeMovimiento::Reposicionamiento),
            llamada_en(2),
        ];
        assert_eq!(preferentes(&movimientos), plantas(&[2]));
    }

    #[test]
    fn solo_cuentan_las_llamadas_del_mismo_dia_de_la_semana() {
        let martes = FechaYHora::nueva(2026, 9, 29, 8, 30, 0).unwrap();
        let movimientos = [
            movimiento(martes, 0, 6, MotivoDeMovimiento::Llamada),
            llamada_en(2),
        ];
        assert_eq!(preferentes(&movimientos), plantas(&[2]));
    }

    #[test]
    fn solo_cuentan_las_llamadas_de_la_misma_franja_horaria() {
        let a_las_9 = FechaYHora::nueva(2026, 9, 28, 9, 30, 0).unwrap();
        let a_las_7 = FechaYHora::nueva(2026, 9, 28, 7, 59, 59).unwrap();
        let movimientos = [
            movimiento(a_las_9, 0, 6, MotivoDeMovimiento::Llamada),
            movimiento(a_las_7, 0, 7, MotivoDeMovimiento::Llamada),
            llamada_en(2),
        ];
        assert_eq!(preferentes(&movimientos), plantas(&[2]));
    }

    #[test]
    fn la_planta_que_recibe_la_demanda_es_la_de_la_llamada_y_no_la_de_origen_del_ascensor() {
        let fecha = FechaYHora::nueva(2026, 9, 28, 8, 30, 0).unwrap();
        let movimientos = [movimiento(fecha, 6, 3, MotivoDeMovimiento::Llamada)];
        assert_eq!(preferentes(&movimientos), plantas(&[3]));
    }

    #[test]
    fn las_llamadas_del_mismo_dia_y_franja_de_semanas_distintas_se_suman() {
        let semanas = [(9, 14), (9, 21), (9, 28)];
        let mut movimientos: Vec<MovimientoDeAscensor> = semanas
            .iter()
            .map(|(mes, dia)| {
                let fecha = FechaYHora::nueva(2026, *mes, *dia, 8, 10, 0).unwrap();
                movimiento(fecha, 0, 4, MotivoDeMovimiento::Llamada)
            })
            .collect();
        movimientos.push(llamada_en(1));
        movimientos.push(llamada_en(1));
        assert_eq!(preferentes(&movimientos), plantas(&[4, 1]));
    }

    #[test]
    fn las_plantas_de_espera_preferentes_se_ordenan_de_mas_a_menos_llamadas() {
        let movimientos = [
            llamada_en(1),
            llamada_en(5),
            llamada_en(5),
            llamada_en(5),
            llamada_en(7),
            llamada_en(7),
        ];
        assert_eq!(preferentes(&movimientos), plantas(&[5, 7, 1]));
    }

    #[test]
    fn a_igual_numero_de_llamadas_se_prefiere_la_planta_mas_cercana_a_la_principal() {
        // La -2 esta mas lejos de la principal que la -1, aunque tenga menor identificador.
        let movimientos = [llamada_en(6), llamada_en(-1), llamada_en(3), llamada_en(-2)];
        assert_eq!(preferentes(&movimientos), plantas(&[-1, -2, 3, 6]));
    }

    #[test]
    fn a_igual_numero_de_llamadas_y_de_distancia_a_la_principal_se_prefiere_la_planta_de_menor_identificador()
     {
        let movimientos = [llamada_en(2), llamada_en(-2)];
        assert_eq!(preferentes(&movimientos), plantas(&[-2, 2]));
    }

    #[test]
    fn las_llamadas_a_plantas_que_no_existen_en_el_edificio_se_ignoran() {
        let movimientos = [llamada_en(8), llamada_en(8), llamada_en(-3), llamada_en(4)];
        assert_eq!(preferentes(&movimientos), plantas(&[4]));
    }

    // Plan de reposicionamiento

    #[test]
    fn sin_plantas_de_espera_preferentes_ningun_ascensor_se_reposiciona() {
        let plan = plan_de_reposicionamiento(&[], &en_reposo(&[(1, 0), (2, 3)]), &[]);
        assert_eq!(plan, vec![]);
    }

    #[test]
    fn sin_ascensores_en_reposo_no_hay_reposicionamientos() {
        let plan = plan_de_reposicionamiento(&plantas(&[5, 2]), &[], &[]);
        assert_eq!(plan, vec![]);
    }

    #[test]
    fn un_ascensor_en_reposo_va_a_la_planta_de_espera_de_mas_prioridad() {
        let plan = plan_de_reposicionamiento(&plantas(&[5, 2]), &en_reposo(&[(1, 0)]), &[]);
        assert_eq!(plan, vec![orden(1, 5)]);
    }

    #[test]
    fn con_menos_ascensores_en_reposo_que_plantas_de_espera_solo_se_cubren_las_de_mas_prioridad() {
        let plan =
            plan_de_reposicionamiento(&plantas(&[5, 2, -1]), &en_reposo(&[(1, 0), (2, 0)]), &[]);
        assert_eq!(plan, vec![orden(1, 5), orden(2, 2)]);
    }

    #[test]
    fn con_mas_ascensores_en_reposo_que_plantas_de_espera_los_sobrantes_se_quedan_donde_estan() {
        let plan =
            plan_de_reposicionamiento(&plantas(&[5]), &en_reposo(&[(1, 0), (2, 0), (3, 0)]), &[]);
        assert_eq!(plan, vec![orden(1, 5)]);
    }

    #[test]
    fn los_ascensores_que_ya_estan_en_una_planta_a_cubrir_se_quedan_y_los_demas_cubren_las_restantes()
     {
        // El 2 (en la 4) y el 4 (en la 2) se quedan, aunque el 2 sea, junto con el 3, el mas
        // cercano a la 5, que tiene mas prioridad. La 5 y la -1 las cubren el 3 y el 1.
        let plan = plan_de_reposicionamiento(
            &plantas(&[5, 4, 2, -1]),
            &en_reposo(&[(1, 0), (2, 4), (3, 6), (4, 2)]),
            &[],
        );
        assert_eq!(plan, vec![orden(1, -1), orden(3, 5)]);
    }

    #[test]
    fn cada_planta_a_cubrir_se_asigna_al_ascensor_en_reposo_disponible_mas_cercano() {
        let plan = plan_de_reposicionamiento(
            &plantas(&[6, -2]),
            &en_reposo(&[(1, 0), (2, 5), (3, -1)]),
            &[],
        );
        assert_eq!(plan, vec![orden(2, 6), orden(3, -2)]);
    }

    #[test]
    fn a_igual_distancia_se_reposiciona_el_ascensor_de_menor_identificador() {
        let plan = plan_de_reposicionamiento(&plantas(&[3]), &en_reposo(&[(2, 1), (1, 5)]), &[]);
        assert_eq!(plan, vec![orden(1, 3)]);
    }

    #[test]
    fn si_varios_ascensores_en_reposo_estan_en_la_misma_planta_a_cubrir_solo_se_queda_el_de_menor_identificador()
     {
        let plan = plan_de_reposicionamiento(
            &plantas(&[0, 4]),
            &en_reposo(&[(3, 0), (1, 0), (2, 0)]),
            &[],
        );
        assert_eq!(plan, vec![orden(2, 4)]);
    }

    #[test]
    fn las_plantas_cubiertas_por_otros_ascensores_no_se_vuelven_a_cubrir() {
        let plan = plan_de_reposicionamiento(
            &plantas(&[5, 2]),
            &en_reposo(&[(1, 0), (2, 0)]),
            &plantas(&[5]),
        );
        assert_eq!(plan, vec![orden(1, 2)]);
    }
}
