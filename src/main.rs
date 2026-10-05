//! Raiz de composicion: une el simulador, el historico en archivo y el reloj del sistema.

use std::time::Duration;

use pruebas_harness_01::adaptadores::historico_de_movimientos_en_archivo::HistoricoDeMovimientosEnArchivo;
use pruebas_harness_01::adaptadores::reloj_del_sistema::fecha_y_hora_local_actual;
use pruebas_harness_01::aplicacion::control_de_trafico::{
    ConfiguracionDelControlDeTrafico, ControlDeTrafico,
};
use pruebas_harness_01::dominio::ascensor::IdentificadorDeAscensor;
use pruebas_harness_01::dominio::configuracion::ConfiguracionDelEdificio;
use pruebas_harness_01::dominio::historico_de_movimientos::HistoricoDeMovimientos;
use pruebas_harness_01::dominio::planta::IdentificadorDePlanta;
use pruebas_harness_01::dominio::simulador::SimuladorDeEdificio;

const RUTA_DEL_HISTORICO: &str = "datos/historico_de_movimientos.csv";

fn main() {
    let simulador = SimuladorDeEdificio::nuevo(ConfiguracionDelEdificio::estandar())
        .expect("la configuracion estandar es valida");
    let historico = HistoricoDeMovimientosEnArchivo::abrir(RUTA_DEL_HISTORICO)
        .expect("el historico de movimientos se puede abrir");
    let mut control = ControlDeTrafico::nuevo(
        simulador,
        historico,
        fecha_y_hora_local_actual(),
        ConfiguracionDelControlDeTrafico::estandar(),
    );
    demostracion(&mut control);
}

fn demostracion<H: HistoricoDeMovimientos>(control: &mut ControlDeTrafico<H>) {
    println!(
        "Inicio de la demostracion: {}",
        control.fecha_y_hora_actual()
    );

    println!(
        "Llamada en la planta 4: {:?}",
        control.pulsar_boton_de_llamada(IdentificadorDePlanta(4))
    );
    println!(
        "Llamada en la planta -1: {:?}",
        control.pulsar_boton_de_llamada(IdentificadorDePlanta(-1))
    );
    println!(
        "Botonera del ascensor 3 a la planta 7: {:?}",
        control.pulsar_boton_de_la_botonera(IdentificadorDeAscensor(3), IdentificadorDePlanta(7))
    );
    println!(
        "Llamada en la planta 2: {:?}",
        control.pulsar_boton_de_llamada(IdentificadorDePlanta(2))
    );
    mostrar_estado(control);

    for _ in 0..3 {
        println!(
            "Avanzan 6 segundos: {:?}",
            control.avanzar_tiempo(Duration::from_secs(6))
        );
        mostrar_estado(control);
    }
}

fn mostrar_estado<H: HistoricoDeMovimientos>(control: &ControlDeTrafico<H>) {
    println!("  Fecha y hora: {}", control.fecha_y_hora_actual());
    for (identificador, posicion) in control.simulador().posiciones_de_todos_los_ascensores() {
        println!(
            "  Ascensor {}: planta {} ({:?})",
            identificador.0, posicion.planta_actual.0, posicion.estado
        );
    }
    println!("  Llamadas pendientes: {:?}", control.llamadas_pendientes());
}
