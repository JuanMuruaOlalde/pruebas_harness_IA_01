//! Control de trafico: decide que ascensor atiende cada llamada, registra los movimientos en el
//! historico y reposiciona los ascensores libres segun la demanda historica.

use std::collections::{HashMap, VecDeque};
use std::fmt;
use std::time::Duration;

use crate::dominio::ascensor::{EstadoDeAscensor, IdentificadorDeAscensor};
use crate::dominio::errores::ErrorDeSimulacion;
use crate::dominio::fecha_y_hora::FechaYHora;
use crate::dominio::historico_de_movimientos::{
    ErrorDeHistoricoDeMovimientos, HistoricoDeMovimientos,
};
use crate::dominio::movimiento::{MotivoDeMovimiento, MovimientoDeAscensor};
use crate::dominio::planta::IdentificadorDePlanta;
use crate::dominio::reposicionamiento::{plan_de_reposicionamiento, plantas_de_espera_preferentes};
use crate::dominio::seleccion_de_ascensor::ascensor_libre_mas_cercano;
use crate::dominio::simulador::SimuladorDeEdificio;

/// Datos que definen el comportamiento del control de trafico.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ConfiguracionDelControlDeTrafico {
    /// Cuantos dias hacia atras se consulta el historico para optimizar (el "ultimo mes").
    pub dias_de_historico_a_considerar: u32,
    /// Tiempo que un ascensor ha de estar libre antes de que se le pueda reposicionar.
    pub tiempo_de_reposo_antes_de_reposicionar: Duration,
}

impl ConfiguracionDelControlDeTrafico {
    /// 28 dias de historico (4 semanas completas) y 30 s de reposo.
    pub fn estandar() -> Self {
        Self {
            dias_de_historico_a_considerar: 28,
            tiempo_de_reposo_antes_de_reposicionar: Duration::from_secs(30),
        }
    }
}

/// Que ha pasado al pulsar un boton de llamada.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResultadoDeLaLlamada {
    /// Se ha enviado este ascensor a la planta (o ya estaba en ella).
    AscensorAsignado(IdentificadorDeAscensor),
    /// No hay ascensores libres: la llamada se atendera en cuanto quede libre alguno.
    PendienteDeAscensorLibre,
    /// La planta ya tenia una llamada pendiente o un ascensor se dirige a ella: no tiene efecto.
    LlamadaYaEnCurso,
}

/// Motivos por los que falla una operacion del control de trafico.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ErrorDeControlDeTrafico {
    /// El simulador ha rechazado la orden.
    Simulacion(ErrorDeSimulacion),
    /// El historico ha fallado. Las ordenes ya dadas a los ascensores no se deshacen.
    Historico(ErrorDeHistoricoDeMovimientos),
}

impl fmt::Display for ErrorDeControlDeTrafico {
    fn fmt(&self, formato: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Simulacion(error) => fmt::Display::fmt(error, formato),
            Self::Historico(error) => fmt::Display::fmt(error, formato),
        }
    }
}

/// `source()` queda por defecto (`None`): el mensaje ya incluye el del error interno.
impl std::error::Error for ErrorDeControlDeTrafico {}

impl From<ErrorDeSimulacion> for ErrorDeControlDeTrafico {
    fn from(error: ErrorDeSimulacion) -> Self {
        Self::Simulacion(error)
    }
}

impl From<ErrorDeHistoricoDeMovimientos> for ErrorDeControlDeTrafico {
    fn from(error: ErrorDeHistoricoDeMovimientos) -> Self {
        Self::Historico(error)
    }
}

/// Unico punto de entrada para mover ascensores: es el dueno del simulador.
pub struct ControlDeTrafico<H: HistoricoDeMovimientos> {
    simulador: SimuladorDeEdificio,
    historico: H,
    fecha_y_hora_de_inicio: FechaYHora,
    configuracion: ConfiguracionDelControlDeTrafico,
    /// Plantas con llamada pendiente, por orden de llegada: se atienden desde el principio.
    llamadas_pendientes: VecDeque<IdentificadorDePlanta>,
    /// Instante de simulacion desde el que esta libre cada ascensor (al terminar su ultimo
    /// desplazamiento ordenado, o al recibir su ultima orden si no tuvo que moverse).
    libre_desde_por_ascensor: HashMap<IdentificadorDeAscensor, Duration>,
}

impl<H: HistoricoDeMovimientos> ControlDeTrafico<H> {
    /// Se queda con el simulador, que normalmente esta recien creado, y sin llamadas pendientes.
    pub fn nuevo(
        simulador: SimuladorDeEdificio,
        historico: H,
        fecha_y_hora_de_inicio: FechaYHora,
        configuracion: ConfiguracionDelControlDeTrafico,
    ) -> Self {
        let instante_actual = simulador.instante_actual();
        let libre_desde_por_ascensor = simulador
            .posiciones_de_todos_los_ascensores()
            .into_iter()
            .map(|(identificador, _)| (identificador, instante_actual))
            .collect();
        Self {
            simulador,
            historico,
            fecha_y_hora_de_inicio,
            configuracion,
            llamadas_pendientes: VecDeque::new(),
            libre_desde_por_ascensor,
        }
    }

    /// Solo lectura: los ascensores se mueven a traves del control de trafico.
    pub fn simulador(&self) -> &SimuladorDeEdificio {
        &self.simulador
    }

    /// Solo lectura: los movimientos se registran a traves del control de trafico.
    pub fn historico(&self) -> &H {
        &self.historico
    }

    /// Fecha y hora de inicio mas el instante de simulacion.
    pub fn fecha_y_hora_actual(&self) -> FechaYHora {
        self.fecha_y_hora_de_inicio
            .sumar(self.simulador.instante_actual())
    }

