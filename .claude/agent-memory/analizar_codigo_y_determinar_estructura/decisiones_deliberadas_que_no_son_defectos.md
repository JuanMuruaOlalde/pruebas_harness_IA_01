---
name: decisiones-deliberadas-que-no-son-defectos
description: Decisiones de diseno del simulador aprobadas a proposito (chrono en el dominio, vista egui sin tests, control de trafico en aplicacion, Ascensor pub(crate)); matizarlas en vez de reportarlas como fallos
metadata:
  type: reference
---

Fuente con el detalle y el porque: `.claude/agent-memory/revisar_codigo_y_refactorizar/decisiones_arquitectura_dominio.md` (memoria del agente revisor) y los requisitos de cada vuelta en `trabajo/3_historico/<timestamp>/lista_de_requisitos_y_casos_de_uso.md` (supuestos con codigos U*, T*, R*).

Decisiones deliberadas vistas el 2026-10-06 (verificar que siguen vigentes):
- `chrono` solo en `dominio/fecha_y_hora.rs` y `adaptadores/reloj_del_sistema.rs`: lo aprobo el usuario al confirmar la dependencia.
- `vista_con_egui.rs` y `main.rs` sin tests automaticos (supuesto U14); la verificacion visual es manual.
- `Ascensor` es `pub(crate)`; `SimuladorDeEdificio` es la raiz del agregado.
- `ControlDeTrafico<H>` esta en `aplicacion` (aunque el glosario lo trate como concepto de dominio): decision de la vuelta 2.
- Tope de 1 s de avance por fotograma en el presentador (supuesto U4).
- Las mejoras que cambian la API publica del simulador se aplazan a pendientes en lugar de hacerse al refactorizar.

**How to apply:** en los informes, presentarlas como "decision deliberada, con estos costes" y no como incumplimientos sin mas. Relacionado: [[como-analizar-este-repositorio]].
