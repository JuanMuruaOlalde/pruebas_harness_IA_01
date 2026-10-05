//! Historico permanente de movimientos en un archivo de texto con campos separados por `;`.
//!
//! Se lee el archivo entero al abrirlo y las consultas se responden desde una copia en memoria.
//! Cada movimiento nuevo se anade al final del archivo y a la copia; nunca se borra nada.

use std::fs::{self, OpenOptions};
use std::io::{ErrorKind, Write};
use std::path::{Path, PathBuf};

use super::historico_de_movimientos_en_memoria::HistoricoDeMovimientosEnMemoria;
use crate::dominio::ascensor::IdentificadorDeAscensor;
use crate::dominio::fecha_y_hora::FechaYHora;
use crate::dominio::historico_de_movimientos::{
    ErrorDeHistoricoDeMovimientos, HistoricoDeMovimientos,
};
use crate::dominio::movimiento::{MotivoDeMovimiento, MovimientoDeAscensor};
use crate::dominio::planta::IdentificadorDePlanta;

const CABECERA: &str = "fecha_y_hora;ascensor;planta_de_origen;planta_de_destino;motivo";
const SEPARADOR: &str = ";";

#[derive(Debug, Clone)]
pub struct HistoricoDeMovimientosEnArchivo {
    ruta: PathBuf,
    /// Todo lo que hay en el archivo, para responder las consultas sin volver a leerlo.
    copia_en_memoria: HistoricoDeMovimientosEnMemoria,
}

impl HistoricoDeMovimientosEnArchivo {
    /// Lee el archivo, si existe; si no existe, el historico empieza vacio y el archivo se crea
    /// al registrar el primer movimiento. Una linea ilegible da error con su numero de linea.
    pub fn abrir(ruta: impl Into<PathBuf>) -> Result<Self, ErrorDeHistoricoDeMovimientos> {
        let ruta = ruta.into();
        let movimientos = match fs::read_to_string(&ruta) {
            Ok(contenido) => movimientos_del_contenido_del_archivo(&contenido)?,
            Err(error) if error.kind() == ErrorKind::NotFound => Vec::new(),
            Err(error) => return Err(error_de_acceso(&error)),
        };
        Ok(Self {
            ruta,
            copia_en_memoria: HistoricoDeMovimientosEnMemoria::con_movimientos(movimientos),
        })
    }

    pub fn todos_los_movimientos(&self) -> &[MovimientoDeAscensor] {
        self.copia_en_memoria.todos_los_movimientos()
    }

    /// Si el archivo no existe o esta vacio, escribe antes la cabecera.
    fn anadir_al_final_del_archivo(
        &self,
        movimiento: &MovimientoDeAscensor,
    ) -> Result<(), std::io::Error> {
        crear_la_carpeta_si_no_existe(&self.ruta)?;
        let hay_que_escribir_la_cabecera = fs::metadata(&self.ruta)
            .map(|metadatos| metadatos.len() == 0)
            .unwrap_or(true);
        let mut archivo = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.ruta)?;
        if hay_que_escribir_la_cabecera {
            writeln!(archivo, "{CABECERA}")?;
        }
        writeln!(archivo, "{}", movimiento_como_linea_de_texto(movimiento))
    }
}

impl HistoricoDeMovimientos for HistoricoDeMovimientosEnArchivo {
    /// Primero en el archivo y, solo si se ha podido escribir, en la copia en memoria.
    fn registrar(
        &mut self,
        movimiento: MovimientoDeAscensor,
    ) -> Result<(), ErrorDeHistoricoDeMovimientos> {
        self.anadir_al_final_del_archivo(&movimiento)
            .map_err(|error| error_de_acceso(&error))?;
        self.copia_en_memoria.registrar(movimiento)
    }

    fn movimientos_desde(
        &self,
        fecha_y_hora: FechaYHora,
    ) -> Result<Vec<MovimientoDeAscensor>, ErrorDeHistoricoDeMovimientos> {
        self.copia_en_memoria.movimientos_desde(fecha_y_hora)
    }
}

fn error_de_acceso(error: &std::io::Error) -> ErrorDeHistoricoDeMovimientos {
    ErrorDeHistoricoDeMovimientos::AccesoAlAlmacenamiento {
        detalle: error.to_string(),
    }
}

fn crear_la_carpeta_si_no_existe(ruta: &Path) -> Result<(), std::io::Error> {
    match ruta.parent() {
        Some(carpeta) if !carpeta.as_os_str().is_empty() => fs::create_dir_all(carpeta),
        _ => Ok(()),
    }
}

/// La primera linea es la cabecera y no se interpreta. Las lineas se cuentan desde 1.
fn movimientos_del_contenido_del_archivo(
    contenido: &str,
) -> Result<Vec<MovimientoDeAscensor>, ErrorDeHistoricoDeMovimientos> {
    contenido
        .lines()
        .enumerate()
        .skip(1)
        .map(|(indice, linea)| {
            let numero_de_linea = indice + 1;
            movimiento_desde_linea_de_texto(linea)
                .ok_or(ErrorDeHistoricoDeMovimientos::LineaIlegible { numero_de_linea })
        })
        .collect()
}

