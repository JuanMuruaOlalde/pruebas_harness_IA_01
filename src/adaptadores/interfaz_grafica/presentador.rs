//! Presentador: traduce el estado del control de trafico a un modelo de vista y las acciones
//! del usuario a ordenes al control. Es logica pura, sin egui.

use std::cmp::Ordering;
use std::time::Duration;

use super::modelo_de_vista::{
    AccionDelUsuario, AscensorEnLaVista, BotonDeLaBotonera, CeldaDelHueco, MarchaDelAscensor,
    MensajeParaElUsuario, ModeloDeVista, PlantaEnLaVista,
};
use crate::aplicacion::control_de_trafico::{ControlDeTrafico, ResultadoDeLaLlamada};
use crate::dominio::ascensor::{EstadoDeAscensor, IdentificadorDeAscensor, PosicionDeAscensor};
use crate::dominio::configuracion::ConfiguracionDelEdificio;
use crate::dominio::fecha_y_hora::{DiaDeLaSemana, FechaYHora};
use crate::dominio::historico_de_movimientos::HistoricoDeMovimientos;
use crate::dominio::planta::IdentificadorDePlanta;

/// Tope de lo que avanza la simulacion en cada fotograma, aunque haya pasado mas tiempo real.
pub const AVANCE_MAXIMO_DE_LA_SIMULACION_POR_FOTOGRAMA: Duration = Duration::from_secs(1);

pub struct PresentadorDelSimulador<H: HistoricoDeMovimientos> {
    control: ControlDeTrafico<H>,
    mensaje_para_el_usuario: Option<MensajeParaElUsuario>,
}

impl<H: HistoricoDeMovimientos> PresentadorDelSimulador<H> {
    pub fn nuevo(control: ControlDeTrafico<H>) -> Self {
        Self {
            control,
            mensaje_para_el_usuario: None,
        }
    }

    /// Solo lectura: las ordenes se dan a traves de `atender`.
    pub fn control(&self) -> &ControlDeTrafico<H> {
        &self.control
    }

    /// Calcula lo que hay que dibujar a partir del estado actual del control. No cambia nada.
    pub fn modelo_de_vista(&self) -> ModeloDeVista {
        let simulador = self.control.simulador();
        let plantas_de_la_mas_alta_a_la_mas_baja =
            plantas_del_edificio_de_la_mas_alta_a_la_mas_baja(simulador.configuracion());
        let ascensores: Vec<AscensorEnLaVista> = simulador
            .posiciones_de_todos_los_ascensores()
            .into_iter()
            .map(|(identificador_de_ascensor, posicion)| {
                ascensor_en_la_vista(
                    identificador_de_ascensor,
                    posicion,
                    &plantas_de_la_mas_alta_a_la_mas_baja,
                )
            })
            .collect();
        let llamadas_pendientes = self.control.llamadas_pendientes();
        let plantas = plantas_de_la_mas_alta_a_la_mas_baja
            .iter()
            .map(|&planta| planta_en_la_vista(planta, &llamadas_pendientes, &ascensores))
            .collect();
        ModeloDeVista {
            texto_de_la_fecha_y_hora: texto_de_la_fecha_y_hora(self.control.fecha_y_hora_actual()),
            plantas,
            ascensores,
            texto_de_las_llamadas_pendientes: texto_de_las_llamadas_pendientes(
                &llamadas_pendientes,
            ),
            mensaje_para_el_usuario: self.mensaje_para_el_usuario.clone(),
        }
    }

    /// Ejecuta la accion en el control y sustituye el mensaje para el usuario por su resultado.
    pub fn atender(&mut self, accion: AccionDelUsuario) {
        self.mensaje_para_el_usuario = Some(match accion {
            AccionDelUsuario::PulsarBotonDeLlamada { planta } => {
                self.pulsar_boton_de_llamada(planta)
            }
            AccionDelUsuario::PulsarBotonDeLaBotonera {
                identificador_de_ascensor,
                planta_de_destino,
            } => self.pulsar_boton_de_la_botonera(identificador_de_ascensor, planta_de_destino),
        });
    }

    /// Avanza la simulacion el tiempo real transcurrido, con el tope por fotograma.
    /// Solo cambia el mensaje para el usuario si el control da un error.
    pub fn pasar_el_tiempo(&mut self, tiempo_real_transcurrido: Duration) {
        let avance = tiempo_real_transcurrido.min(AVANCE_MAXIMO_DE_LA_SIMULACION_POR_FOTOGRAMA);
        if let Err(error) = self.control.avanzar_tiempo(avance) {
            self.mensaje_para_el_usuario = Some(MensajeParaElUsuario::Error(format!(
                "Al avanzar el tiempo: {error}"
            )));
        }
    }

    fn pulsar_boton_de_llamada(&mut self, planta: IdentificadorDePlanta) -> MensajeParaElUsuario {
        let numero_de_planta = planta.0;
        match self.control.pulsar_boton_de_llamada(planta) {
            Ok(ResultadoDeLaLlamada::AscensorAsignado(ascensor_asignado)) => {
                MensajeParaElUsuario::Informacion(format!(
                    "Llamada en la planta {numero_de_planta}: atendida por el ascensor {}",
                    ascensor_asignado.0
                ))
            }
            Ok(ResultadoDeLaLlamada::PendienteDeAscensorLibre) => {
                MensajeParaElUsuario::Informacion(format!(
                    "Llamada en la planta {numero_de_planta}: pendiente hasta que quede libre algún ascensor"
                ))
            }
            Ok(ResultadoDeLaLlamada::LlamadaYaEnCurso) => MensajeParaElUsuario::Informacion(
                format!("Llamada en la planta {numero_de_planta}: ya estaba en curso"),
            ),
            Err(error) => MensajeParaElUsuario::Error(format!(
                "Llamada en la planta {numero_de_planta}: {error}"
            )),
        }
    }