    /// Plantas con llamada pendiente, por orden de llegada.
    pub fn llamadas_pendientes(&self) -> Vec<IdentificadorDePlanta> {
        self.llamadas_pendientes.iter().copied().collect()
    }

    /// Pulsar el boton de llamada de una planta. Por este orden: rechaza la planta si no existe;
    /// no tiene efecto si ya hay una llamada en curso para ella; si hay algun ascensor libre,
    /// envia el mas cercano y registra el movimiento; si no, la llamada queda pendiente.
    pub fn pulsar_boton_de_llamada(
        &mut self,
        planta: IdentificadorDePlanta,
    ) -> Result<ResultadoDeLaLlamada, ErrorDeControlDeTrafico> {
        if !self.simulador.configuracion().contiene_la_planta(planta) {
            return Err(ErrorDeSimulacion::PlantaInexistente.into());
        }
        if self.hay_una_llamada_en_curso_para(planta) {
            return Ok(ResultadoDeLaLlamada::LlamadaYaEnCurso);
        }
        match self.ascensor_libre_mas_cercano_a(planta) {
            Some(ascensor) => {
                self.dar_orden(ascensor, planta, MotivoDeMovimiento::Llamada)?;
                Ok(ResultadoDeLaLlamada::AscensorAsignado(ascensor))
            }
            None => {
                self.llamadas_pendientes.push_back(planta);
                Ok(ResultadoDeLaLlamada::PendienteDeAscensorLibre)
            }
        }
    }

    /// Pulsar, dentro de un ascensor, el boton de una planta.
    pub fn pulsar_boton_de_la_botonera(
        &mut self,
        identificador_de_ascensor: IdentificadorDeAscensor,
        planta_de_destino: IdentificadorDePlanta,
    ) -> Result<(), ErrorDeControlDeTrafico> {
        self.dar_orden(
            identificador_de_ascensor,
            planta_de_destino,
            MotivoDeMovimiento::Botonera,
        )
    }

    /// Avanza el tiempo del simulador, atiende las llamadas pendientes que se pueda y
    /// reposiciona los ascensores en reposo. Se detiene en el primer error de historico.
    pub fn avanzar_tiempo(&mut self, duracion: Duration) -> Result<(), ErrorDeControlDeTrafico> {
        self.simulador.avanzar_tiempo(duracion);
        self.atender_llamadas_pendientes()?;
        self.reposicionar_ascensores_en_reposo()
    }

    fn hay_una_llamada_en_curso_para(&self, planta: IdentificadorDePlanta) -> bool {
        let ya_esta_pendiente = self.llamadas_pendientes.contains(&planta);
        let se_dirige_un_ascensor_hacia_ella = self
            .simulador
            .posiciones_de_todos_los_ascensores()
            .iter()
            .any(|(_, posicion)| {
                posicion.estado
                    == EstadoDeAscensor::Desplazandose {
                        planta_de_destino: planta,
                    }
            });
        ya_esta_pendiente || se_dirige_un_ascensor_hacia_ella
    }

    fn ascensor_libre_mas_cercano_a(
        &self,
        planta: IdentificadorDePlanta,
    ) -> Option<IdentificadorDeAscensor> {
        ascensor_libre_mas_cercano(&self.simulador.posiciones_de_todos_los_ascensores(), planta)
    }

    /// Ordena al ascensor ir a la planta, anota desde cuando quedara libre y registra el
    /// movimiento. Si el historico falla, la orden ya dada al ascensor no se deshace.
    /// No se registra una orden a la planta en la que ya esta el ascensor, salvo que sea
    /// una llamada, porque esta representa demanda.
    fn dar_orden(
        &mut self,
        identificador_de_ascensor: IdentificadorDeAscensor,
        planta_de_destino: IdentificadorDePlanta,
        motivo: MotivoDeMovimiento,
    ) -> Result<(), ErrorDeControlDeTrafico> {
        let planta_de_origen = self
            .simulador
            .posicion_del_ascensor(identificador_de_ascensor)?
            .planta_actual;
        self.simulador
            .ordenar_mover_ascensor(identificador_de_ascensor, planta_de_destino)?;
        self.anotar_cuando_quedara_libre(
            identificador_de_ascensor,
            planta_de_origen.distancia_a(planta_de_destino),
        );
        let hay_que_registrarlo =
            planta_de_origen != planta_de_destino || motivo == MotivoDeMovimiento::Llamada;
        if hay_que_registrarlo {
            self.historico.registrar(MovimientoDeAscensor {
                fecha_y_hora: self.fecha_y_hora_actual(),
                identificador_de_ascensor,
                planta_de_origen,
                planta_de_destino,
                motivo,
            })?;
        }
        Ok(())
    }

    fn anotar_cuando_quedara_libre(
        &mut self,
        identificador_de_ascensor: IdentificadorDeAscensor,
        distancia_del_desplazamiento: u32,
    ) {
        let instante_actual = self.simulador.instante_actual();
        let duracion_del_desplazamiento = if distancia_del_desplazamiento == 0 {
            Duration::ZERO
        } else {
            self.simulador
                .configuracion()
                .duracion_de_un_desplazamiento(distancia_del_desplazamiento)
        };
        self.libre_desde_por_ascensor.insert(
            identificador_de_ascensor,
            instante_actual + duracion_del_desplazamiento,
        );
    }

