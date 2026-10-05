//! Vista con egui: unico archivo del proyecto que usa `eframe`.
//! Es lo mas fina posible: traduce el `ModeloDeVista` a widgets y los clics a `AccionDelUsuario`.
//! No tiene tests automaticos: se comprueba a mano.

use std::time::{Duration, Instant};

use eframe::egui;

use super::modelo_de_vista::{
    AccionDelUsuario, AscensorEnLaVista, CeldaDelHueco, MarchaDelAscensor, MensajeParaElUsuario,
    ModeloDeVista, PlantaEnLaVista,
};
use super::presentador::PresentadorDelSimulador;
use crate::dominio::historico_de_movimientos::HistoricoDeMovimientos;

const TITULO_DE_LA_VENTANA: &str = "Simulador de ascensores";
const TAMANIO_INICIAL_DE_LA_VENTANA: [f32; 2] = [960.0, 640.0];
/// Se repinta aunque el usuario no haga nada, porque la simulacion avanza sola.
const INTERVALO_DE_REPINTADO: Duration = Duration::from_millis(100);

pub struct InterfazGraficaConEgui<H: HistoricoDeMovimientos> {
    presentador: PresentadorDelSimulador<H>,
    instante_real_del_fotograma_anterior: Instant,
}

impl<H: HistoricoDeMovimientos> InterfazGraficaConEgui<H> {
    pub fn nueva(presentador: PresentadorDelSimulador<H>) -> Self {
        Self {
            presentador,
            instante_real_del_fotograma_anterior: Instant::now(),
        }
    }

    fn medir_el_tiempo_real_transcurrido_desde_el_fotograma_anterior(&mut self) -> Duration {
        let instante_real_actual = Instant::now();
        let tiempo_real_transcurrido =
            instante_real_actual.duration_since(self.instante_real_del_fotograma_anterior);
        self.instante_real_del_fotograma_anterior = instante_real_actual;
        tiempo_real_transcurrido
    }
}

impl<H: HistoricoDeMovimientos> eframe::App for InterfazGraficaConEgui<H> {
    fn ui(&mut self, ui: &mut egui::Ui, _marco_de_la_aplicacion: &mut eframe::Frame) {
        let tiempo_real_transcurrido =
            self.medir_el_tiempo_real_transcurrido_desde_el_fotograma_anterior();
        self.presentador.pasar_el_tiempo(tiempo_real_transcurrido);

        let modelo_de_vista = self.presentador.modelo_de_vista();
        let acciones_del_usuario = dibujar(ui, &modelo_de_vista);

        if !acciones_del_usuario.is_empty() {
            for accion in acciones_del_usuario {
                self.presentador.atender(accion);
            }
            ui.ctx().request_repaint();
        }
        ui.ctx().request_repaint_after(INTERVALO_DE_REPINTADO);
    }
}

/// Abre la ventana y no vuelve hasta que se cierra.
pub fn ejecutar_interfaz_grafica<H: HistoricoDeMovimientos + 'static>(
    presentador: PresentadorDelSimulador<H>,
) -> Result<(), eframe::Error> {
    let opciones = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title(TITULO_DE_LA_VENTANA)
            .with_inner_size(TAMANIO_INICIAL_DE_LA_VENTANA),
        ..Default::default()
    };
    eframe::run_native(
        TITULO_DE_LA_VENTANA,
        opciones,
        Box::new(|_contexto| Ok(Box::new(InterfazGraficaConEgui::nueva(presentador)))),
    )
}

