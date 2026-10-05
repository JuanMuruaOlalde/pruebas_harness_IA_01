//! Simulador del edificio: raiz del agregado.

use std::time::Duration;

use super::ascensor::{Ascensor, IdentificadorDeAscensor, PosicionDeAscensor};
use super::configuracion::ConfiguracionDelEdificio;
use super::errores::{ErrorDeConfiguracion, ErrorDeSimulacion};
use super::planta::IdentificadorDePlanta;

/// Simulacion de un edificio con sus ascensores y un tiempo simulado propio.
#[derive(Debug, Clone)]
pub struct SimuladorDeEdificio {
    configuracion: ConfiguracionDelEdificio,
    /// Ordenados por identificador, del 1 al numero de ascensores.
    ascensores: Vec<Ascensor>,
    instante_actual: Duration,
}

impl SimuladorDeEdificio {
    /// Crea el simulador en el instante 0, con todos los ascensores parados en la planta principal.
    pub fn nuevo(configuracion: ConfiguracionDelEdificio) -> Result<Self, ErrorDeConfiguracion> {
        configuracion.validar()?;
        let ascensores = (1..=configuracion.numero_de_ascensores)
            .map(|numero_de_ascensor| {
                Ascensor::parado_en(
                    IdentificadorDeAscensor(numero_de_ascensor),
                    IdentificadorDePlanta::PRINCIPAL,
                )
            })
            .collect();
        Ok(Self {
            configuracion,
            ascensores,
            instante_actual: Duration::ZERO,
        })
    }

    pub fn configuracion(&self) -> &ConfiguracionDelEdificio {
        &self.configuracion
    }

    pub fn instante_actual(&self) -> Duration {
        self.instante_actual
    }

    /// Solo hace avanzar el instante actual: la posicion de cada ascensor se calcula al consultarla.
    pub fn avanzar_tiempo(&mut self, duracion: Duration) {
        self.instante_actual += duracion;
    }

    /// Ordena a un ascensor que se desplace, desde el instante actual, hasta la planta de destino.
    /// Comprueba, por este orden, que el ascensor existe, que la planta existe y que el ascensor
    /// no esta ocupado. Una orden rechazada no cambia nada.
    pub fn ordenar_mover_ascensor(
        &mut self,
        identificador_de_ascensor: IdentificadorDeAscensor,
        planta_de_destino: IdentificadorDePlanta,
    ) -> Result<(), ErrorDeSimulacion> {
        let indice = self.indice_del_ascensor(identificador_de_ascensor)?;
        if !self.configuracion.contiene_la_planta(planta_de_destino) {
            return Err(ErrorDeSimulacion::PlantaInexistente);
        }
        self.ascensores[indice].iniciar_desplazamiento(
            planta_de_destino,
            self.instante_actual,
            &self.configuracion,
        )
    }

    pub fn posicion_del_ascensor(
        &self,
        identificador_de_ascensor: IdentificadorDeAscensor,
    ) -> Result<PosicionDeAscensor, ErrorDeSimulacion> {
        let indice = self.indice_del_ascensor(identificador_de_ascensor)?;
        Ok(self.posicion_en_el_instante_actual(&self.ascensores[indice]))
    }

    /// Posicion de cada ascensor junto con su identificador, ordenadas por identificador.
    pub fn posiciones_de_todos_los_ascensores(
        &self,
    ) -> Vec<(IdentificadorDeAscensor, PosicionDeAscensor)> {
        self.ascensores
            .iter()
            .map(|ascensor| {
                (
                    ascensor.identificador(),
                    self.posicion_en_el_instante_actual(ascensor),
                )
            })
            .collect()
    }

    fn posicion_en_el_instante_actual(&self, ascensor: &Ascensor) -> PosicionDeAscensor {
        ascensor.posicion_en(self.instante_actual, &self.configuracion)
    }

