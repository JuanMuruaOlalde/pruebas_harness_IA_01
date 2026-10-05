//! Raiz de composicion: une el simulador, el historico en archivo, el reloj del sistema y la
//! interfaz grafica.

use std::process::ExitCode;

use pruebas_harness_01::adaptadores::historico_de_movimientos_en_archivo::HistoricoDeMovimientosEnArchivo;
use pruebas_harness_01::adaptadores::interfaz_grafica::presentador::PresentadorDelSimulador;
use pruebas_harness_01::adaptadores::interfaz_grafica::vista_con_egui::ejecutar_interfaz_grafica;
use pruebas_harness_01::adaptadores::reloj_del_sistema::fecha_y_hora_local_actual;
use pruebas_harness_01::aplicacion::control_de_trafico::{
    ConfiguracionDelControlDeTrafico, ControlDeTrafico,
};
use pruebas_harness_01::dominio::configuracion::ConfiguracionDelEdificio;
use pruebas_harness_01::dominio::simulador::SimuladorDeEdificio;

const RUTA_DEL_HISTORICO: &str = "datos/historico_de_movimientos.csv";

fn main() -> ExitCode {
    match iniciar_el_simulador() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("No se ha podido iniciar el simulador: {error}");
            ExitCode::FAILURE
        }
    }
}

/// Compone el simulador con sus adaptadores y abre la interfaz grafica.
/// No vuelve hasta que se cierra la ventana.
fn iniciar_el_simulador() -> Result<(), Box<dyn std::error::Error>> {
    let simulador = SimuladorDeEdificio::nuevo(ConfiguracionDelEdificio::estandar())?;
    let historico = HistoricoDeMovimientosEnArchivo::abrir(RUTA_DEL_HISTORICO)?;
    let control = ControlDeTrafico::nuevo(
        simulador,
        historico,
        fecha_y_hora_local_actual(),
        ConfiguracionDelControlDeTrafico::estandar(),
    );
    ejecutar_interfaz_grafica(PresentadorDelSimulador::nuevo(control))?;
    Ok(())
}
