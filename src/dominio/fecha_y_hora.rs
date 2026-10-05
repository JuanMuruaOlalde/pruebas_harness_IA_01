//! Fecha y hora local de calendario, sin zona horaria.
//! Es el unico modulo del dominio que conoce la biblioteca `chrono`.

use std::fmt;
use std::str::FromStr;
use std::time::Duration;

use chrono::{Datelike, Days, NaiveDate, NaiveDateTime, TimeDelta, Timelike, Weekday};

const FORMATO_ISO_8601: &str = "%Y-%m-%dT%H:%M:%S%.f";

/// Dia de la semana de una fecha.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DiaDeLaSemana {
    Lunes,
    Martes,
    Miercoles,
    Jueves,
    Viernes,
    Sabado,
    Domingo,
}

/// Fecha y hora local, con fracciones de segundo. Se ordena cronologicamente.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FechaYHora(NaiveDateTime);

/// El texto no es una fecha y hora en formato ISO 8601 `AAAA-MM-DDTHH:MM:SS`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ErrorDeFormatoDeFechaYHora;

impl FechaYHora {
    /// Devuelve `None` si la combinacion no existe (30 de febrero, hora 24...).
    pub fn nueva(
        anio: i32,
        mes: u32,
        dia: u32,
        hora: u32,
        minuto: u32,
        segundo: u32,
    ) -> Option<FechaYHora> {
        let fecha = NaiveDate::from_ymd_opt(anio, mes, dia)?;
        let fecha_y_hora = fecha.and_hms_opt(hora, minuto, segundo)?;
        Some(FechaYHora(fecha_y_hora))
    }

    /// Fecha y hora a partir de la hora local del sistema, ya leida por un adaptador.
    pub(crate) fn desde_hora_local_de_chrono(fecha_y_hora_local: NaiveDateTime) -> FechaYHora {
        FechaYHora(fecha_y_hora_local)
    }

    pub fn dia_de_la_semana(&self) -> DiaDeLaSemana {
        match self.0.weekday() {
            Weekday::Mon => DiaDeLaSemana::Lunes,
            Weekday::Tue => DiaDeLaSemana::Martes,
            Weekday::Wed => DiaDeLaSemana::Miercoles,
            Weekday::Thu => DiaDeLaSemana::Jueves,
            Weekday::Fri => DiaDeLaSemana::Viernes,
            Weekday::Sat => DiaDeLaSemana::Sabado,
            Weekday::Sun => DiaDeLaSemana::Domingo,
        }
    }

    /// Hora del dia, de 0 a 23: es la franja horaria.
    pub fn hora_del_dia(&self) -> u32 {
        self.0.hour()
    }

    /// Avanza esta fecha y hora la duracion indicada. Si se saliera del calendario, no cambia.
    pub fn sumar(&self, duracion: Duration) -> FechaYHora {
        let suma = TimeDelta::from_std(duracion)
            .ok()
            .and_then(|incremento| self.0.checked_add_signed(incremento));
        FechaYHora(suma.unwrap_or(self.0))
    }

    /// Retrocede esta fecha y hora el numero de dias indicado, conservando la hora.
    /// Si se saliera del calendario, no cambia.
    pub fn restar_dias(&self, dias: u32) -> FechaYHora {
        let resta = self.0.checked_sub_days(Days::new(u64::from(dias)));
        FechaYHora(resta.unwrap_or(self.0))
    }
}

impl fmt::Display for FechaYHora {
    fn fmt(&self, formato: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formato, "{}", self.0.format(FORMATO_ISO_8601))
    }
}

impl FromStr for FechaYHora {
    type Err = ErrorDeFormatoDeFechaYHora;