    fn pulsar_boton_de_la_botonera(
        &mut self,
        identificador_de_ascensor: IdentificadorDeAscensor,
        planta_de_destino: IdentificadorDePlanta,
    ) -> MensajeParaElUsuario {
        let numero_de_ascensor = identificador_de_ascensor.0;
        let numero_de_planta_de_destino = planta_de_destino.0;
        match self
            .control
            .pulsar_boton_de_la_botonera(identificador_de_ascensor, planta_de_destino)
        {
            // Una orden aceptada que deja el ascensor parado es a la planta en la que ya estaba.
            Ok(()) if self.el_ascensor_esta_parado(identificador_de_ascensor) => {
                MensajeParaElUsuario::Informacion(format!(
                    "Ascensor {numero_de_ascensor}: ya está en la planta {numero_de_planta_de_destino}"
                ))
            }
            Ok(()) => MensajeParaElUsuario::Informacion(format!(
                "Ascensor {numero_de_ascensor}: va a la planta {numero_de_planta_de_destino}"
            )),
            Err(error) => MensajeParaElUsuario::Error(format!(
                "Ascensor {numero_de_ascensor}, botón de la planta {numero_de_planta_de_destino}: {error}"
            )),
        }
    }

    fn el_ascensor_esta_parado(&self, identificador_de_ascensor: IdentificadorDeAscensor) -> bool {
        self.control
            .simulador()
            .posicion_del_ascensor(identificador_de_ascensor)
            .is_ok_and(|posicion| posicion.estado == EstadoDeAscensor::Parado)
    }
}

fn plantas_del_edificio_de_la_mas_alta_a_la_mas_baja(
    configuracion: &ConfiguracionDelEdificio,
) -> Vec<IdentificadorDePlanta> {
    (configuracion.planta_mas_baja.0..=configuracion.planta_mas_alta.0)
        .rev()
        .map(IdentificadorDePlanta)
        .collect()
}

fn ascensor_en_la_vista(
    identificador_de_ascensor: IdentificadorDeAscensor,
    posicion: PosicionDeAscensor,
    plantas_de_la_mas_alta_a_la_mas_baja: &[IdentificadorDePlanta],
) -> AscensorEnLaVista {
    let planta_actual = posicion.planta_actual;
    let (marcha, planta_de_destino) = marcha_y_planta_de_destino(posicion);
    // Solo un ascensor parado acepta ordenes; mientras se desplaza, se ve adonde va.
    let botonera = plantas_de_la_mas_alta_a_la_mas_baja
        .iter()
        .map(|&planta| BotonDeLaBotonera {
            planta,
            habilitado: marcha == MarchaDelAscensor::Parado,
            encendido: planta_de_destino == Some(planta),
        })
        .collect();
    AscensorEnLaVista {
        identificador_de_ascensor,
        planta_actual,
        marcha,
        planta_de_destino,
        descripcion_del_estado: descripcion_del_estado(planta_actual, marcha, planta_de_destino),
        botonera,
    }
}

/// Un ascensor parado no tiene destino. Si se desplaza, su marcha depende de donde esta el
/// destino respecto de la planta actual: esta llegando cuando ya esta en ella.
fn marcha_y_planta_de_destino(
    posicion: PosicionDeAscensor,
) -> (MarchaDelAscensor, Option<IdentificadorDePlanta>) {
    match posicion.estado {
        EstadoDeAscensor::Parado => (MarchaDelAscensor::Parado, None),
        EstadoDeAscensor::Desplazandose { planta_de_destino } => {
            let marcha = match planta_de_destino.cmp(&posicion.planta_actual) {
                Ordering::Greater => MarchaDelAscensor::Subiendo,
                Ordering::Less => MarchaDelAscensor::Bajando,
                Ordering::Equal => MarchaDelAscensor::Llegando,
            };
            (marcha, Some(planta_de_destino))
        }
    }
}

fn descripcion_del_estado(
    planta_actual: IdentificadorDePlanta,
    marcha: MarchaDelAscensor,
    planta_de_destino: Option<IdentificadorDePlanta>,
) -> String {
    let actual = planta_actual.0;
    // Solo el ascensor parado no tiene destino, y su descripcion no lo usa.
    let destino = planta_de_destino.map_or(actual, |planta| planta.0);
    match marcha {
        MarchaDelAscensor::Parado => format!("Parado en la planta {actual}"),
        MarchaDelAscensor::Subiendo => {
            format!("En la planta {actual}, subiendo hacia la {destino}")
        }
        MarchaDelAscensor::Bajando => format!("En la planta {actual}, bajando hacia la {destino}"),
        MarchaDelAscensor::Llegando => format!("Llegando a la planta {destino}"),
    }
}

/// Una fila de la rejilla: el boton de llamada de la planta y su celda en cada hueco.
fn planta_en_la_vista(
    planta: IdentificadorDePlanta,
    llamadas_pendientes: &[IdentificadorDePlanta],
    ascensores: &[AscensorEnLaVista],
) -> PlantaEnLaVista {
    PlantaEnLaVista {
        planta,
        boton_de_llamada_encendido: boton_de_llamada_encendido(
            planta,
            llamadas_pendientes,
            ascensores,
        ),
        celdas_de_los_huecos: ascensores
            .iter()
            .map(|ascensor| celda_del_hueco(ascensor, planta))
            .collect(),
    }
}