    fn indice_del_ascensor(
        &self,
        identificador_de_ascensor: IdentificadorDeAscensor,
    ) -> Result<usize, ErrorDeSimulacion> {
        self.ascensores
            .iter()
            .position(|ascensor| ascensor.identificador() == identificador_de_ascensor)
            .ok_or(ErrorDeSimulacion::AscensorInexistente)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dominio::ascensor::EstadoDeAscensor;

    const ASCENSOR_1: IdentificadorDeAscensor = IdentificadorDeAscensor(1);
    const ASCENSOR_2: IdentificadorDeAscensor = IdentificadorDeAscensor(2);
    const ASCENSOR_3: IdentificadorDeAscensor = IdentificadorDeAscensor(3);

    fn planta(numero: i32) -> IdentificadorDePlanta {
        IdentificadorDePlanta(numero)
    }

    fn segundos(numero: u64) -> Duration {
        Duration::from_secs(numero)
    }

    fn milisegundos(numero: u64) -> Duration {
        Duration::from_millis(numero)
    }

    fn simulador_estandar() -> SimuladorDeEdificio {
        SimuladorDeEdificio::nuevo(ConfiguracionDelEdificio::estandar()).unwrap()
    }

    fn parado_en(numero: i32) -> PosicionDeAscensor {
        PosicionDeAscensor {
            planta_actual: planta(numero),
            estado: EstadoDeAscensor::Parado,
        }
    }

    fn desplazandose_en(planta_actual: i32, planta_de_destino: i32) -> PosicionDeAscensor {
        PosicionDeAscensor {
            planta_actual: planta(planta_actual),
            estado: EstadoDeAscensor::Desplazandose {
                planta_de_destino: planta(planta_de_destino),
            },
        }
    }

    fn posicion(
        simulador: &SimuladorDeEdificio,
        ascensor: IdentificadorDeAscensor,
    ) -> PosicionDeAscensor {
        simulador.posicion_del_ascensor(ascensor).unwrap()
    }

    // Creacion del simulador: validacion de la configuracion

    #[test]
    fn crear_simulador_con_planta_mas_baja_por_encima_de_la_mas_alta_da_error_de_configuracion() {
        let configuracion = ConfiguracionDelEdificio {
            planta_mas_baja: planta(3),
            planta_mas_alta: planta(-3),
            ..ConfiguracionDelEdificio::estandar()
        };
        assert_eq!(
            SimuladorDeEdificio::nuevo(configuracion).err(),
            Some(ErrorDeConfiguracion::PlantaMasBajaPorEncimaDeLaMasAlta)
        );
    }

    #[test]
    fn crear_simulador_sin_la_planta_principal_en_el_rango_de_plantas_da_error_de_configuracion() {
        let configuracion = ConfiguracionDelEdificio {
            planta_mas_baja: planta(1),
            planta_mas_alta: planta(5),
            ..ConfiguracionDelEdificio::estandar()
        };
        assert_eq!(
            SimuladorDeEdificio::nuevo(configuracion).err(),
            Some(ErrorDeConfiguracion::PlantaPrincipalFueraDelRangoDePlantas)
        );
    }

    #[test]
    fn crear_simulador_sin_ascensores_da_error_de_configuracion() {
        let configuracion = ConfiguracionDelEdificio {
            numero_de_ascensores: 0,
            ..ConfiguracionDelEdificio::estandar()
        };
        assert_eq!(
            SimuladorDeEdificio::nuevo(configuracion).err(),
            Some(ErrorDeConfiguracion::SinAscensores)
        );
    }

    #[test]
    fn crear_simulador_con_tiempo_de_desplazamiento_cero_da_error_de_configuracion() {
        let configuracion = ConfiguracionDelEdificio {
            tiempo_de_desplazamiento: Duration::ZERO,
            ..ConfiguracionDelEdificio::estandar()
        };
        assert_eq!(
            SimuladorDeEdificio::nuevo(configuracion).err(),
            Some(ErrorDeConfiguracion::TiempoDeDesplazamientoCero)
        );
    }

    #[test]
    fn crear_simulador_con_tiempo_de_arranque_y_de_parada_cero_es_valido() {
        let configuracion = ConfiguracionDelEdificio {
            tiempo_de_arranque_y_de_parada: Duration::ZERO,
            ..ConfiguracionDelEdificio::estandar()
        };
        assert!(SimuladorDeEdificio::nuevo(configuracion).is_ok());
    }

    // Estado inicial y consulta de posiciones

    #[test]
    fn al_iniciar_la_simulacion_el_instante_actual_es_cero() {
        assert_eq!(simulador_estandar().instante_actual(), Duration::ZERO);
    }

    #[test]
    fn al_iniciar_la_simulacion_todos_los_ascensores_estan_parados_en_la_planta_principal() {
        let simulador = simulador_estandar();
        for ascensor in [ASCENSOR_1, ASCENSOR_2, ASCENSOR_3] {
            assert_eq!(posicion(&simulador, ascensor), parado_en(0));
        }
    }

    #[test]
    fn posiciones_de_todos_los_ascensores_devuelve_una_por_ascensor_ordenadas_por_identificador() {
        let mut simulador = simulador_estandar();
        simulador
            .ordenar_mover_ascensor(ASCENSOR_2, planta(5))
            .unwrap();
        assert_eq!(
            simulador.posiciones_de_todos_los_ascensores(),
            vec![
                (ASCENSOR_1, parado_en(0)),
                (ASCENSOR_2, desplazandose_en(0, 5)),
                (ASCENSOR_3, parado_en(0)),
            ]
        );
    }

    #[test]
    fn consultar_posicion_de_ascensor_con_identificador_cero_da_error_de_ascensor_inexistente() {
        assert_eq!(
            simulador_estandar().posicion_del_ascensor(IdentificadorDeAscensor(0)),
            Err(ErrorDeSimulacion::AscensorInexistente)
        );
    }

    #[test]
    fn consultar_posicion_de_ascensor_con_identificador_mayor_que_el_numero_de_ascensores_da_error_de_ascensor_inexistente()
     {
        assert_eq!(
            simulador_estandar().posicion_del_ascensor(IdentificadorDeAscensor(4)),
            Err(ErrorDeSimulacion::AscensorInexistente)
        );
    }

    // Avance del tiempo simulado

    #[test]
    fn avanzar_tiempo_suma_la_duracion_al_instante_actual() {
        let mut simulador = simulador_estandar();
        simulador.avanzar_tiempo(segundos(3));
        simulador.avanzar_tiempo(segundos(4));
        assert_eq!(simulador.instante_actual(), segundos(7));
    }

    #[test]
    fn avanzar_tiempo_cero_no_cambia_nada() {
        let mut simulador = simulador_estandar();
        simulador
            .ordenar_mover_ascensor(ASCENSOR_1, planta(3))
            .unwrap();
        simulador.avanzar_tiempo(segundos(4));
        let antes = simulador.posiciones_de_todos_los_ascensores();
        simulador.avanzar_tiempo(Duration::ZERO);
        assert_eq!(simulador.instante_actual(), segundos(4));
        assert_eq!(simulador.posiciones_de_todos_los_ascensores(), antes);
    }

    #[test]
    fn avanzar_tiempo_sin_ordenes_no_mueve_ningun_ascensor() {
        let mut simulador = simulador_estandar();
        simulador.avanzar_tiempo(segundos(100));
        for ascensor in [ASCENSOR_1, ASCENSOR_2, ASCENSOR_3] {
            assert_eq!(posicion(&simulador, ascensor), parado_en(0));
        }
    }

    // Orden de mover un ascensor: validaciones

    #[test]
    fn ordenar_mover_ascensor_inexistente_da_error_de_ascensor_inexistente() {
        let mut simulador = simulador_estandar();
        assert_eq!(
            simulador.ordenar_mover_ascensor(IdentificadorDeAscensor(4), planta(1)),
            Err(ErrorDeSimulacion::AscensorInexistente)
        );
        assert_eq!(
            simulador.ordenar_mover_ascensor(IdentificadorDeAscensor(0), planta(1)),
            Err(ErrorDeSimulacion::AscensorInexistente)
        );
    }

    #[test]
    fn ordenar_mover_ascensor_a_planta_por_encima_de_la_mas_alta_da_error_de_planta_inexistente() {
        assert_eq!(
            simulador_estandar().ordenar_mover_ascensor(ASCENSOR_1, planta(8)),
            Err(ErrorDeSimulacion::PlantaInexistente)
        );
    }

    #[test]
    fn ordenar_mover_ascensor_a_planta_por_debajo_de_la_mas_baja_da_error_de_planta_inexistente() {
        assert_eq!(
            simulador_estandar().ordenar_mover_ascensor(ASCENSOR_1, planta(-3)),
            Err(ErrorDeSimulacion::PlantaInexistente)
        );
    }

    #[test]
    fn ordenar_mover_ascensor_a_la_planta_mas_alta_es_aceptado() {
        assert_eq!(
            simulador_estandar().ordenar_mover_ascensor(ASCENSOR_1, planta(7)),
            Ok(())
        );
    }

    #[test]
    fn ordenar_mover_ascensor_a_la_planta_mas_baja_es_aceptado() {
        assert_eq!(
            simulador_estandar().ordenar_mover_ascensor(ASCENSOR_1, planta(-2)),
            Ok(())
        );
    }

    #[test]
    fn ordenar_mover_ascensor_desplazandose_da_error_de_ascensor_ocupado() {
        let mut simulador = simulador_estandar();
        simulador
            .ordenar_mover_ascensor(ASCENSOR_1, planta(3))
            .unwrap();
        simulador.avanzar_tiempo(segundos(5));
        assert_eq!(
            simulador.ordenar_mover_ascensor(ASCENSOR_1, planta(1)),
            Err(ErrorDeSimulacion::AscensorOcupado)
        );
    }

    #[test]
    fn ordenar_mover_ascensor_durante_su_parada_en_la_planta_de_destino_da_error_de_ascensor_ocupado()
     {
        let mut simulador = simulador_estandar();
        simulador
            .ordenar_mover_ascensor(ASCENSOR_1, planta(3))
            .unwrap();
        simulador.avanzar_tiempo(segundos(9));
        assert_eq!(posicion(&simulador, ASCENSOR_1), desplazandose_en(3, 3));
        assert_eq!(
            simulador.ordenar_mover_ascensor(ASCENSOR_1, planta(1)),
            Err(ErrorDeSimulacion::AscensorOcupado)
        );
    }

    #[test]
    fn una_orden_rechazada_no_altera_la_posicion_ni_el_estado_del_ascensor() {
        let mut simulador = simulador_estandar();
        simulador
            .ordenar_mover_ascensor(ASCENSOR_1, planta(3))
            .unwrap();
        simulador.avanzar_tiempo(segundos(5));
        let antes = simulador.posiciones_de_todos_los_ascensores();
        let _ = simulador.ordenar_mover_ascensor(ASCENSOR_1, planta(1));
        let _ = simulador.ordenar_mover_ascensor(ASCENSOR_2, planta(99));
        let _ = simulador.ordenar_mover_ascensor(IdentificadorDeAscensor(9), planta(1));
        assert_eq!(simulador.instante_actual(), segundos(5));
        assert_eq!(simulador.posiciones_de_todos_los_ascensores(), antes);
        simulador.avanzar_tiempo(segundos(5));
        assert_eq!(posicion(&simulador, ASCENSOR_1), parado_en(3));
    }

    #[test]
    fn ordenar_mover_ascensor_a_la_planta_en_la_que_esta_parado_lo_deja_parado_en_ella() {
        let mut simulador = simulador_estandar();
        assert_eq!(
            simulador.ordenar_mover_ascensor(ASCENSOR_1, planta(0)),
            Ok(())
        );
        assert_eq!(posicion(&simulador, ASCENSOR_1), parado_en(0));
        simulador.avanzar_tiempo(segundos(10));
        assert_eq!(posicion(&simulador, ASCENSOR_1), parado_en(0));
    }

    // Desplazamiento de un ascensor en el tiempo

    #[test]
    fn tras_ordenar_mover_un_ascensor_se_esta_desplazando_hacia_la_planta_de_destino_sin_haber_salido_de_la_de_origen()
     {
        let mut simulador = simulador_estandar();
        simulador
            .ordenar_mover_ascensor(ASCENSOR_1, planta(3))
            .unwrap();
        assert_eq!(posicion(&simulador, ASCENSOR_1), desplazandose_en(0, 3));
    }

    #[test]
    fn durante_el_arranque_el_ascensor_permanece_en_la_planta_de_origen() {
        let mut simulador = simulador_estandar();
        simulador
            .ordenar_mover_ascensor(ASCENSOR_1, planta(3))
            .unwrap();
        simulador.avanzar_tiempo(segundos(1));
        assert_eq!(posicion(&simulador, ASCENSOR_1), desplazandose_en(0, 3));
        simulador.avanzar_tiempo(milisegundos(2900));
        assert_eq!(simulador.instante_actual(), milisegundos(3900));
        assert_eq!(posicion(&simulador, ASCENSOR_1), desplazandose_en(0, 3));
    }

    #[test]
    fn el_ascensor_alcanza_cada_planta_intermedia_al_cumplirse_su_tiempo_de_desplazamiento() {
        let mut simulador = simulador_estandar();
        simulador
            .ordenar_mover_ascensor(ASCENSOR_1, planta(3))
            .unwrap();
        simulador.avanzar_tiempo(segundos(4));
        assert_eq!(posicion(&simulador, ASCENSOR_1), desplazandose_en(1, 3));
        simulador.avanzar_tiempo(segundos(2));
        assert_eq!(posicion(&simulador, ASCENSOR_1), desplazandose_en(2, 3));
        simulador.avanzar_tiempo(segundos(2));
        assert_eq!(posicion(&simulador, ASCENSOR_1), desplazandose_en(3, 3));
    }

    #[test]
    fn entre_dos_plantas_la_planta_actual_es_la_ultima_alcanzada() {
        let mut simulador = simulador_estandar();
        simulador
            .ordenar_mover_ascensor(ASCENSOR_1, planta(3))
            .unwrap();
        simulador.avanzar_tiempo(milisegundos(5999));
        assert_eq!(posicion(&simulador, ASCENSOR_1), desplazandose_en(1, 3));
        simulador.avanzar_tiempo(milisegundos(1));
        assert_eq!(posicion(&simulador, ASCENSOR_1), desplazandose_en(2, 3));
    }

    #[test]
    fn durante_la_parada_el_ascensor_esta_en_la_planta_de_destino_pero_sigue_desplazandose() {
        let mut simulador = simulador_estandar();
        simulador
            .ordenar_mover_ascensor(ASCENSOR_1, planta(3))
            .unwrap();
        simulador.avanzar_tiempo(segundos(8));
        assert_eq!(posicion(&simulador, ASCENSOR_1), desplazandose_en(3, 3));
        simulador.avanzar_tiempo(milisegundos(1999));
        assert_eq!(posicion(&simulador, ASCENSOR_1), desplazandose_en(3, 3));
    }

    #[test]
    fn al_completar_el_desplazamiento_el_ascensor_queda_parado_en_la_planta_de_destino() {
        let mut simulador = simulador_estandar();
        simulador
            .ordenar_mover_ascensor(ASCENSOR_1, planta(3))
            .unwrap();
        simulador.avanzar_tiempo(segundos(10));
        assert_eq!(posicion(&simulador, ASCENSOR_1), parado_en(3));
    }

    #[test]
    fn el_ascensor_baja_planta_a_planta_hasta_una_planta_por_debajo_de_la_principal() {
        let mut simulador = simulador_estandar();
        simulador
            .ordenar_mover_ascensor(ASCENSOR_1, planta(-2))
            .unwrap();
        simulador.avanzar_tiempo(segundos(3));
        assert_eq!(posicion(&simulador, ASCENSOR_1), desplazandose_en(0, -2));
        simulador.avanzar_tiempo(segundos(1));
        assert_eq!(posicion(&simulador, ASCENSOR_1), desplazandose_en(-1, -2));
        simulador.avanzar_tiempo(segundos(2));
        assert_eq!(posicion(&simulador, ASCENSOR_1), desplazandose_en(-2, -2));
        simulador.avanzar_tiempo(segundos(2));
        assert_eq!(posicion(&simulador, ASCENSOR_1), parado_en(-2));
    }

    #[test]
    fn con_tiempo_de_arranque_y_de_parada_cero_el_ascensor_queda_parado_al_alcanzar_la_planta_de_destino()
     {
        let configuracion = ConfiguracionDelEdificio {
            tiempo_de_arranque_y_de_parada: Duration::ZERO,
            ..ConfiguracionDelEdificio::estandar()
        };
        let mut simulador = SimuladorDeEdificio::nuevo(configuracion).unwrap();
        simulador
            .ordenar_mover_ascensor(ASCENSOR_1, planta(2))
            .unwrap();
        simulador.avanzar_tiempo(segundos(2));
        assert_eq!(posicion(&simulador, ASCENSOR_1), desplazandose_en(1, 2));
        simulador.avanzar_tiempo(segundos(2));
        assert_eq!(posicion(&simulador, ASCENSOR_1), parado_en(2));
    }

    #[test]
    fn avanzar_tiempo_admite_fracciones_de_segundo() {
        let mut simulador = simulador_estandar();
        simulador
            .ordenar_mover_ascensor(ASCENSOR_1, planta(1))
            .unwrap();
        simulador.avanzar_tiempo(milisegundos(500));
        simulador.avanzar_tiempo(milisegundos(250));
        assert_eq!(simulador.instante_actual(), milisegundos(750));
        simulador.avanzar_tiempo(milisegundos(3250));
        assert_eq!(posicion(&simulador, ASCENSOR_1), desplazandose_en(1, 1));
    }

    #[test]
    fn avanzar_tiempo_de_una_vez_mas_alla_del_final_deja_el_ascensor_parado_en_la_planta_de_destino()
     {
        let mut simulador = simulador_estandar();
        simulador
            .ordenar_mover_ascensor(ASCENSOR_1, planta(7))
            .unwrap();
        simulador.avanzar_tiempo(segundos(1000));
        assert_eq!(posicion(&simulador, ASCENSOR_1), parado_en(7));
    }

    #[test]
    fn avanzar_tiempo_en_varios_pasos_da_la_misma_posicion_que_avanzarlo_de_una_vez() {
        let mut de_una_vez = simulador_estandar();
        let mut en_pasos = simulador_estandar();
        de_una_vez
            .ordenar_mover_ascensor(ASCENSOR_1, planta(6))
            .unwrap();
        en_pasos
            .ordenar_mover_ascensor(ASCENSOR_1, planta(6))
            .unwrap();
        for _ in 0..1000 {
            en_pasos.avanzar_tiempo(milisegundos(16));
        }
        de_una_vez.avanzar_tiempo(milisegundos(16_000));
        assert_eq!(
            en_pasos.posiciones_de_todos_los_ascensores(),
            de_una_vez.posiciones_de_todos_los_ascensores()
        );
        let mut a_medias = simulador_estandar();
        let mut de_golpe = simulador_estandar();
        a_medias
            .ordenar_mover_ascensor(ASCENSOR_1, planta(6))
            .unwrap();
        de_golpe
            .ordenar_mover_ascensor(ASCENSOR_1, planta(6))
            .unwrap();
        for _ in 0..7 {
            a_medias.avanzar_tiempo(milisegundos(1000));
        }
        de_golpe.avanzar_tiempo(segundos(7));
        assert_eq!(
            posicion(&a_medias, ASCENSOR_1),
            posicion(&de_golpe, ASCENSOR_1)
        );
        assert_eq!(posicion(&a_medias, ASCENSOR_1), desplazandose_en(2, 6));
    }

    #[test]
    fn el_desplazamiento_se_cuenta_desde_el_instante_en_que_se_ordena() {
        let mut simulador = simulador_estandar();
        simulador.avanzar_tiempo(segundos(100));
        simulador
            .ordenar_mover_ascensor(ASCENSOR_1, planta(3))
            .unwrap();
        assert_eq!(posicion(&simulador, ASCENSOR_1), desplazandose_en(0, 3));
        simulador.avanzar_tiempo(segundos(4));
        assert_eq!(posicion(&simulador, ASCENSOR_1), desplazandose_en(1, 3));
        simulador.avanzar_tiempo(segundos(6));
        assert_eq!(posicion(&simulador, ASCENSOR_1), parado_en(3));
    }

    #[test]
    fn un_ascensor_que_ha_completado_su_desplazamiento_puede_desplazarse_de_nuevo_desde_la_planta_alcanzada()
     {
        let mut simulador = simulador_estandar();
        simulador
            .ordenar_mover_ascensor(ASCENSOR_1, planta(3))
            .unwrap();
        simulador.avanzar_tiempo(segundos(10));
        assert_eq!(
            simulador.ordenar_mover_ascensor(ASCENSOR_1, planta(1)),
            Ok(())
        );
        assert_eq!(posicion(&simulador, ASCENSOR_1), desplazandose_en(3, 1));
        simulador.avanzar_tiempo(segundos(4));
        assert_eq!(posicion(&simulador, ASCENSOR_1), desplazandose_en(2, 1));
        simulador.avanzar_tiempo(segundos(4));
        assert_eq!(posicion(&simulador, ASCENSOR_1), parado_en(1));
    }

    // Independencia entre ascensores

    #[test]
    fn mover_un_ascensor_no_altera_la_posicion_de_los_demas() {
        let mut simulador = simulador_estandar();
        simulador
            .ordenar_mover_ascensor(ASCENSOR_2, planta(5))
            .unwrap();
        simulador.avanzar_tiempo(segundos(20));
        assert_eq!(posicion(&simulador, ASCENSOR_1), parado_en(0));
        assert_eq!(posicion(&simulador, ASCENSOR_2), parado_en(5));
        assert_eq!(posicion(&simulador, ASCENSOR_3), parado_en(0));
    }

    #[test]
    fn varios_ascensores_se_desplazan_a_la_vez_de_forma_independiente() {
        let mut simulador = simulador_estandar();
        simulador
            .ordenar_mover_ascensor(ASCENSOR_1, planta(3))
            .unwrap();
        simulador
            .ordenar_mover_ascensor(ASCENSOR_2, planta(-2))
            .unwrap();
        simulador
            .ordenar_mover_ascensor(ASCENSOR_3, planta(7))
            .unwrap();
        simulador.avanzar_tiempo(segundos(8));
        assert_eq!(posicion(&simulador, ASCENSOR_1), desplazandose_en(3, 3));
        assert_eq!(posicion(&simulador, ASCENSOR_2), parado_en(-2));
        assert_eq!(posicion(&simulador, ASCENSOR_3), desplazandose_en(3, 7));
    }

    #[test]
    fn ascensores_que_reciben_la_orden_en_instantes_distintos_avanzan_cada_uno_segun_su_propio_inicio()
     {
        let mut simulador = simulador_estandar();
        simulador
            .ordenar_mover_ascensor(ASCENSOR_1, planta(3))
            .unwrap();
        simulador.avanzar_tiempo(segundos(4));
        simulador
            .ordenar_mover_ascensor(ASCENSOR_2, planta(3))
            .unwrap();
        simulador.avanzar_tiempo(segundos(4));
        assert_eq!(posicion(&simulador, ASCENSOR_1), desplazandose_en(3, 3));
        assert_eq!(posicion(&simulador, ASCENSOR_2), desplazandose_en(1, 3));
        simulador.avanzar_tiempo(segundos(2));
        assert_eq!(posicion(&simulador, ASCENSOR_1), parado_en(3));
        assert_eq!(posicion(&simulador, ASCENSOR_2), desplazandose_en(2, 3));
    }
}