    /// Por orden de llegada, mientras quede algun ascensor libre. Una llamada se da por
    /// atendida en cuanto se le asigna ascensor, aunque despues falle el registro.
    fn atender_llamadas_pendientes(&mut self) -> Result<(), ErrorDeControlDeTrafico> {
        while let Some(&planta) = self.llamadas_pendientes.front() {
            let Some(ascensor) = self.ascensor_libre_mas_cercano_a(planta) else {
                break;
            };
            self.llamadas_pendientes.pop_front();
            self.dar_orden(ascensor, planta, MotivoDeMovimiento::Llamada)?;
        }
        Ok(())
    }

    /// Lleva los ascensores en reposo a las plantas de espera preferentes que no cubre ya otro
    /// ascensor. Solo consulta el historico si hay algun ascensor en reposo.
    fn reposicionar_ascensores_en_reposo(&mut self) -> Result<(), ErrorDeControlDeTrafico> {
        let (ascensores_en_reposo, plantas_cubiertas_por_otros_ascensores) =
            self.ascensores_en_reposo_y_plantas_cubiertas_por_otros_ascensores();
        if ascensores_en_reposo.is_empty() {
            return Ok(());
        }
        let plan = plan_de_reposicionamiento(
            &self.plantas_de_espera_preferentes_ahora()?,
            &ascensores_en_reposo,
            &plantas_cubiertas_por_otros_ascensores,
        );
        for orden in plan {
            self.dar_orden(
                orden.identificador_de_ascensor,
                orden.planta_de_espera,
                MotivoDeMovimiento::Reposicionamiento,
            )?;
        }
        Ok(())
    }

    /// Separa los ascensores en reposo (identificador y planta actual) de las plantas que cubren
    /// los demas ascensores: la de destino de los que se desplazan y la actual de los que estan
    /// parados pero todavia no en reposo.
    fn ascensores_en_reposo_y_plantas_cubiertas_por_otros_ascensores(
        &self,
    ) -> (
        Vec<(IdentificadorDeAscensor, IdentificadorDePlanta)>,
        Vec<IdentificadorDePlanta>,
    ) {
        let mut ascensores_en_reposo = Vec::new();
        let mut plantas_cubiertas_por_otros_ascensores = Vec::new();
        for (identificador, posicion) in self.simulador.posiciones_de_todos_los_ascensores() {
            match posicion.estado {
                EstadoDeAscensor::Desplazandose { planta_de_destino } => {
                    plantas_cubiertas_por_otros_ascensores.push(planta_de_destino);
                }
                EstadoDeAscensor::Parado if self.esta_en_reposo(identificador) => {
                    ascensores_en_reposo.push((identificador, posicion.planta_actual));
                }
                EstadoDeAscensor::Parado => {
                    plantas_cubiertas_por_otros_ascensores.push(posicion.planta_actual);
                }
            }
        }
        (ascensores_en_reposo, plantas_cubiertas_por_otros_ascensores)
    }

    /// Plantas de espera preferentes para la fecha y hora actual, segun los movimientos de los
    /// ultimos `dias_de_historico_a_considerar` dias.
    fn plantas_de_espera_preferentes_ahora(
        &self,
    ) -> Result<Vec<IdentificadorDePlanta>, ErrorDeControlDeTrafico> {
        let fecha_y_hora_actual = self.fecha_y_hora_actual();
        let inicio_del_periodo_considerado =
            fecha_y_hora_actual.restar_dias(self.configuracion.dias_de_historico_a_considerar);
        let movimientos = self
            .historico
            .movimientos_desde(inicio_del_periodo_considerado)?;
        Ok(plantas_de_espera_preferentes(
            &movimientos,
            fecha_y_hora_actual,
            self.simulador.configuracion(),
        ))
    }