/// Encendido cuando pulsarlo no tendria efecto: la planta tiene una llamada pendiente o algun
/// ascensor se desplaza hacia ella, por el motivo que sea.
fn boton_de_llamada_encendido(
    planta: IdentificadorDePlanta,
    llamadas_pendientes: &[IdentificadorDePlanta],
    ascensores: &[AscensorEnLaVista],
) -> bool {
    llamadas_pendientes.contains(&planta)
        || ascensores
            .iter()
            .any(|ascensor| ascensor.planta_de_destino == Some(planta))
}

fn celda_del_hueco(ascensor: &AscensorEnLaVista, planta: IdentificadorDePlanta) -> CeldaDelHueco {
    if ascensor.planta_actual == planta {
        CeldaDelHueco::Ascensor(ascensor.marcha)
    } else if ascensor.planta_de_destino == Some(planta) {
        CeldaDelHueco::DestinoDelAscensor
    } else {
        CeldaDelHueco::Vacia
    }
}

fn texto_de_las_llamadas_pendientes(llamadas_pendientes: &[IdentificadorDePlanta]) -> String {
    if llamadas_pendientes.is_empty() {
        return "Llamadas pendientes: ninguna".to_string();
    }
    let plantas: Vec<String> = llamadas_pendientes
        .iter()
        .map(|planta| planta.0.to_string())
        .collect();
    format!("Llamadas pendientes: {}", plantas.join(", "))
}

fn nombre_del_dia_de_la_semana(dia: DiaDeLaSemana) -> &'static str {
    match dia {
        DiaDeLaSemana::Lunes => "Lunes",
        DiaDeLaSemana::Martes => "Martes",
        DiaDeLaSemana::Miercoles => "Miércoles",
        DiaDeLaSemana::Jueves => "Jueves",
        DiaDeLaSemana::Viernes => "Viernes",
        DiaDeLaSemana::Sabado => "Sábado",
        DiaDeLaSemana::Domingo => "Domingo",
    }
}