/// Dibuja el modelo y devuelve las acciones del usuario. No tiene acceso al presentador.
fn dibujar(ui: &mut egui::Ui, modelo: &ModeloDeVista) -> Vec<AccionDelUsuario> {
    let mut acciones = Vec::new();

    egui::Panel::top("panel_superior").show(ui, |ui| {
        ui.horizontal(|ui| {
            ui.heading(TITULO_DE_LA_VENTANA);
            ui.label(&modelo.texto_de_la_fecha_y_hora);
        });
    });

    egui::Panel::bottom("panel_inferior").show(ui, |ui| {
        ui.label(&modelo.texto_de_las_llamadas_pendientes);
        match &modelo.mensaje_para_el_usuario {
            Some(MensajeParaElUsuario::Informacion(texto)) => {
                ui.label(texto);
            }
            Some(MensajeParaElUsuario::Error(texto)) => {
                ui.colored_label(ui.visuals().error_fg_color, texto);
            }
            None => {}
        }
    });

    egui::Panel::right("panel_de_las_botoneras").show(ui, |ui| {
        ui.horizontal_top(|ui| {
            for ascensor in &modelo.ascensores {
                dibujar_botonera(ui, ascensor, &mut acciones);
            }
        });
    });

    egui::CentralPanel::default().show(ui, |ui| {
        dibujar_rejilla(ui, modelo, &mut acciones);
    });

    acciones
}

fn dibujar_rejilla(
    ui: &mut egui::Ui,
    modelo: &ModeloDeVista,
    acciones: &mut Vec<AccionDelUsuario>,
) {
    egui::Grid::new("rejilla_de_plantas")
        .striped(true)
        .show(ui, |ui| {
            ui.strong("Planta");
            ui.strong("Llamada");
            for ascensor in &modelo.ascensores {
                ui.strong(titulo_del_ascensor(ascensor));
            }
            ui.end_row();

            for planta in &modelo.plantas {
                dibujar_fila_de_la_planta(ui, planta, &modelo.ascensores, acciones);
                ui.end_row();
            }
        });
}

fn dibujar_fila_de_la_planta(
    ui: &mut egui::Ui,
    planta: &PlantaEnLaVista,
    ascensores: &[AscensorEnLaVista],
    acciones: &mut Vec<AccionDelUsuario>,
) {
    ui.label(planta.planta.0.to_string());
    let boton_de_llamada = egui::Button::new("Llamar").selected(planta.boton_de_llamada_encendido);
    if ui.add(boton_de_llamada).clicked() {
        acciones.push(AccionDelUsuario::PulsarBotonDeLlamada {
            planta: planta.planta,
        });
    }
    for (celda, ascensor) in planta.celdas_de_los_huecos.iter().zip(ascensores) {
        ui.label(texto_de_la_celda(*celda, ascensor));
    }
}

/// Los simbolos son de los que incluyen las fuentes por defecto de egui en el texto normal.
fn texto_de_la_celda(celda: CeldaDelHueco, ascensor: &AscensorEnLaVista) -> String {
    match celda {
        CeldaDelHueco::Vacia => String::new(),
        CeldaDelHueco::DestinoDelAscensor => "○".to_string(),
        CeldaDelHueco::Ascensor(marcha) => format!(
            "{} {}",
            simbolo_de_la_marcha(marcha),
            ascensor.identificador_de_ascensor.0
        ),
    }
}

fn simbolo_de_la_marcha(marcha: MarchaDelAscensor) -> &'static str {
    match marcha {
        MarchaDelAscensor::Parado => "■",
        MarchaDelAscensor::Subiendo => "⬆",
        MarchaDelAscensor::Bajando => "⬇",
        MarchaDelAscensor::Llegando => "⏹",
    }
}

fn titulo_del_ascensor(ascensor: &AscensorEnLaVista) -> String {
    format!("Ascensor {}", ascensor.identificador_de_ascensor.0)
}

fn dibujar_botonera(
    ui: &mut egui::Ui,
    ascensor: &AscensorEnLaVista,
    acciones: &mut Vec<AccionDelUsuario>,
) {
    ui.vertical(|ui| {
        ui.strong(titulo_del_ascensor(ascensor));
        ui.label(&ascensor.descripcion_del_estado);
        for boton in &ascensor.botonera {
            let boton_de_la_botonera =
                egui::Button::new(boton.planta.0.to_string()).selected(boton.encendido);
            if ui
                .add_enabled(boton.habilitado, boton_de_la_botonera)
                .clicked()
            {
                acciones.push(AccionDelUsuario::PulsarBotonDeLaBotonera {
                    identificador_de_ascensor: ascensor.identificador_de_ascensor,
                    planta_de_destino: boton.planta,
                });
            }
        }
    });
}
