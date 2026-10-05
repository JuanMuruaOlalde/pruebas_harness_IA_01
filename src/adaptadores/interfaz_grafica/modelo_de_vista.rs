//! Modelo de vista: datos puros que describen lo que se dibuja, sin ningun tipo de egui.

use crate::dominio::ascensor::IdentificadorDeAscensor;
use crate::dominio::planta::IdentificadorDePlanta;

/// Todo lo que la vista necesita para dibujar un fotograma.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModeloDeVista {
    pub texto_de_la_fecha_y_hora: String,
    /// De la planta mas alta a la mas baja.
    pub plantas: Vec<PlantaEnLaVista>,
    /// Ordenados por identificador.
    pub ascensores: Vec<AscensorEnLaVista>,
    pub texto_de_las_llamadas_pendientes: String,
    pub mensaje_para_el_usuario: Option<MensajeParaElUsuario>,
}

/// Una fila de la rejilla: el boton de llamada de la planta y una celda por cada hueco.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlantaEnLaVista {
    pub planta: IdentificadorDePlanta,
    pub boton_de_llamada_encendido: bool,
    /// Una por ascensor, en el mismo orden que `ModeloDeVista::ascensores`.
    pub celdas_de_los_huecos: Vec<CeldaDelHueco>,
}

/// Contenido de la celda de un hueco para una planta.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CeldaDelHueco {
    Vacia,
    /// El ascensor esta en esta planta, con esta marcha.
    Ascensor(MarchaDelAscensor),
    /// Esta planta es el destino de un ascensor que todavia no ha llegado a ella.
    DestinoDelAscensor,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MarchaDelAscensor {
    Parado,
    Subiendo,
    Bajando,
    /// Esta en su planta de destino, en la fase de parada.
    Llegando,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AscensorEnLaVista {
    pub identificador_de_ascensor: IdentificadorDeAscensor,
    pub planta_actual: IdentificadorDePlanta,
    pub marcha: MarchaDelAscensor,
    pub planta_de_destino: Option<IdentificadorDePlanta>,
    pub descripcion_del_estado: String,
    /// Un boton por planta, de la mas alta a la mas baja.
    pub botonera: Vec<BotonDeLaBotonera>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BotonDeLaBotonera {
    pub planta: IdentificadorDePlanta,
    pub habilitado: bool,
    pub encendido: bool,
}

/// Resultado de la ultima accion del usuario, o ultimo error.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MensajeParaElUsuario {
    Informacion(String),
    Error(String),
}

/// Lo que la vista devuelve cuando el usuario pulsa algo.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AccionDelUsuario {
    PulsarBotonDeLlamada {
        planta: IdentificadorDePlanta,
    },
    PulsarBotonDeLaBotonera {
        identificador_de_ascensor: IdentificadorDeAscensor,
        planta_de_destino: IdentificadorDePlanta,
    },
}