/// Devuelve `None` si la linea no tiene los cinco campos o alguno no se puede interpretar.
fn movimiento_desde_linea_de_texto(linea: &str) -> Option<MovimientoDeAscensor> {
    let campos: Vec<&str> = linea.split(SEPARADOR).collect();
    let [fecha_y_hora, ascensor, origen, destino, motivo] = campos.as_slice() else {
        return None;
    };
    Some(MovimientoDeAscensor {
        fecha_y_hora: fecha_y_hora.parse().ok()?,
        identificador_de_ascensor: IdentificadorDeAscensor(ascensor.parse().ok()?),
        planta_de_origen: IdentificadorDePlanta(origen.parse().ok()?),
        planta_de_destino: IdentificadorDePlanta(destino.parse().ok()?),
        motivo: motivo_desde_texto(motivo)?,
    })
}

fn movimiento_como_linea_de_texto(movimiento: &MovimientoDeAscensor) -> String {
    [
        movimiento.fecha_y_hora.to_string(),
        movimiento.identificador_de_ascensor.0.to_string(),
        movimiento.planta_de_origen.0.to_string(),
        movimiento.planta_de_destino.0.to_string(),
        motivo_como_texto(movimiento.motivo).to_string(),
    ]
    .join(SEPARADOR)
}

fn motivo_desde_texto(texto: &str) -> Option<MotivoDeMovimiento> {
    match texto {
        "llamada" => Some(MotivoDeMovimiento::Llamada),
        "botonera" => Some(MotivoDeMovimiento::Botonera),
        "reposicionamiento" => Some(MotivoDeMovimiento::Reposicionamiento),
        _ => None,
    }
}