    fn from_str(texto: &str) -> Result<Self, Self::Err> {
        NaiveDateTime::parse_from_str(texto, FORMATO_ISO_8601)
            .map(FechaYHora)
            .map_err(|_| ErrorDeFormatoDeFechaYHora)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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

    #[test]
    fn dia_de_la_semana_de_fechas_conocidas_es_el_correcto() {
        let casos = [
            (fecha_y_hora(2026, 10, 5, 8, 0, 0), DiaDeLaSemana::Lunes),
            (fecha_y_hora(2026, 9, 29, 8, 0, 0), DiaDeLaSemana::Martes),
            (fecha_y_hora(2026, 10, 7, 8, 0, 0), DiaDeLaSemana::Miercoles),
            (fecha_y_hora(2026, 10, 8, 8, 0, 0), DiaDeLaSemana::Jueves),
            (fecha_y_hora(2026, 10, 9, 8, 0, 0), DiaDeLaSemana::Viernes),
            (fecha_y_hora(2026, 10, 10, 8, 0, 0), DiaDeLaSemana::Sabado),
            (fecha_y_hora(2026, 10, 11, 8, 0, 0), DiaDeLaSemana::Domingo),
        ];
        for (fecha, dia_esperado) in casos {
            assert_eq!(fecha.dia_de_la_semana(), dia_esperado, "{fecha}");
        }
    }

    #[test]
    fn hora_del_dia_es_la_hora_sin_minutos_ni_segundos() {
        assert_eq!(fecha_y_hora(2026, 10, 5, 8, 59, 59).hora_del_dia(), 8);
        assert_eq!(fecha_y_hora(2026, 10, 5, 0, 0, 0).hora_del_dia(), 0);
        assert_eq!(fecha_y_hora(2026, 10, 5, 23, 30, 0).hora_del_dia(), 23);
    }

    #[test]
    fn sumar_una_duracion_avanza_la_fecha_y_hora_esa_duracion() {
        assert_eq!(
            fecha_y_hora(2026, 10, 5, 8, 0, 0).sumar(Duration::from_secs(75)),
            fecha_y_hora(2026, 10, 5, 8, 1, 15)
        );
    }

    #[test]
    fn sumar_una_duracion_que_pasa_de_medianoche_cambia_de_dia_y_de_dia_de_la_semana() {
        let resultado = fecha_y_hora(2026, 10, 5, 23, 59, 30).sumar(Duration::from_secs(45));
        assert_eq!(resultado, fecha_y_hora(2026, 10, 6, 0, 0, 15));
        assert_eq!(resultado.dia_de_la_semana(), DiaDeLaSemana::Martes);
    }

    #[test]
    fn restar_dias_retrocede_ese_numero_de_dias_conservando_la_hora() {
        assert_eq!(
            fecha_y_hora(2026, 10, 5, 8, 0, 0).restar_dias(28),
            fecha_y_hora(2026, 9, 7, 8, 0, 0)
        );
    }

    #[test]
    fn crear_una_fecha_y_hora_imposible_no_da_fecha_y_hora() {
        assert_eq!(FechaYHora::nueva(2026, 2, 30, 8, 0, 0), None);
        assert_eq!(FechaYHora::nueva(2026, 13, 1, 8, 0, 0), None);
        assert_eq!(FechaYHora::nueva(2026, 10, 5, 24, 0, 0), None);
        assert_eq!(FechaYHora::nueva(2026, 10, 5, 8, 60, 0), None);
    }

    #[test]
    fn fecha_y_hora_se_escribe_como_texto_iso_8601() {
        assert_eq!(
            fecha_y_hora(2026, 10, 5, 8, 0, 10).to_string(),
            "2026-10-05T08:00:10"
        );
    }

    #[test]
    fn fecha_y_hora_escrita_como_texto_y_leida_de_nuevo_es_la_misma_incluso_con_fraccion_de_segundo()
     {
        let sin_fraccion = fecha_y_hora(2026, 10, 5, 8, 0, 10);
        let con_fraccion = sin_fraccion.sumar(Duration::from_millis(500));
        assert_eq!(con_fraccion.to_string(), "2026-10-05T08:00:10.500");
        for fecha in [sin_fraccion, con_fraccion] {
            assert_eq!(fecha.to_string().parse::<FechaYHora>(), Ok(fecha));
        }
    }

    #[test]
    fn leer_una_fecha_y_hora_de_un_texto_mal_formado_da_error() {
        for texto in [
            "",
            "hola",
            "2026-10-05",
            "2026-10-05 08:00:10",
            "2026-13-05T08:00:10",
        ] {
            assert_eq!(
                texto.parse::<FechaYHora>(),
                Err(ErrorDeFormatoDeFechaYHora),
                "{texto:?}"
            );
        }
    }
}
