//! Plantas del edificio.

/// Numero entero que identifica una planta: 0 es la principal, positivos por encima, negativos por debajo.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct IdentificadorDePlanta(pub i32);

impl IdentificadorDePlanta {
    /// Planta principal, por donde se accede al edificio.
    pub const PRINCIPAL: IdentificadorDePlanta = IdentificadorDePlanta(0);

    /// Numero de plantas que separan esta planta de otra.
    pub fn distancia_a(&self, otra: IdentificadorDePlanta) -> u32 {
        self.0.abs_diff(otra.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn planta(numero: i32) -> IdentificadorDePlanta {
        IdentificadorDePlanta(numero)
    }

    #[test]
    fn distancia_de_una_planta_a_si_misma_es_cero() {
        assert_eq!(planta(3).distancia_a(planta(3)), 0);
    }

    #[test]
    fn distancia_entre_dos_plantas_es_el_valor_absoluto_de_la_diferencia_de_sus_identificadores() {
        assert_eq!(planta(2).distancia_a(planta(7)), 5);
    }

    #[test]
    fn distancia_entre_dos_plantas_es_simetrica() {
        assert_eq!(
            planta(2).distancia_a(planta(7)),
            planta(7).distancia_a(planta(2))
        );
    }

    #[test]
    fn distancia_entre_planta_por_debajo_y_planta_por_encima_de_la_principal_suma_ambos_tramos() {
        assert_eq!(planta(-2).distancia_a(planta(7)), 9);
    }
}
