---
name: decisiones-arquitectura-dominio
description: Decisiones DDD/hexagonales del simulador de ascensores (agregado SimuladorDeEdificio, Ascensor pub(crate), control de trafico en aplicacion, puerto del historico, chrono aislado, interfaz MVP con eframe aislado) y su porque
metadata:
  type: project
---

Decisiones tomadas al refactorizar la primera funcionalidad (2026-10-05, "Estructura basica del edificio y ascensores"):

- `SimuladorDeEdificio` es la raiz del agregado; `Ascensor` es `pub(crate)` para que nadie fuera del crate lo modifique saltandose la raiz.
  **Why:** la propuesta de diseno lo define como raiz del agregado; exponer `Ascensor` permitia ordenes sin validar planta/existencia.
  **How to apply:** si una funcionalidad nueva necesita datos del ascensor, exponerlos via el simulador (p. ej. `PosicionDeAscensor`), no haciendo `Ascensor` publico.
- `Ascensor` valida el mismo que esta libre (`AscensorOcupado`); el simulador solo valida existencia del ascensor y de la planta. Orden de errores (R5.2): inexistente -> planta -> ocupado.
- El calculo temporal vive en el objeto valor privado `Desplazamiento` (en `ascensor.rs`); posicion calculada bajo demanda, sin `f64`.
- Los tests van en el modulo del tipo que ejercitan (los `crear_simulador_*` estan en `simulador.rs`).

Segunda funcionalidad (2026-10-05, "Un control de trafico basico"), capas `dominio` / `aplicacion` / `adaptadores`:

- `ControlDeTrafico<H: HistoricoDeMovimientos>` (aplicacion) es dueno del simulador y unico punto de entrada para mover ascensores; el puerto `HistoricoDeMovimientos` esta en el dominio. Seleccion y reposicionamiento son funciones puras del dominio.
- `chrono` solo se usa en `dominio/fecha_y_hora.rs` (envuelto en `FechaYHora`) y en `adaptadores/reloj_del_sistema.rs`.
  **Why:** aprobado asi por el usuario al confirmar la dependencia; el resto del codigo no debe ver tipos de chrono.
- En la refactorizacion, el adaptador de archivo pasa a componer `HistoricoDeMovimientosEnMemoria` como copia en memoria, para no duplicar `movimientos_desde` (T18: lee todo al abrir y consulta desde memoria).
- El control duplica el calculo de "cuando queda libre" que ya hace el simulador. Esta anotado en pendientes y no se cambio porque altera la API del simulador.
- En el dominio, `posicion` significa `PosicionDeAscensor`: no usarlo como nombre de indice de un `Vec` (se usa `indice_...`).

Tercera funcionalidad (2026-10-05, "Interfaz de usuario para manejar el simulador", egui/eframe 0.36):

- MVP de vista pasiva en `adaptadores/interfaz_grafica/`: `modelo_de_vista` (datos puros), `presentador` (logica, con tests) y `vista_con_egui` (unico archivo con eframe, sin tests por decision U14).
  **How to apply:** cualquier decision de que se muestra va al presentador; en la vista solo el como (simbolos, widgets, titulos). Nunca ejecutar `cargo run` (abre ventana y bloquea).
- Errores con `Display` en espanol (minuscula, sin punto) y `std::error::Error`; `ErrorDeControlDeTrafico` delega con `fmt::Display::fmt` explicito y deja `source()` en `None` para no repetir el mensaje.
- Un solo `#[cfg(test)] mod tests` por archivo: se unio un `mod tests_de_mensajes_de_error` aparte que habia creado el programador.

Ideas aplazadas en `trabajo/0_funcionalidades_y_tareas_pendientes/`: `InstanteDeSimulacion`, robustez del archivo del historico, simulador que diga cuando queda libre un ascensor, tests que faltan en el control, `Display` para los identificadores de planta y de ascensor. Relacionado: [[flujo-paso-refactorizar]]