    /// Libre desde hace al menos el tiempo de reposo.
    fn esta_en_reposo(&self, identificador_de_ascensor: IdentificadorDeAscensor) -> bool {
        let instante_actual = self.simulador.instante_actual();
        self.libre_desde_por_ascensor
            .get(&identificador_de_ascensor)
            .is_some_and(|libre_desde| {
                instante_actual
                    >= *libre_desde + self.configuracion.tiempo_de_reposo_antes_de_reposicionar
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::adaptadores::historico_de_movimientos_en_memoria::HistoricoDeMovimientosEnMemoria;
    use crate::dominio::ascensor::PosicionDeAscensor;
    use crate::dominio::configuracion::ConfiguracionDelEdificio;

    const ASCENSOR_1: IdentificadorDeAscensor = IdentificadorDeAscensor(1);
    const ASCENSOR_2: IdentificadorDeAscensor = IdentificadorDeAscensor(2);
    const ASCENSOR_3: IdentificadorDeAscensor = IdentificadorDeAscensor(3);

    type ControlDeTraficoDePrueba = ControlDeTrafico<HistoricoDeMovimientosEnMemoria>;

    fn planta(numero: i32) -> IdentificadorDePlanta {
        IdentificadorDePlanta(numero)
    }

    fn segundos(numero: u64) -> Duration {
        Duration::from_secs(numero)
    }

    fn fecha_y_hora(
        anio: i32,
        mes: u32,
        dia: u32,
        hora: u32,
        minuto: u32,
        segundo: u32,
    ) -> FechaYHora {
        FechaYHora::nueva(anio, mes, dia, hora, minuto, segundo).unwrap()
    }

    /// Lunes 5 de octubre de 2026, 08:00:00.
    fn inicio_estandar() -> FechaYHora {
        fecha_y_hora(2026, 10, 5, 8, 0, 0)
    }

    fn control_con(
        configuracion_del_edificio: ConfiguracionDelEdificio,
        movimientos_previos: Vec<MovimientoDeAscensor>,
        fecha_y_hora_de_inicio: FechaYHora,
        configuracion_del_control: ConfiguracionDelControlDeTrafico,
    ) -> ControlDeTraficoDePrueba {
        ControlDeTrafico::nuevo(
            SimuladorDeEdificio::nuevo(configuracion_del_edificio).unwrap(),
            HistoricoDeMovimientosEnMemoria::con_movimientos(movimientos_previos),
            fecha_y_hora_de_inicio,
            configuracion_del_control,
        )
    }

    fn control_estandar() -> ControlDeTraficoDePrueba {
        control_con(
            ConfiguracionDelEdificio::estandar(),
            vec![],
            inicio_estandar(),
            ConfiguracionDelControlDeTrafico::estandar(),
        )
    }

    fn control_estandar_con_historico(
        movimientos_previos: Vec<MovimientoDeAscensor>,
    ) -> ControlDeTraficoDePrueba {
        control_con(
            ConfiguracionDelEdificio::estandar(),
            movimientos_previos,
            inicio_estandar(),
            ConfiguracionDelControlDeTrafico::estandar(),
        )
    }

    fn edificio_de_un_ascensor() -> ConfiguracionDelEdificio {
        ConfiguracionDelEdificio {
            numero_de_ascensores: 1,
            ..ConfiguracionDelEdificio::estandar()
        }
    }

    fn edificio_de_dos_ascensores() -> ConfiguracionDelEdificio {
        ConfiguracionDelEdificio {
            numero_de_ascensores: 2,
            ..ConfiguracionDelEdificio::estandar()
        }
    }

    fn control_sin_tiempo_de_reposo(
        configuracion_del_edificio: ConfiguracionDelEdificio,
        movimientos_previos: Vec<MovimientoDeAscensor>,
    ) -> ControlDeTraficoDePrueba {
        control_con(
            configuracion_del_edificio,
            movimientos_previos,
            inicio_estandar(),
            ConfiguracionDelControlDeTrafico {
                tiempo_de_reposo_antes_de_reposicionar: Duration::ZERO,
                ..ConfiguracionDelControlDeTrafico::estandar()
            },
        )
    }

    /// Llamada hecha en una fecha y hora anterior (el historico previo de los tests).
    fn llamada_previa(fecha_y_hora: FechaYHora, planta_de_destino: i32) -> MovimientoDeAscensor {
        MovimientoDeAscensor {
            fecha_y_hora,
            identificador_de_ascensor: ASCENSOR_1,
            planta_de_origen: planta(0),
            planta_de_destino: planta(planta_de_destino),
            motivo: MotivoDeMovimiento::Llamada,
        }
    }

    /// Llamada del lunes 28 de septiembre de 2026 (una semana antes del inicio) a las 08:30.
    fn llamada_del_lunes_anterior(planta_de_destino: i32) -> MovimientoDeAscensor {
        llamada_previa(fecha_y_hora(2026, 9, 28, 8, 30, 0), planta_de_destino)
    }

    fn movimiento(
        fecha_y_hora: FechaYHora,
        identificador_de_ascensor: IdentificadorDeAscensor,
        planta_de_origen: i32,
        planta_de_destino: i32,
        motivo: MotivoDeMovimiento,
    ) -> MovimientoDeAscensor {
        MovimientoDeAscensor {
            fecha_y_hora,
            identificador_de_ascensor,
            planta_de_origen: planta(planta_de_origen),
            planta_de_destino: planta(planta_de_destino),
            motivo,
        }
    }

    fn posicion<H: HistoricoDeMovimientos>(
        control: &ControlDeTrafico<H>,
        ascensor: IdentificadorDeAscensor,
    ) -> PosicionDeAscensor {
        control.simulador().posicion_del_ascensor(ascensor).unwrap()
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

    fn mandar_por_botonera(
        control: &mut ControlDeTraficoDePrueba,
        ascensor: IdentificadorDeAscensor,
        numero_de_planta: i32,
    ) {
        control
            .pulsar_boton_de_la_botonera(ascensor, planta(numero_de_planta))
            .unwrap();
    }

    /// En el instante 0 manda el ascensor 1 a la 1 (libre a los 6 s), el 2 a la 5 (a los 14 s)
    /// y el 3 a la 7 (a los 18 s), con lo que ninguno esta libre.
    fn ocupar_los_tres_ascensores(control: &mut ControlDeTraficoDePrueba) {
        mandar_por_botonera(control, ASCENSOR_1, 1);
        mandar_por_botonera(control, ASCENSOR_2, 5);
        mandar_por_botonera(control, ASCENSOR_3, 7);
    }

    fn avanzar(control: &mut ControlDeTraficoDePrueba, numero_de_segundos: u64) {
        control
            .avanzar_tiempo(segundos(numero_de_segundos))
            .unwrap();
    }

    fn movimientos_registrados(control: &ControlDeTraficoDePrueba) -> Vec<MovimientoDeAscensor> {
        control.historico().todos_los_movimientos().to_vec()
    }

    // Creacion y tiempo

    #[test]
    fn configuracion_estandar_del_control_de_trafico_considera_28_dias_de_historico_y_30_segundos_de_reposo()
     {
        let configuracion = ConfiguracionDelControlDeTrafico::estandar();
        assert_eq!(configuracion.dias_de_historico_a_considerar, 28);
        assert_eq!(
            configuracion.tiempo_de_reposo_antes_de_reposicionar,
            segundos(30)
        );
    }

    #[test]
    fn al_crear_el_control_de_trafico_no_hay_llamadas_pendientes_y_la_fecha_y_hora_actual_es_la_de_inicio()
     {
        let control = control_estandar();
        assert_eq!(control.llamadas_pendientes(), vec![]);
        assert_eq!(control.fecha_y_hora_actual(), inicio_estandar());
    }

    #[test]
    fn avanzar_tiempo_en_el_control_avanza_el_instante_del_simulador_y_la_fecha_y_hora_actual() {
        let mut control = control_estandar();
        avanzar(&mut control, 90);
        assert_eq!(control.simulador().instante_actual(), segundos(90));
        assert_eq!(
            control.fecha_y_hora_actual(),
            fecha_y_hora(2026, 10, 5, 8, 1, 30)
        );
    }

    // Llamadas

    #[test]
    fn pulsar_el_boton_de_llamada_de_una_planta_inexistente_da_error_de_planta_inexistente() {
        let mut control = control_estandar();
        assert_eq!(
            control.pulsar_boton_de_llamada(planta(8)),
            Err(ErrorDeControlDeTrafico::Simulacion(
                ErrorDeSimulacion::PlantaInexistente
            ))
        );
        assert_eq!(control.llamadas_pendientes(), vec![]);
    }

    #[test]
    fn una_llamada_envia_el_ascensor_libre_mas_cercano_a_la_planta_de_la_llamada() {
        // El mas cercano es el 2, y no el de menor identificador.
        let mut control = control_estandar();
        mandar_por_botonera(&mut control, ASCENSOR_2, 3);
        avanzar(&mut control, 10);
        assert_eq!(
            control.pulsar_boton_de_llamada(planta(4)),
            Ok(ResultadoDeLaLlamada::AscensorAsignado(ASCENSOR_2))
        );
        assert_eq!(posicion(&control, ASCENSOR_2), desplazandose_en(3, 4));
        assert_eq!(posicion(&control, ASCENSOR_1), parado_en(0));
        assert_eq!(posicion(&control, ASCENSOR_3), parado_en(0));
    }

    #[test]
    fn una_llamada_desde_una_planta_con_un_ascensor_parado_en_ella_se_atiende_con_ese_ascensor_sin_moverlo()
     {
        // El parado en la planta es el 2, y no el de menor identificador.
        let mut control = control_estandar();
        mandar_por_botonera(&mut control, ASCENSOR_2, 3);
        avanzar(&mut control, 10);
        assert_eq!(
            control.pulsar_boton_de_llamada(planta(3)),
            Ok(ResultadoDeLaLlamada::AscensorAsignado(ASCENSOR_2))
        );
        assert_eq!(posicion(&control, ASCENSOR_2), parado_en(3));
        assert_eq!(posicion(&control, ASCENSOR_1), parado_en(0));
        assert_eq!(posicion(&control, ASCENSOR_3), parado_en(0));
    }

    #[test]
    fn una_llamada_no_envia_ascensores_que_se_estan_desplazando() {
        let mut control = control_estandar();
        mandar_por_botonera(&mut control, ASCENSOR_1, 4);
        assert_eq!(
            control.pulsar_boton_de_llamada(planta(5)),
            Ok(ResultadoDeLaLlamada::AscensorAsignado(ASCENSOR_2))
        );
        assert_eq!(posicion(&control, ASCENSOR_1), desplazandose_en(0, 4));
        assert_eq!(posicion(&control, ASCENSOR_2), desplazandose_en(0, 5));
    }

    #[test]
    fn una_llamada_sin_ascensores_libres_queda_pendiente() {
        let mut control = control_estandar();
        ocupar_los_tres_ascensores(&mut control);
        assert_eq!(
            control.pulsar_boton_de_llamada(planta(2)),
            Ok(ResultadoDeLaLlamada::PendienteDeAscensorLibre)
        );
        assert_eq!(control.llamadas_pendientes(), vec![planta(2)]);
    }

    #[test]
    fn una_llamada_pendiente_se_atiende_al_avanzar_el_tiempo_en_cuanto_queda_libre_un_ascensor() {
        let mut control = control_estandar();
        ocupar_los_tres_ascensores(&mut control);
        control.pulsar_boton_de_llamada(planta(2)).unwrap();
        avanzar(&mut control, 5);
        assert_eq!(control.llamadas_pendientes(), vec![planta(2)]);
        avanzar(&mut control, 1);
        assert_eq!(control.llamadas_pendientes(), vec![]);
        assert_eq!(posicion(&control, ASCENSOR_1), desplazandose_en(1, 2));
    }

    #[test]
    fn las_llamadas_pendientes_se_atienden_por_orden_de_llegada() {
        let mut control = control_estandar();
        mandar_por_botonera(&mut control, ASCENSOR_1, 1);
        mandar_por_botonera(&mut control, ASCENSOR_2, 2);
        mandar_por_botonera(&mut control, ASCENSOR_3, 3);
        control.pulsar_boton_de_llamada(planta(5)).unwrap();
        control.pulsar_boton_de_llamada(planta(-1)).unwrap();
        assert_eq!(control.llamadas_pendientes(), vec![planta(5), planta(-1)]);
        avanzar(&mut control, 6);
        assert_eq!(control.llamadas_pendientes(), vec![planta(-1)]);
        assert_eq!(posicion(&control, ASCENSOR_1), desplazandose_en(1, 5));
        avanzar(&mut control, 2);
        assert_eq!(control.llamadas_pendientes(), vec![]);
        assert_eq!(posicion(&control, ASCENSOR_2), desplazandose_en(2, -1));
    }

    #[test]
    fn pulsar_de_nuevo_el_boton_de_una_planta_con_llamada_pendiente_no_crea_otra_llamada() {
        let mut control = control_estandar();
        ocupar_los_tres_ascensores(&mut control);
        control.pulsar_boton_de_llamada(planta(2)).unwrap();
        assert_eq!(
            control.pulsar_boton_de_llamada(planta(2)),
            Ok(ResultadoDeLaLlamada::LlamadaYaEnCurso)
        );
        assert_eq!(control.llamadas_pendientes(), vec![planta(2)]);
    }

    #[test]
    fn pulsar_el_boton_de_una_planta_hacia_la_que_ya_se_desplaza_un_ascensor_no_envia_otro() {
        let mut control = control_estandar();
        control.pulsar_boton_de_llamada(planta(5)).unwrap();
        assert_eq!(
            control.pulsar_boton_de_llamada(planta(5)),
            Ok(ResultadoDeLaLlamada::LlamadaYaEnCurso)
        );
        assert_eq!(posicion(&control, ASCENSOR_1), desplazandose_en(0, 5));
        assert_eq!(posicion(&control, ASCENSOR_2), parado_en(0));
        assert_eq!(posicion(&control, ASCENSOR_3), parado_en(0));
        assert_eq!(control.llamadas_pendientes(), vec![]);
    }

    // Botonera

    #[test]
    fn pulsar_un_boton_de_la_botonera_envia_ese_ascensor_a_la_planta_indicada() {
        let mut control = control_estandar();
        assert_eq!(
            control.pulsar_boton_de_la_botonera(ASCENSOR_2, planta(3)),
            Ok(())
        );
        assert_eq!(posicion(&control, ASCENSOR_2), desplazandose_en(0, 3));
        assert_eq!(posicion(&control, ASCENSOR_1), parado_en(0));
    }

    #[test]
    fn pulsar_un_boton_de_la_botonera_de_un_ascensor_inexistente_da_error_de_ascensor_inexistente()
    {
        let mut control = control_estandar();
        assert_eq!(
            control.pulsar_boton_de_la_botonera(IdentificadorDeAscensor(4), planta(3)),
            Err(ErrorDeControlDeTrafico::Simulacion(
                ErrorDeSimulacion::AscensorInexistente
            ))
        );
    }

    #[test]
    fn pulsar_un_boton_de_la_botonera_hacia_una_planta_inexistente_da_error_de_planta_inexistente()
    {
        let mut control = control_estandar();
        assert_eq!(
            control.pulsar_boton_de_la_botonera(ASCENSOR_1, planta(-3)),
            Err(ErrorDeControlDeTrafico::Simulacion(
                ErrorDeSimulacion::PlantaInexistente
            ))
        );
    }

    #[test]
    fn pulsar_un_boton_de_la_botonera_de_un_ascensor_que_se_desplaza_da_error_de_ascensor_ocupado()
    {
        let mut control = control_estandar();
        mandar_por_botonera(&mut control, ASCENSOR_1, 5);
        assert_eq!(
            control.pulsar_boton_de_la_botonera(ASCENSOR_1, planta(2)),
            Err(ErrorDeControlDeTrafico::Simulacion(
                ErrorDeSimulacion::AscensorOcupado
            ))
        );
        assert_eq!(posicion(&control, ASCENSOR_1), desplazandose_en(0, 5));
    }

    // Registro en el historico

    #[test]
    fn atender_una_llamada_registra_un_movimiento_por_llamada_con_su_fecha_y_hora_ascensor_origen_y_destino()
     {
        let mut control = control_estandar();
        control.pulsar_boton_de_llamada(planta(4)).unwrap();
        assert_eq!(
            movimientos_registrados(&control),
            vec![movimiento(
                inicio_estandar(),
                ASCENSOR_1,
                0,
                4,
                MotivoDeMovimiento::Llamada
            )]
        );
    }

    #[test]
    fn atender_una_llamada_con_un_ascensor_ya_en_la_planta_registra_un_movimiento_con_origen_igual_a_destino()
     {
        let mut control = control_estandar();
        control.pulsar_boton_de_llamada(planta(0)).unwrap();
        assert_eq!(
            movimientos_registrados(&control),
            vec![movimiento(
                inicio_estandar(),
                ASCENSOR_1,
                0,
                0,
                MotivoDeMovimiento::Llamada
            )]
        );
    }

    #[test]
    fn una_llamada_pendiente_se_registra_al_atenderse_con_la_fecha_y_hora_en_que_se_atiende() {
        let mut control = control_estandar();
        ocupar_los_tres_ascensores(&mut control);
        control.pulsar_boton_de_llamada(planta(2)).unwrap();
        avanzar(&mut control, 5);
        assert_eq!(movimientos_registrados(&control).len(), 3);
        avanzar(&mut control, 1);
        assert_eq!(
            movimientos_registrados(&control).last(),
            Some(&movimiento(
                fecha_y_hora(2026, 10, 5, 8, 0, 6),
                ASCENSOR_1,
                1,
                2,
                MotivoDeMovimiento::Llamada
            ))
        );
    }

    #[test]
    fn una_llamada_repetida_que_no_envia_ascensor_no_registra_movimiento() {
        let mut control = control_estandar();
        control.pulsar_boton_de_llamada(planta(5)).unwrap();
        control.pulsar_boton_de_llamada(planta(5)).unwrap();
        assert_eq!(movimientos_registrados(&control).len(), 1);
    }

    #[test]
    fn una_orden_de_la_botonera_registra_un_movimiento_por_botonera() {
        let mut control = control_estandar();
        mandar_por_botonera(&mut control, ASCENSOR_2, 3);
        assert_eq!(
            movimientos_registrados(&control),
            vec![movimiento(
                inicio_estandar(),
                ASCENSOR_2,
                0,
                3,
                MotivoDeMovimiento::Botonera
            )]
        );
    }

    #[test]
    fn una_orden_de_la_botonera_a_la_planta_en_la_que_esta_parado_el_ascensor_no_registra_movimiento()
     {
        let mut control = control_estandar();
        mandar_por_botonera(&mut control, ASCENSOR_1, 0);
        assert_eq!(movimientos_registrados(&control), vec![]);
    }

    #[test]
    fn una_orden_rechazada_no_registra_movimiento() {
        let mut control = control_estandar();
        mandar_por_botonera(&mut control, ASCENSOR_1, 5);
        let registrados_antes = movimientos_registrados(&control);
        assert!(control.pulsar_boton_de_llamada(planta(9)).is_err());
        assert!(
            control
                .pulsar_boton_de_la_botonera(IdentificadorDeAscensor(7), planta(1))
                .is_err()
        );
        assert!(
            control
                .pulsar_boton_de_la_botonera(ASCENSOR_2, planta(9))
                .is_err()
        );
        assert!(
            control
                .pulsar_boton_de_la_botonera(ASCENSOR_1, planta(1))
                .is_err()
        );
        assert_eq!(movimientos_registrados(&control), registrados_antes);
    }

    struct HistoricoQueFallaAlRegistrar;

    impl HistoricoDeMovimientos for HistoricoQueFallaAlRegistrar {
        fn registrar(
            &mut self,
            _movimiento: MovimientoDeAscensor,
        ) -> Result<(), ErrorDeHistoricoDeMovimientos> {
            Err(ErrorDeHistoricoDeMovimientos::AccesoAlAlmacenamiento {
                detalle: "fallo simulado".to_string(),
            })
        }

        fn movimientos_desde(
            &self,
            _fecha_y_hora: FechaYHora,
        ) -> Result<Vec<MovimientoDeAscensor>, ErrorDeHistoricoDeMovimientos> {
            Ok(vec![])
        }
    }

    #[test]
    fn si_el_historico_falla_al_registrar_la_operacion_devuelve_error_de_historico() {
        let mut control = ControlDeTrafico::nuevo(
            SimuladorDeEdificio::nuevo(ConfiguracionDelEdificio::estandar()).unwrap(),
            HistoricoQueFallaAlRegistrar,
            inicio_estandar(),
            ConfiguracionDelControlDeTrafico::estandar(),
        );
        let error_esperado = ErrorDeControlDeTrafico::Historico(
            ErrorDeHistoricoDeMovimientos::AccesoAlAlmacenamiento {
                detalle: "fallo simulado".to_string(),
            },
        );
        assert_eq!(
            control.pulsar_boton_de_llamada(planta(5)),
            Err(error_esperado.clone())
        );
        // La orden ya dada al ascensor no se deshace.
        assert_eq!(posicion(&control, ASCENSOR_1), desplazandose_en(0, 5));
        assert_eq!(
            control.pulsar_boton_de_la_botonera(ASCENSOR_2, planta(3)),
            Err(error_esperado)
        );
    }

    // Reposicionamiento

    #[test]
    fn sin_historico_los_ascensores_libres_no_se_reposicionan() {
        let mut control = control_estandar();
        avanzar(&mut control, 60);
        for ascensor in [ASCENSOR_1, ASCENSOR_2, ASCENSOR_3] {
            assert_eq!(posicion(&control, ascensor), parado_en(0));
        }
        assert_eq!(movimientos_registrados(&control), vec![]);
    }

    #[test]
    fn los_ascensores_en_reposo_se_reposicionan_a_las_plantas_con_mas_llamadas_del_mismo_dia_de_la_semana_y_franja_horaria()
     {
        let lunes_anterior = |hora, minuto| fecha_y_hora(2026, 9, 28, hora, minuto, 0);
        let martes_anterior = fecha_y_hora(2026, 9, 29, 8, 30, 0);
        let mut control = control_estandar_con_historico(vec![
            llamada_previa(lunes_anterior(8, 10), 5),
            llamada_previa(lunes_anterior(8, 20), 5),
            llamada_previa(lunes_anterior(8, 30), 2),
            llamada_previa(lunes_anterior(9, 30), 3),
            llamada_previa(lunes_anterior(9, 30), 3),
            llamada_previa(lunes_anterior(9, 30), 3),
            llamada_previa(martes_anterior, 6),
            llamada_previa(martes_anterior, 6),
            llamada_previa(martes_anterior, 6),
        ]);
        avanzar(&mut control, 30);
        assert_eq!(posicion(&control, ASCENSOR_1), desplazandose_en(0, 5));
        assert_eq!(posicion(&control, ASCENSOR_2), desplazandose_en(0, 2));
        assert_eq!(posicion(&control, ASCENSOR_3), parado_en(0));
    }

    #[test]
    fn un_ascensor_libre_no_se_reposiciona_hasta_cumplir_el_tiempo_de_reposo() {
        let mut control = control_con(
            edificio_de_un_ascensor(),
            vec![llamada_del_lunes_anterior(5)],
            inicio_estandar(),
            ConfiguracionDelControlDeTrafico::estandar(),
        );
        avanzar(&mut control, 29);
        assert_eq!(posicion(&control, ASCENSOR_1), parado_en(0));
        avanzar(&mut control, 1);
        assert_eq!(posicion(&control, ASCENSOR_1), desplazandose_en(0, 5));
    }

    #[test]
    fn el_tiempo_de_reposo_se_cuenta_desde_el_final_del_ultimo_desplazamiento_ordenado() {
        let mut control = control_con(
            edificio_de_un_ascensor(),
            vec![llamada_del_lunes_anterior(5)],
            inicio_estandar(),
            ConfiguracionDelControlDeTrafico::estandar(),
        );
        mandar_por_botonera(&mut control, ASCENSOR_1, 2);
        avanzar(&mut control, 37);
        assert_eq!(posicion(&control, ASCENSOR_1), parado_en(2));
        avanzar(&mut control, 1);
        assert_eq!(posicion(&control, ASCENSOR_1), desplazandose_en(2, 5));
    }

    #[test]
    fn las_llamadas_de_hace_mas_de_28_dias_no_influyen_en_el_reposicionamiento() {
        let hace_35_dias = fecha_y_hora(2026, 8, 31, 8, 30, 0);
        let hace_28_dias = fecha_y_hora(2026, 9, 7, 8, 30, 0);
        let mut control = control_con(
            edificio_de_un_ascensor(),
            vec![
                llamada_previa(hace_35_dias, 5),
                llamada_previa(hace_35_dias, 5),
                llamada_previa(hace_28_dias, 3),
            ],
            inicio_estandar(),
            ConfiguracionDelControlDeTrafico::estandar(),
        );
        avanzar(&mut control, 30);
        assert_eq!(posicion(&control, ASCENSOR_1), desplazandose_en(0, 3));
    }

    #[test]
    fn al_cambiar_de_franja_horaria_los_ascensores_en_reposo_se_reposicionan_segun_la_nueva_franja()
    {
        let mut control = control_con(
            edificio_de_un_ascensor(),
            vec![
                llamada_previa(fecha_y_hora(2026, 9, 28, 8, 30, 0), 5),
                llamada_previa(fecha_y_hora(2026, 9, 28, 9, 30, 0), -1),
            ],
            fecha_y_hora(2026, 10, 5, 8, 59, 0),
            ConfiguracionDelControlDeTrafico::estandar(),
        );
        avanzar(&mut control, 30);
        assert_eq!(posicion(&control, ASCENSOR_1), desplazandose_en(0, 5));
        avanzar(&mut control, 45);
        assert_eq!(
            control.fecha_y_hora_actual(),
            fecha_y_hora(2026, 10, 5, 9, 0, 15)
        );
        assert_eq!(posicion(&control, ASCENSOR_1), desplazandose_en(5, -1));
    }

    #[test]
    fn un_reposicionamiento_registra_un_movimiento_por_reposicionamiento() {
        let mut control = control_con(
            edificio_de_un_ascensor(),
            vec![llamada_del_lunes_anterior(5)],
            inicio_estandar(),
            ConfiguracionDelControlDeTrafico::estandar(),
        );
        avanzar(&mut control, 30);
        assert_eq!(
            movimientos_registrados(&control).last(),
            Some(&movimiento(
                fecha_y_hora(2026, 10, 5, 8, 0, 30),
                ASCENSOR_1,
                0,
                5,
                MotivoDeMovimiento::Reposicionamiento
            ))
        );
        assert_eq!(movimientos_registrados(&control).len(), 2);
    }

    #[test]
    fn las_llamadas_pendientes_se_atienden_antes_de_reposicionar_ascensores() {
        let mut control = control_sin_tiempo_de_reposo(
            edificio_de_un_ascensor(),
            vec![llamada_del_lunes_anterior(6)],
        );
        mandar_por_botonera(&mut control, ASCENSOR_1, 2);
        assert_eq!(
            control.pulsar_boton_de_llamada(planta(3)),
            Ok(ResultadoDeLaLlamada::PendienteDeAscensorLibre)
        );
        avanzar(&mut control, 10);
        assert_eq!(control.llamadas_pendientes(), vec![]);
        assert_eq!(posicion(&control, ASCENSOR_1), desplazandose_en(2, 3));
        let motivos: Vec<MotivoDeMovimiento> = movimientos_registrados(&control)
            .iter()
            .map(|movimiento| movimiento.motivo)
            .collect();
        assert!(!motivos.contains(&MotivoDeMovimiento::Reposicionamiento));
    }

    #[test]
    fn un_ascensor_en_reposo_no_se_reposiciona_hacia_una_planta_a_la_que_ya_se_dirige_otro_ascensor()
     {
        let mut control = control_sin_tiempo_de_reposo(
            edificio_de_dos_ascensores(),
            vec![llamada_del_lunes_anterior(5)],
        );
        mandar_por_botonera(&mut control, ASCENSOR_1, 5);
        avanzar(&mut control, 1);
        assert_eq!(posicion(&control, ASCENSOR_1), desplazandose_en(0, 5));
        assert_eq!(posicion(&control, ASCENSOR_2), parado_en(0));
    }

    // Mensajes de error

    #[test]
    fn los_errores_del_control_de_trafico_se_muestran_con_el_mensaje_del_error_que_contienen() {
        let de_simulacion = ErrorDeControlDeTrafico::Simulacion(ErrorDeSimulacion::AscensorOcupado);
        assert_eq!(
            de_simulacion.to_string(),
            ErrorDeSimulacion::AscensorOcupado.to_string()
        );
        let error_del_historico =
            ErrorDeHistoricoDeMovimientos::LineaIlegible { numero_de_linea: 3 };
        let del_historico = ErrorDeControlDeTrafico::Historico(error_del_historico.clone());
        assert_eq!(del_historico.to_string(), error_del_historico.to_string());
    }

    #[test]
    fn los_errores_del_dominio_y_del_control_de_trafico_implementan_std_error_error() {
        fn es_un_error<E: std::error::Error>() {}
        es_un_error::<crate::dominio::errores::ErrorDeConfiguracion>();
        es_un_error::<ErrorDeSimulacion>();
        es_un_error::<ErrorDeHistoricoDeMovimientos>();
        es_un_error::<ErrorDeControlDeTrafico>();
    }
}
