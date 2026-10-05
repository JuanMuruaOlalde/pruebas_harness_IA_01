use pruebas_harness_01::dominio::configuracion::ConfiguracionDelEdificio;
use pruebas_harness_01::dominio::simulador::SimuladorDeEdificio;

fn main() {
    let simulador = SimuladorDeEdificio::nuevo(ConfiguracionDelEdificio::estandar())
        .expect("la configuracion estandar es valida");
    for (identificador, posicion) in simulador.posiciones_de_todos_los_ascensores() {
        println!(
            "Ascensor {}: planta {} ({:?})",
            identificador.0, posicion.planta_actual.0, posicion.estado
        );
    }
}
