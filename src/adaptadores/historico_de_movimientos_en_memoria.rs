//! Historico de movimientos que vive solo en memoria: no es permanente.

use crate::dominio::fecha_y_hora::FechaYHora;
use crate::dominio::historico_de_movimientos::{
    ErrorDeHistoricoDeMovimientos, HistoricoDeMovimientos,
};
use crate::dominio::movimiento::MovimientoDeAscensor;

#[derive(Debug, Clone, Default)]
pub struct HistoricoDeMovimientosEnMemoria {
    movimientos: Vec<MovimientoDeAscensor>,
}

impl HistoricoDeMovimientosEnMemoria {
    pub fn nuevo() -> Self {
        Self::default()
    }

    pub fn con_movimientos(movimientos_iniciales: Vec<MovimientoDeAscensor>) -> Self {
        Self {
            movimientos: movimientos_iniciales,
        }
    }

    pub fn todos_los_movimientos(&self) -> &[MovimientoDeAscensor] {
        &self.movimientos
    }
}

impl HistoricoDeMovimientos for HistoricoDeMovimientosEnMemoria {
    fn registrar(
        &mut self,
        movimiento: MovimientoDeAscensor,
    ) -> Result<(), ErrorDeHistoricoDeMovimientos> {
        self.movimientos.push(movimiento);
        Ok(())
    }

    fn movimientos_desde(
        &self,
        fecha_y_hora: FechaYHora,
    ) -> Result<Vec<MovimientoDeAscensor>, ErrorDeHistoricoDeMovimientos> {
        Ok(self
            .movimientos
            .iter()
            .filter(|movimiento| movimiento.fecha_y_hora >= fecha_y_hora)
            .copied()
            .collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dominio::ascensor::IdentificadorDeAscensor;
    use crate::dominio::movimiento::MotivoDeMovimiento;
    use crate::dominio::planta::IdentificadorDePlanta;

    fn movimiento_a_las(hora: u32, minuto: u32, planta_de_destino: i32) -> MovimientoDeAscensor {
        MovimientoDeAscensor {
            fecha_y_hora: FechaYHora::nueva(2026, 10, 5, hora, minuto, 0).unwrap(),
            identificador_de_ascensor: IdentificadorDeAscensor(1),
            planta_de_origen: IdentificadorDePlanta(0),
            planta_de_destino: IdentificadorDePlanta(planta_de_destino),
            motivo: MotivoDeMovimiento::Llamada,
        }
    }

    #[test]
    fn un_historico_en_memoria_nuevo_esta_vacio() {
        let historico = HistoricoDeMovimientosEnMemoria::nuevo();
        assert!(historico.todos_los_movimientos().is_empty());
        let desde = FechaYHora::nueva(2000, 1, 1, 0, 0, 0).unwrap();
        assert_eq!(historico.movimientos_desde(desde), Ok(vec![]));
    }

    #[test]
    fn los_movimientos_registrados_en_memoria_se_recuperan_en_el_orden_en_que_se_registraron() {
        let mut historico = HistoricoDeMovimientosEnMemoria::nuevo();
        let primero = movimiento_a_las(9, 0, 3);
        let segundo = movimiento_a_las(8, 0, 5);
        historico.registrar(primero).unwrap();
        historico.registrar(segundo).unwrap();
        assert_eq!(historico.todos_los_movimientos(), [primero, segundo]);
        let desde = FechaYHora::nueva(2000, 1, 1, 0, 0, 0).unwrap();
        assert_eq!(
            historico.movimientos_desde(desde),
            Ok(vec![primero, segundo])
        );
    }

    #[test]
    fn movimientos_desde_una_fecha_y_hora_excluye_los_anteriores_e_incluye_los_de_esa_misma_fecha_y_hora()
     {
        let anterior = movimiento_a_las(7, 59, 1);
        let justo_en_la_fecha = movimiento_a_las(8, 0, 2);
        let posterior = movimiento_a_las(8, 1, 3);
        let historico = HistoricoDeMovimientosEnMemoria::con_movimientos(vec![
            anterior,
            justo_en_la_fecha,
            posterior,
        ]);
        let desde = FechaYHora::nueva(2026, 10, 5, 8, 0, 0).unwrap();
        assert_eq!(
            historico.movimientos_desde(desde),
            Ok(vec![justo_en_la_fecha, posterior])
        );
    }
}