/// Por ejemplo, `Lunes 05/10/2026 08:00:10`: sin fraccion de segundo.
fn texto_de_la_fecha_y_hora(fecha_y_hora: FechaYHora) -> String {
    format!(
        "{} {:02}/{:02}/{:04} {:02}:{:02}:{:02}",
        nombre_del_dia_de_la_semana(fecha_y_hora.dia_de_la_semana()),
        fecha_y_hora.dia(),
        fecha_y_hora.mes(),
        fecha_y_hora.anio(),
        fecha_y_hora.hora_del_dia(),
        fecha_y_hora.minuto(),
        fecha_y_hora.segundo(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::adaptadores::historico_de_movimientos_en_memoria::HistoricoDeMovimientosEnMemoria;
    use crate::aplicacion::control_de_trafico::ConfiguracionDelControlDeTrafico;
    use crate::dominio::configuracion::ConfiguracionDelEdificio;
    use crate::dominio::historico_de_movimientos::ErrorDeHistoricoDeMovimientos;
    use crate::dominio::movimiento::{MotivoDeMovimiento, MovimientoDeAscensor};
    use crate::dominio::simulador::SimuladorDeEdificio;

    const ASCENSOR_1: IdentificadorDeAscensor = IdentificadorDeAscensor(1);
    const ASCENSOR_2: IdentificadorDeAscensor = IdentificadorDeAscensor(2);
    const ASCENSOR_3: IdentificadorDeAscensor = IdentificadorDeAscensor(3);
    const UN_SEGUNDO: Duration = Duration::from_secs(1);

    fn planta(numero: i32) -> IdentificadorDePlanta {
        IdentificadorDePlanta(numero)
    }

    fn lunes_5_de_octubre_de_2026_a_las_8() -> FechaYHora {
        FechaYHora::nueva(2026, 10, 5, 8, 0, 0).unwrap()
    }

    /// Doble de test: un historico que siempre falla al registrar.
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
            Ok(Vec::new())
        }
    }

    fn presentador_con_historico<H: HistoricoDeMovimientos>(
        configuracion_del_edificio: ConfiguracionDelEdificio,
        historico: H,
    ) -> PresentadorDelSimulador<H> {
        let simulador = SimuladorDeEdificio::nuevo(configuracion_del_edificio).unwrap();
        PresentadorDelSimulador::nuevo(ControlDeTrafico::nuevo(
            simulador,
            historico,
            lunes_5_de_octubre_de_2026_a_las_8(),
            ConfiguracionDelControlDeTrafico::estandar(),
        ))
    }

    fn presentador_estandar() -> PresentadorDelSimulador<HistoricoDeMovimientosEnMemoria> {
        presentador_con_historico(
            ConfiguracionDelEdificio::estandar(),
            HistoricoDeMovimientosEnMemoria::nuevo(),
        )
    }

    fn pasar_segundos<H: HistoricoDeMovimientos>(
        presentador: &mut PresentadorDelSimulador<H>,
        segundos: u32,
    ) {
        for _ in 0..segundos {
            presentador.pasar_el_tiempo(UN_SEGUNDO);
        }
    }

    fn pulsar_llamada<H: HistoricoDeMovimientos>(
        presentador: &mut PresentadorDelSimulador<H>,
        numero_de_planta: i32,
    ) {
        presentador.atender(AccionDelUsuario::PulsarBotonDeLlamada {
            planta: planta(numero_de_planta),
        });
    }

    fn pulsar_botonera<H: HistoricoDeMovimientos>(
        presentador: &mut PresentadorDelSimulador<H>,
        identificador_de_ascensor: IdentificadorDeAscensor,
        numero_de_planta_de_destino: i32,
    ) {
        presentador.atender(AccionDelUsuario::PulsarBotonDeLaBotonera {
            identificador_de_ascensor,
            planta_de_destino: planta(numero_de_planta_de_destino),
        });
    }

    fn ascensor_en_la_vista(
        modelo: &ModeloDeVista,
        identificador: IdentificadorDeAscensor,
    ) -> &AscensorEnLaVista {
        modelo
            .ascensores
            .iter()
            .find(|ascensor| ascensor.identificador_de_ascensor == identificador)
            .unwrap()
    }

    fn planta_en_la_vista(modelo: &ModeloDeVista, numero_de_planta: i32) -> &PlantaEnLaVista {
        modelo
            .plantas
            .iter()
            .find(|planta_en_la_vista| planta_en_la_vista.planta == planta(numero_de_planta))
            .unwrap()
    }

    /// La celda de la planta en el hueco del ascensor (el ascensor 1 es el hueco 0).
    fn celda_del_hueco(
        modelo: &ModeloDeVista,
        numero_de_planta: i32,
        identificador: IdentificadorDeAscensor,
    ) -> CeldaDelHueco {
        let indice_del_hueco = identificador.0 as usize - 1;
        planta_en_la_vista(modelo, numero_de_planta).celdas_de_los_huecos[indice_del_hueco]
    }

    fn mensaje(
        presentador: &PresentadorDelSimulador<impl HistoricoDeMovimientos>,
    ) -> MensajeParaElUsuario {
        presentador
            .modelo_de_vista()
            .mensaje_para_el_usuario
            .expect("deberia haber un mensaje para el usuario")
    }

    fn informacion(texto: &str) -> MensajeParaElUsuario {
        MensajeParaElUsuario::Informacion(texto.to_string())
    }

    fn error(texto: &str) -> MensajeParaElUsuario {
        MensajeParaElUsuario::Error(texto.to_string())
    }

    // ---- Estado inicial y estructura del modelo de vista ----

    #[test]
    fn al_crear_el_presentador_no_hay_mensaje_para_el_usuario() {
        let presentador = presentador_estandar();
        assert_eq!(presentador.modelo_de_vista().mensaje_para_el_usuario, None);
    }

    #[test]
    fn el_modelo_de_vista_tiene_una_planta_por_cada_planta_del_edificio_de_la_mas_alta_a_la_mas_baja()
     {
        let modelo = presentador_estandar().modelo_de_vista();
        let plantas: Vec<i32> = modelo.plantas.iter().map(|p| p.planta.0).collect();
        assert_eq!(plantas, vec![7, 6, 5, 4, 3, 2, 1, 0, -1, -2]);
    }

    #[test]
    fn cada_planta_del_modelo_de_vista_tiene_una_celda_de_hueco_por_ascensor() {
        let modelo = presentador_estandar().modelo_de_vista();
        for planta_en_la_vista in &modelo.plantas {
            assert_eq!(planta_en_la_vista.celdas_de_los_huecos.len(), 3);
        }
    }

    #[test]
    fn el_modelo_de_vista_tiene_un_ascensor_por_cada_ascensor_del_edificio_ordenados_por_identificador()
     {
        let modelo = presentador_estandar().modelo_de_vista();
        let identificadores: Vec<_> = modelo
            .ascensores
            .iter()
            .map(|a| a.identificador_de_ascensor)
            .collect();
        assert_eq!(identificadores, vec![ASCENSOR_1, ASCENSOR_2, ASCENSOR_3]);
    }

    #[test]
    fn la_botonera_de_cada_ascensor_tiene_un_boton_por_planta_de_la_mas_alta_a_la_mas_baja() {
        let modelo = presentador_estandar().modelo_de_vista();
        for ascensor in &modelo.ascensores {
            let plantas: Vec<i32> = ascensor.botonera.iter().map(|b| b.planta.0).collect();
            assert_eq!(plantas, vec![7, 6, 5, 4, 3, 2, 1, 0, -1, -2]);
        }
    }

    #[test]
    fn al_iniciar_todos_los_ascensores_aparecen_parados_en_la_planta_principal() {
        let modelo = presentador_estandar().modelo_de_vista();
        for ascensor in &modelo.ascensores {
            assert_eq!(ascensor.planta_actual, planta(0));
            assert_eq!(ascensor.marcha, MarchaDelAscensor::Parado);
            assert_eq!(ascensor.planta_de_destino, None);
            assert_eq!(
                celda_del_hueco(&modelo, 0, ascensor.identificador_de_ascensor),
                CeldaDelHueco::Ascensor(MarchaDelAscensor::Parado)
            );
        }
    }

    // ---- Posicion y marcha de los ascensores ----

    #[test]
    fn la_celda_del_hueco_de_la_planta_actual_de_un_ascensor_muestra_el_ascensor_con_su_marcha() {
        let mut presentador = presentador_estandar();
        pulsar_botonera(&mut presentador, ASCENSOR_1, 4);
        pasar_segundos(&mut presentador, 4);
        let modelo = presentador.modelo_de_vista();
        assert_eq!(
            celda_del_hueco(&modelo, 1, ASCENSOR_1),
            CeldaDelHueco::Ascensor(MarchaDelAscensor::Subiendo)
        );
        assert_eq!(
            celda_del_hueco(&modelo, 0, ASCENSOR_2),
            CeldaDelHueco::Ascensor(MarchaDelAscensor::Parado)
        );
    }

    #[test]
    fn la_celda_del_hueco_de_la_planta_de_destino_muestra_el_destino_mientras_el_ascensor_no_ha_llegado()
     {
        let mut presentador = presentador_estandar();
        pulsar_botonera(&mut presentador, ASCENSOR_1, 4);
        for segundos_transcurridos in [0, 4] {
            let modelo = presentador.modelo_de_vista();
            assert_eq!(
                celda_del_hueco(&modelo, 4, ASCENSOR_1),
                CeldaDelHueco::DestinoDelAscensor,
                "a los {segundos_transcurridos} s"
            );
            pasar_segundos(&mut presentador, 4);
        }
    }

    #[test]
    fn las_celdas_del_hueco_de_las_demas_plantas_estan_vacias() {
        let mut presentador = presentador_estandar();
        pulsar_botonera(&mut presentador, ASCENSOR_1, 4);
        pasar_segundos(&mut presentador, 4);
        let modelo = presentador.modelo_de_vista();
        for numero_de_planta in [7, 6, 5, 3, 2, 0, -1, -2] {
            assert_eq!(
                celda_del_hueco(&modelo, numero_de_planta, ASCENSOR_1),
                CeldaDelHueco::Vacia,
                "planta {numero_de_planta}"
            );
        }
    }

    #[test]
    fn un_ascensor_que_va_hacia_una_planta_mas_alta_esta_subiendo() {
        let mut presentador = presentador_estandar();
        pulsar_botonera(&mut presentador, ASCENSOR_1, 4);
        let modelo = presentador.modelo_de_vista();
        let ascensor = ascensor_en_la_vista(&modelo, ASCENSOR_1);
        assert_eq!(ascensor.marcha, MarchaDelAscensor::Subiendo);
        assert_eq!(ascensor.planta_de_destino, Some(planta(4)));
    }

    #[test]
    fn un_ascensor_que_va_hacia_una_planta_mas_baja_esta_bajando() {
        let mut presentador = presentador_estandar();
        pulsar_botonera(&mut presentador, ASCENSOR_2, -2);
        let modelo = presentador.modelo_de_vista();
        let ascensor = ascensor_en_la_vista(&modelo, ASCENSOR_2);
        assert_eq!(ascensor.marcha, MarchaDelAscensor::Bajando);
        assert_eq!(ascensor.planta_de_destino, Some(planta(-2)));
    }

    #[test]
    fn un_ascensor_en_la_parada_de_su_planta_de_destino_esta_llegando() {
        let mut presentador = presentador_estandar();
        pulsar_botonera(&mut presentador, ASCENSOR_1, 4);
        pasar_segundos(&mut presentador, 10);
        let modelo = presentador.modelo_de_vista();
        let ascensor = ascensor_en_la_vista(&modelo, ASCENSOR_1);
        assert_eq!(ascensor.planta_actual, planta(4));
        assert_eq!(ascensor.marcha, MarchaDelAscensor::Llegando);
        assert_eq!(ascensor.planta_de_destino, Some(planta(4)));
        assert_eq!(
            celda_del_hueco(&modelo, 4, ASCENSOR_1),
            CeldaDelHueco::Ascensor(MarchaDelAscensor::Llegando)
        );
    }

    #[test]
    fn un_ascensor_que_ha_terminado_su_desplazamiento_queda_parado_en_la_planta_de_destino() {
        let mut presentador = presentador_estandar();
        pulsar_botonera(&mut presentador, ASCENSOR_1, 4);
        pasar_segundos(&mut presentador, 12);
        let modelo = presentador.modelo_de_vista();
        let ascensor = ascensor_en_la_vista(&modelo, ASCENSOR_1);
        assert_eq!(ascensor.planta_actual, planta(4));
        assert_eq!(ascensor.marcha, MarchaDelAscensor::Parado);
        assert_eq!(ascensor.planta_de_destino, None);
        assert_eq!(
            celda_del_hueco(&modelo, 4, ASCENSOR_1),
            CeldaDelHueco::Ascensor(MarchaDelAscensor::Parado)
        );
    }

    #[test]
    fn la_descripcion_de_un_ascensor_parado_indica_su_planta() {
        let mut presentador = presentador_estandar();
        assert_eq!(
            ascensor_en_la_vista(&presentador.modelo_de_vista(), ASCENSOR_1).descripcion_del_estado,
            "Parado en la planta 0"
        );
        pulsar_botonera(&mut presentador, ASCENSOR_1, 4);
        pasar_segundos(&mut presentador, 12);
        assert_eq!(
            ascensor_en_la_vista(&presentador.modelo_de_vista(), ASCENSOR_1).descripcion_del_estado,
            "Parado en la planta 4"
        );
    }

    #[test]
    fn la_descripcion_de_un_ascensor_en_marcha_indica_su_planta_actual_su_sentido_y_su_destino() {
        let mut presentador = presentador_estandar();
        pulsar_botonera(&mut presentador, ASCENSOR_1, 4);
        pulsar_botonera(&mut presentador, ASCENSOR_2, -2);
        let descripcion = |presentador: &PresentadorDelSimulador<_>, ascensor| {
            ascensor_en_la_vista(&presentador.modelo_de_vista(), ascensor)
                .descripcion_del_estado
                .clone()
        };
        assert_eq!(
            descripcion(&presentador, ASCENSOR_1),
            "En la planta 0, subiendo hacia la 4"
        );
        assert_eq!(
            descripcion(&presentador, ASCENSOR_2),
            "En la planta 0, bajando hacia la -2"
        );
        pasar_segundos(&mut presentador, 4);
        assert_eq!(
            descripcion(&presentador, ASCENSOR_1),
            "En la planta 1, subiendo hacia la 4"
        );
        pasar_segundos(&mut presentador, 2);
        assert_eq!(
            descripcion(&presentador, ASCENSOR_2),
            "Llegando a la planta -2"
        );
    }

    // ---- Botones de llamada y botoneras ----

    #[test]
    fn el_boton_de_llamada_de_una_planta_sin_llamada_en_curso_esta_apagado() {
        let modelo = presentador_estandar().modelo_de_vista();
        for planta_en_la_vista in &modelo.plantas {
            assert!(!planta_en_la_vista.boton_de_llamada_encendido);
        }
    }

    #[test]
    fn el_boton_de_llamada_de_una_planta_con_llamada_pendiente_esta_encendido() {
        let mut presentador = presentador_estandar();
        pulsar_botonera(&mut presentador, ASCENSOR_1, 1);
        pulsar_botonera(&mut presentador, ASCENSOR_2, 5);
        pulsar_botonera(&mut presentador, ASCENSOR_3, 7);
        pulsar_llamada(&mut presentador, 2);
        let modelo = presentador.modelo_de_vista();
        assert!(planta_en_la_vista(&modelo, 2).boton_de_llamada_encendido);
        assert!(!planta_en_la_vista(&modelo, 3).boton_de_llamada_encendido);
    }

    #[test]
    fn el_boton_de_llamada_de_una_planta_hacia_la_que_se_desplaza_un_ascensor_esta_encendido() {
        let mut presentador = presentador_estandar();
        pulsar_botonera(&mut presentador, ASCENSOR_2, -2);
        let modelo = presentador.modelo_de_vista();
        assert!(planta_en_la_vista(&modelo, -2).boton_de_llamada_encendido);
    }

    #[test]
    fn el_boton_de_llamada_se_apaga_cuando_el_ascensor_queda_parado_en_la_planta() {
        let mut presentador = presentador_estandar();
        pulsar_llamada(&mut presentador, 4);
        for segundos_a_avanzar in [0, 4, 6] {
            pasar_segundos(&mut presentador, segundos_a_avanzar);
            assert!(
                planta_en_la_vista(&presentador.modelo_de_vista(), 4).boton_de_llamada_encendido
            );
        }
        pasar_segundos(&mut presentador, 2);
        assert!(!planta_en_la_vista(&presentador.modelo_de_vista(), 4).boton_de_llamada_encendido);
    }

    #[test]
    fn los_botones_de_la_botonera_de_un_ascensor_parado_estan_habilitados_y_apagados() {
        let modelo = presentador_estandar().modelo_de_vista();
        for ascensor in &modelo.ascensores {
            for boton in &ascensor.botonera {
                assert!(boton.habilitado, "planta {}", boton.planta.0);
                assert!(!boton.encendido, "planta {}", boton.planta.0);
            }
        }
    }

    #[test]
    fn los_botones_de_la_botonera_de_un_ascensor_que_se_desplaza_estan_deshabilitados() {
        let mut presentador = presentador_estandar();
        pulsar_botonera(&mut presentador, ASCENSOR_1, 4);
        // Subiendo a los 0 s y a los 4 s, y llegando a los 10 s: se desplaza hasta los 12 s.
        for segundos_a_avanzar in [0, 4, 6] {
            pasar_segundos(&mut presentador, segundos_a_avanzar);
            let instante = presentador.control().simulador().instante_actual();
            let modelo = presentador.modelo_de_vista();
            for boton in &ascensor_en_la_vista(&modelo, ASCENSOR_1).botonera {
                assert!(
                    !boton.habilitado,
                    "planta {} a los {instante:?}",
                    boton.planta.0
                );
            }
            for boton in &ascensor_en_la_vista(&modelo, ASCENSOR_2).botonera {
                assert!(
                    boton.habilitado,
                    "planta {} a los {instante:?}",
                    boton.planta.0
                );
            }
        }
    }

    #[test]
    fn el_boton_de_la_planta_de_destino_de_la_botonera_esta_encendido_mientras_el_ascensor_se_desplaza()
     {
        let mut presentador = presentador_estandar();
        pulsar_botonera(&mut presentador, ASCENSOR_1, 4);
        for segundos_a_avanzar in [0, 4, 6] {
            pasar_segundos(&mut presentador, segundos_a_avanzar);
            let modelo = presentador.modelo_de_vista();
            let encendidos: Vec<BotonDeLaBotonera> = ascensor_en_la_vista(&modelo, ASCENSOR_1)
                .botonera
                .iter()
                .copied()
                .filter(|boton| boton.encendido)
                .collect();
            assert_eq!(encendidos.len(), 1);
            assert_eq!(encendidos[0].planta, planta(4));
        }
        pasar_segundos(&mut presentador, 2);
        let modelo = presentador.modelo_de_vista();
        assert!(
            ascensor_en_la_vista(&modelo, ASCENSOR_1)
                .botonera
                .iter()
                .all(|boton| !boton.encendido)
        );
    }

    // ---- Textos de la fecha y hora y de las llamadas pendientes ----

    #[test]
    fn la_fecha_y_hora_se_muestra_con_el_dia_de_la_semana_y_sin_fraccion_de_segundo() {
        let mut presentador = presentador_estandar();
        assert_eq!(
            presentador.modelo_de_vista().texto_de_la_fecha_y_hora,
            "Lunes 05/10/2026 08:00:00"
        );
        pasar_segundos(&mut presentador, 10);
        presentador.pasar_el_tiempo(Duration::from_millis(500));
        assert_eq!(
            presentador.modelo_de_vista().texto_de_la_fecha_y_hora,
            "Lunes 05/10/2026 08:00:10"
        );
        assert_eq!(
            texto_de_la_fecha_y_hora(FechaYHora::nueva(2026, 1, 3, 9, 5, 7).unwrap()),
            "Sábado 03/01/2026 09:05:07"
        );
    }

    #[test]
    fn los_dias_de_la_semana_se_muestran_con_su_nombre_en_espanol() {
        let casos = [
            (DiaDeLaSemana::Lunes, "Lunes"),
            (DiaDeLaSemana::Martes, "Martes"),
            (DiaDeLaSemana::Miercoles, "Miércoles"),
            (DiaDeLaSemana::Jueves, "Jueves"),
            (DiaDeLaSemana::Viernes, "Viernes"),
            (DiaDeLaSemana::Sabado, "Sábado"),
            (DiaDeLaSemana::Domingo, "Domingo"),
        ];
        for (dia, nombre_esperado) in casos {
            assert_eq!(nombre_del_dia_de_la_semana(dia), nombre_esperado);
        }
    }

    #[test]
    fn sin_llamadas_pendientes_el_texto_de_las_llamadas_pendientes_lo_indica() {
        assert_eq!(
            presentador_estandar()
                .modelo_de_vista()
                .texto_de_las_llamadas_pendientes,
            "Llamadas pendientes: ninguna"
        );
    }

    #[test]
    fn las_llamadas_pendientes_se_muestran_por_orden_de_llegada() {
        let mut presentador = presentador_estandar();
        pulsar_botonera(&mut presentador, ASCENSOR_1, 1);
        pulsar_botonera(&mut presentador, ASCENSOR_2, 5);
        pulsar_botonera(&mut presentador, ASCENSOR_3, 7);
        pulsar_llamada(&mut presentador, 2);
        pulsar_llamada(&mut presentador, -1);
        assert_eq!(
            presentador
                .modelo_de_vista()
                .texto_de_las_llamadas_pendientes,
            "Llamadas pendientes: 2, -1"
        );
        pasar_segundos(&mut presentador, 6);
        assert_eq!(
            presentador
                .modelo_de_vista()
                .texto_de_las_llamadas_pendientes,
            "Llamadas pendientes: -1"
        );
    }

    // ---- Boton de llamada ----

    #[test]
    fn pulsar_un_boton_de_llamada_envia_el_ascensor_libre_mas_cercano_a_esa_planta() {
        let mut presentador = presentador_estandar();
        pulsar_botonera(&mut presentador, ASCENSOR_2, 3);
        pasar_segundos(&mut presentador, 10);
        pulsar_llamada(&mut presentador, 4);
        let simulador = presentador.control().simulador();
        assert_eq!(
            simulador.posicion_del_ascensor(ASCENSOR_2).unwrap().estado,
            EstadoDeAscensor::Desplazandose {
                planta_de_destino: planta(4)
            }
        );
        assert_eq!(
            simulador.posicion_del_ascensor(ASCENSOR_1).unwrap().estado,
            EstadoDeAscensor::Parado
        );
        assert_eq!(
            mensaje(&presentador),
            informacion("Llamada en la planta 4: atendida por el ascensor 2")
        );
    }

    #[test]
    fn el_mensaje_de_una_llamada_atendida_indica_la_planta_y_el_ascensor_que_la_atiende() {
        let mut presentador = presentador_estandar();
        pulsar_llamada(&mut presentador, 4);
        assert_eq!(
            mensaje(&presentador),
            informacion("Llamada en la planta 4: atendida por el ascensor 1")
        );
    }

    #[test]
    fn el_mensaje_de_una_llamada_sin_ascensores_libres_indica_que_queda_pendiente() {
        let mut presentador = presentador_estandar();
        pulsar_botonera(&mut presentador, ASCENSOR_1, 1);
        pulsar_botonera(&mut presentador, ASCENSOR_2, 5);
        pulsar_botonera(&mut presentador, ASCENSOR_3, 7);
        pulsar_llamada(&mut presentador, 2);
        assert_eq!(
            mensaje(&presentador),
            informacion("Llamada en la planta 2: pendiente hasta que quede libre algún ascensor")
        );
    }

    #[test]
    fn el_mensaje_de_una_llamada_repetida_indica_que_ya_estaba_en_curso() {
        let mut presentador = presentador_estandar();
        pulsar_llamada(&mut presentador, 4);
        pulsar_llamada(&mut presentador, 4);
        assert_eq!(
            mensaje(&presentador),
            informacion("Llamada en la planta 4: ya estaba en curso")
        );
    }

    #[test]
    fn el_mensaje_de_una_llamada_a_una_planta_inexistente_es_un_error() {
        let mut presentador = presentador_estandar();
        pulsar_llamada(&mut presentador, 9);
        assert_eq!(
            mensaje(&presentador),
            error("Llamada en la planta 9: la planta no existe en el edificio")
        );
    }

    #[test]
    fn si_falla_el_historico_al_pulsar_un_boton_de_llamada_el_mensaje_es_un_error_aunque_el_ascensor_acuda()
     {
        let mut presentador = presentador_con_historico(
            ConfiguracionDelEdificio::estandar(),
            HistoricoQueFallaAlRegistrar,
        );
        pulsar_llamada(&mut presentador, 3);
        assert_eq!(
            mensaje(&presentador),
            error(
                "Llamada en la planta 3: no se ha podido acceder al histórico de movimientos: fallo simulado"
            )
        );
        assert_eq!(
            presentador
                .control()
                .simulador()
                .posicion_del_ascensor(ASCENSOR_1)
                .unwrap()
                .estado,
            EstadoDeAscensor::Desplazandose {
                planta_de_destino: planta(3)
            }
        );
    }

    // ---- Botonera ----

    #[test]
    fn pulsar_un_boton_de_la_botonera_envia_ese_ascensor_a_esa_planta() {
        let mut presentador = presentador_estandar();
        pulsar_botonera(&mut presentador, ASCENSOR_2, 3);
        let simulador = presentador.control().simulador();
        assert_eq!(
            simulador.posicion_del_ascensor(ASCENSOR_2).unwrap().estado,
            EstadoDeAscensor::Desplazandose {
                planta_de_destino: planta(3)
            }
        );
        let movimientos = presentador.control().historico().todos_los_movimientos();
        assert_eq!(movimientos.len(), 1);
        assert_eq!(movimientos[0].identificador_de_ascensor, ASCENSOR_2);
        assert_eq!(movimientos[0].planta_de_destino, planta(3));
        assert_eq!(movimientos[0].motivo, MotivoDeMovimiento::Botonera);
    }

    #[test]
    fn el_mensaje_de_una_orden_de_la_botonera_aceptada_indica_el_ascensor_y_la_planta_de_destino() {
        let mut presentador = presentador_estandar();
        pulsar_botonera(&mut presentador, ASCENSOR_2, -2);
        assert_eq!(
            mensaje(&presentador),
            informacion("Ascensor 2: va a la planta -2")
        );
    }

    #[test]
    fn el_mensaje_de_una_orden_de_la_botonera_a_la_planta_en_la_que_esta_parado_el_ascensor_indica_que_ya_esta_en_ella()
     {
        let mut presentador = presentador_estandar();
        pulsar_botonera(&mut presentador, ASCENSOR_1, 0);
        assert_eq!(
            mensaje(&presentador),
            informacion("Ascensor 1: ya está en la planta 0")
        );
    }

    #[test]
    fn el_mensaje_de_una_orden_de_la_botonera_a_un_ascensor_que_se_desplaza_es_un_error_de_ascensor_ocupado()
     {
        let mut presentador = presentador_estandar();
        pulsar_botonera(&mut presentador, ASCENSOR_1, 3);
        pulsar_botonera(&mut presentador, ASCENSOR_1, 5);
        assert_eq!(
            mensaje(&presentador),
            error(
                "Ascensor 1, botón de la planta 5: el ascensor se está desplazando y no acepta nuevas órdenes"
            )
        );
    }

    #[test]
    fn una_accion_nueva_sustituye_el_mensaje_de_la_accion_anterior() {
        let mut presentador = presentador_estandar();
        pulsar_llamada(&mut presentador, 9);
        assert!(matches!(
            mensaje(&presentador),
            MensajeParaElUsuario::Error(_)
        ));
        pulsar_llamada(&mut presentador, 4);
        assert_eq!(
            mensaje(&presentador),
            informacion("Llamada en la planta 4: atendida por el ascensor 1")
        );
        pulsar_botonera(&mut presentador, ASCENSOR_2, 3);
        assert_eq!(
            mensaje(&presentador),
            informacion("Ascensor 2: va a la planta 3")
        );
    }

    // ---- Paso del tiempo ----

    #[test]
    fn pasar_el_tiempo_avanza_la_simulacion_el_tiempo_real_transcurrido() {
        let mut presentador = presentador_estandar();
        presentador.pasar_el_tiempo(Duration::from_millis(250));
        presentador.pasar_el_tiempo(Duration::from_millis(250));
        assert_eq!(
            presentador.control().simulador().instante_actual(),
            Duration::from_millis(500)
        );
    }

    #[test]
    fn pasar_el_tiempo_avanza_la_simulacion_como_maximo_un_segundo_aunque_haya_transcurrido_mas_tiempo_real()
     {
        let mut presentador = presentador_estandar();
        presentador.pasar_el_tiempo(Duration::from_secs(5));
        assert_eq!(
            presentador.control().simulador().instante_actual(),
            AVANCE_MAXIMO_DE_LA_SIMULACION_POR_FOTOGRAMA
        );
        assert_eq!(
            presentador.control().simulador().instante_actual(),
            Duration::from_secs(1)
        );
    }

    #[test]
    fn pasar_el_tiempo_sin_errores_conserva_el_ultimo_mensaje_para_el_usuario() {
        let mut presentador = presentador_estandar();
        pulsar_llamada(&mut presentador, 4);
        let mensaje_anterior = mensaje(&presentador);
        pasar_segundos(&mut presentador, 3);
        assert_eq!(mensaje(&presentador), mensaje_anterior);
    }

    #[test]
    fn si_falla_el_historico_al_pasar_el_tiempo_el_mensaje_para_el_usuario_es_un_error() {
        let configuracion_de_un_ascensor = ConfiguracionDelEdificio {
            numero_de_ascensores: 1,
            ..ConfiguracionDelEdificio::estandar()
        };
        let mut presentador =
            presentador_con_historico(configuracion_de_un_ascensor, HistoricoQueFallaAlRegistrar);
        pulsar_botonera(&mut presentador, ASCENSOR_1, 1);
        assert_eq!(
            mensaje(&presentador),
            error(
                "Ascensor 1, botón de la planta 1: no se ha podido acceder al histórico de movimientos: fallo simulado"
            )
        );
        pulsar_llamada(&mut presentador, 3);
        assert_eq!(
            mensaje(&presentador),
            informacion("Llamada en la planta 3: pendiente hasta que quede libre algún ascensor")
        );
        pasar_segundos(&mut presentador, 5);
        assert_eq!(
            mensaje(&presentador),
            informacion("Llamada en la planta 3: pendiente hasta que quede libre algún ascensor")
        );
        pasar_segundos(&mut presentador, 1);
        assert_eq!(
            mensaje(&presentador),
            error(
                "Al avanzar el tiempo: no se ha podido acceder al histórico de movimientos: fallo simulado"
            )
        );
    }
}