fn motivo_como_texto(motivo: MotivoDeMovimiento) -> &'static str {
    match motivo {
        MotivoDeMovimiento::Llamada => "llamada",
        MotivoDeMovimiento::Botonera => "botonera",
        MotivoDeMovimiento::Reposicionamiento => "reposicionamiento",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Carpeta temporal unica por test, que se borra al empezar y al acabar.
    struct CarpetaTemporal(PathBuf);

    impl CarpetaTemporal {
        fn nueva(nombre_del_test: &str) -> Self {
            let carpeta = std::env::temp_dir().join(format!(
                "historico_en_archivo_{nombre_del_test}_{}",
                std::process::id()
            ));
            let _ = fs::remove_dir_all(&carpeta);
            Self(carpeta)
        }

        fn ruta_del_archivo(&self) -> PathBuf {
            self.0.join("historico.csv")
        }
    }

    impl Drop for CarpetaTemporal {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.0);
        }
    }

    fn movimiento(
        hora: u32,
        ascensor: u32,
        origen: i32,
        destino: i32,
        motivo: MotivoDeMovimiento,
    ) -> MovimientoDeAscensor {
        MovimientoDeAscensor {
            fecha_y_hora: FechaYHora::nueva(2026, 10, 5, hora, 0, 10).unwrap(),
            identificador_de_ascensor: IdentificadorDeAscensor(ascensor),
            planta_de_origen: IdentificadorDePlanta(origen),
            planta_de_destino: IdentificadorDePlanta(destino),
            motivo,
        }
    }

    #[test]
    fn abrir_un_historico_en_archivo_inexistente_da_un_historico_vacio() {
        let carpeta = CarpetaTemporal::nueva("inexistente");
        let historico = HistoricoDeMovimientosEnArchivo::abrir(carpeta.ruta_del_archivo()).unwrap();
        assert!(historico.todos_los_movimientos().is_empty());
        assert!(!carpeta.ruta_del_archivo().exists());
    }

    #[test]
    fn el_archivo_del_historico_tiene_una_cabecera_y_una_linea_de_texto_por_movimiento() {
        let carpeta = CarpetaTemporal::nueva("formato");
        let mut historico =
            HistoricoDeMovimientosEnArchivo::abrir(carpeta.ruta_del_archivo()).unwrap();
        historico
            .registrar(movimiento(8, 1, 0, 4, MotivoDeMovimiento::Llamada))
            .unwrap();
        historico
            .registrar(movimiento(9, 2, -1, 3, MotivoDeMovimiento::Botonera))
            .unwrap();
        historico
            .registrar(movimiento(
                10,
                3,
                5,
                0,
                MotivoDeMovimiento::Reposicionamiento,
            ))
            .unwrap();
        let contenido = fs::read_to_string(carpeta.ruta_del_archivo()).unwrap();
        assert_eq!(
            contenido,
            "fecha_y_hora;ascensor;planta_de_origen;planta_de_destino;motivo\n\
             2026-10-05T08:00:10;1;0;4;llamada\n\
             2026-10-05T09:00:10;2;-1;3;botonera\n\
             2026-10-05T10:00:10;3;5;0;reposicionamiento\n"
        );
    }

    #[test]
    fn los_movimientos_registrados_en_archivo_se_recuperan_iguales_al_reabrir_el_archivo() {
        let carpeta = CarpetaTemporal::nueva("reabrir");
        let registrados = [
            movimiento(8, 1, 0, 4, MotivoDeMovimiento::Llamada),
            movimiento(9, 2, -1, 3, MotivoDeMovimiento::Botonera),
            movimiento(10, 3, 5, 0, MotivoDeMovimiento::Reposicionamiento),
        ];
        let mut historico =
            HistoricoDeMovimientosEnArchivo::abrir(carpeta.ruta_del_archivo()).unwrap();
        for movimiento in registrados {
            historico.registrar(movimiento).unwrap();
        }
        let reabierto = HistoricoDeMovimientosEnArchivo::abrir(carpeta.ruta_del_archivo()).unwrap();
        assert_eq!(reabierto.todos_los_movimientos(), registrados);
    }

    #[test]
    fn registrar_en_archivo_anade_al_final_sin_borrar_los_movimientos_de_sesiones_anteriores() {
        let carpeta = CarpetaTemporal::nueva("anadir");
        let de_la_primera_sesion = movimiento(8, 1, 0, 4, MotivoDeMovimiento::Llamada);
        let de_la_segunda_sesion = movimiento(9, 2, 4, 1, MotivoDeMovimiento::Botonera);
        let mut primera_sesion =
            HistoricoDeMovimientosEnArchivo::abrir(carpeta.ruta_del_archivo()).unwrap();
        primera_sesion.registrar(de_la_primera_sesion).unwrap();
        let mut segunda_sesion =
            HistoricoDeMovimientosEnArchivo::abrir(carpeta.ruta_del_archivo()).unwrap();
        segunda_sesion.registrar(de_la_segunda_sesion).unwrap();
        let tercera_sesion =
            HistoricoDeMovimientosEnArchivo::abrir(carpeta.ruta_del_archivo()).unwrap();
        assert_eq!(
            tercera_sesion.todos_los_movimientos(),
            [de_la_primera_sesion, de_la_segunda_sesion]
        );
        let contenido = fs::read_to_string(carpeta.ruta_del_archivo()).unwrap();
        assert_eq!(contenido.matches(CABECERA).count(), 1);
    }

    #[test]
    fn movimientos_desde_en_un_historico_en_archivo_excluye_los_anteriores_a_esa_fecha_y_hora() {
        let carpeta = CarpetaTemporal::nueva("desde");
        let anterior = movimiento(7, 1, 0, 4, MotivoDeMovimiento::Llamada);
        let justo_en_la_fecha = movimiento(8, 2, 4, 1, MotivoDeMovimiento::Botonera);
        let posterior = movimiento(9, 3, 1, 2, MotivoDeMovimiento::Llamada);
        let mut historico =
            HistoricoDeMovimientosEnArchivo::abrir(carpeta.ruta_del_archivo()).unwrap();
        for movimiento in [anterior, justo_en_la_fecha, posterior] {
            historico.registrar(movimiento).unwrap();
        }
        let desde = FechaYHora::nueva(2026, 10, 5, 8, 0, 10).unwrap();
        assert_eq!(
            historico.movimientos_desde(desde),
            Ok(vec![justo_en_la_fecha, posterior])
        );
    }

    #[test]
    fn registrar_en_archivo_crea_la_carpeta_si_no_existe() {
        let carpeta = CarpetaTemporal::nueva("carpeta");
        let ruta = carpeta.0.join("subcarpeta").join("historico.csv");
        let mut historico = HistoricoDeMovimientosEnArchivo::abrir(&ruta).unwrap();
        historico
            .registrar(movimiento(8, 1, 0, 4, MotivoDeMovimiento::Llamada))
            .unwrap();
        assert!(ruta.is_file());
    }

    #[test]
    fn abrir_un_historico_en_archivo_con_una_linea_ilegible_da_error_con_el_numero_de_linea() {
        let carpeta = CarpetaTemporal::nueva("ilegible");
        fs::create_dir_all(&carpeta.0).unwrap();
        fs::write(
            carpeta.ruta_del_archivo(),
            format!("{CABECERA}\n2026-10-05T08:00:10;1;0;4;llamada\nesto no es un movimiento\n"),
        )
        .unwrap();
        let resultado = HistoricoDeMovimientosEnArchivo::abrir(carpeta.ruta_del_archivo());
        assert_eq!(
            resultado.err(),
            Some(ErrorDeHistoricoDeMovimientos::LineaIlegible { numero_de_linea: 3 })
        );
    }
}
