# Control de tráfico básico: requisitos y casos de uso

Funcionalidad de origen: `trabajo/2_en_curso/Un control de trafico basico.md`.

Se construye sobre el modelo de simulación ya implementado (commit `4b7a4dd`; requisitos archivados en `trabajo/3_historico/20261005T083405/`). Los supuestos S1–S7 de aquella funcionalidad siguen vigentes; en particular S6: un ascensor que se está desplazando rechaza nuevas órdenes.

El enunciado pide tres cosas:
1. Enviar el **ascensor libre más cercano** a la planta donde una persona pulsa el **botón de llamada**.
2. Conservar un **histórico permanente de movimientos**, etiquetados con **día y hora**.
3. Con el histórico del último mes, **optimizar la posición de los ascensores libres** a lo largo del día, según el **día de la semana**: moverlos para agilizar los desplazamientos futuros.

### Por qué esta funcionalidad va antes que la interfaz gráfica

La interfaz gráfica (la otra funcionalidad de `1_listo_para_implementar/`) ha de permitir "pulsar el botón de llamada en una planta". Para eso, alguien tiene que decidir qué ascensor acude, y eso es justo el control de tráfico. Con el control de tráfico hecho, la interfaz será un adaptador fino que solo llama a sus operaciones y dibuja el estado.


## 1. Supuestos adoptados (pendientes de confirmar por el usuario)

Se numeran T1, T2... para no confundirlos con los S1–S7 de la funcionalidad anterior.

| # | Supuesto | Motivo | Alternativa |
|---|----------|--------|-------------|
| T1 | El **control de tráfico es el dueño del simulador** y el único punto de entrada para mover ascensores. Además de las llamadas, ofrece la **orden de la botonera** (pulsar, dentro de un ascensor, el botón de una planta). | Así el histórico recoge todos los movimientos, y la interfaz gráfica solo tendrá que llamar al control. | Dejar la botonera para la funcionalidad de la interfaz gráfica; el histórico no tendría esos movimientos hasta entonces. |
| T2 | Hay **un único botón de llamada por planta**, sin sentido (no hay botones de subir y bajar). | El enunciado habla de "el botón de llamada". | Dos botones por planta (subir y bajar). |
| T3 | **Ascensor libre** = ascensor parado (`EstadoDeAscensor::Parado`). El **más cercano** es el de menor distancia entre su planta actual y la planta de la llamada; **a igual distancia, el de menor identificador**. | Es la regla más simple y determinista. | Desempatar por otro criterio (por ejemplo, el que lleve más tiempo parado). |
| T4 | Si al pulsar el botón de llamada **no hay ningún ascensor libre**, la llamada **queda pendiente** en el control de tráfico. Las llamadas pendientes se atienden **por orden de llegada**, en cuanto queda libre algún ascensor. | No perder llamadas sin tener que implementar todavía la cola de destinos por ascensor (idea en `0_funcionalidades_y_tareas_pendientes/`). | Rechazar la llamada con un error. |
| T5 | **Llamada repetida**: si la planta ya tiene una llamada pendiente, o algún ascensor se está desplazando hacia ella (por el motivo que sea), pulsar su botón de llamada **no tiene efecto**: no se envía otro ascensor ni se registra nada. Si hay un ascensor **parado** en la planta, la llamada se atiende con él sin moverlo. | Es lo que ocurre en un ascensor real cuando el botón ya está encendido. | Enviar otro ascensor igualmente. |
| T6 | Las llamadas pendientes y el reposicionamiento se procesan **al final de cada avance de tiempo**, no en el instante exacto en que un ascensor queda libre. | El simulador calcula las posiciones a demanda y no genera eventos. La interfaz gráfica avanzará el tiempo fotograma a fotograma, así que la diferencia será despreciable. | Partir cada avance en los instantes en que terminan los desplazamientos. |
| T7 | Un **movimiento** (registro del histórico) es una orden dada a un ascensor: fecha y hora, ascensor, planta de origen, planta de destino y **motivo** (`Llamada`, `Botonera` o `Reposicionamiento`). Se registra cada orden aceptada que desplaza un ascensor y, además, **cada llamada atendida aunque el ascensor ya esté en la planta** (con origen igual a destino), porque representa demanda. Una llamada pendiente se registra **cuando se atiende**, con la fecha y hora de ese momento. No se registran las órdenes rechazadas, las llamadas repetidas (T5) ni las órdenes de botonera a la planta en la que ya está parado el ascensor. | La optimización necesita saber dónde y cuándo llama la gente; los demás movimientos se guardan para tener el histórico completo. | Registrar las llamadas y los desplazamientos en dos históricos distintos. |
| T8 | **Fecha y hora** de la simulación = fecha y hora de inicio + instante de simulación. La fecha y hora de inicio la da quien crea el control de tráfico; en `main.rs`, la **hora local del sistema** al arrancar. Es una hora local **sin zona horaria**: los cambios de horario de verano se ignoran. | Mantiene el tiempo simulado del simulador (S2) y los tests deterministas. | Trabajar en UTC. |
| T9 | **Histórico permanente** = archivo de texto con campos separados por `;`, en `datos/historico_de_movimientos.csv`: una cabecera y una línea por movimiento. Solo se añaden líneas al final; nunca se borra nada. La carpeta `datos/` se añade a `.gitignore`. | Es legible, no necesita más dependencias y basta para el volumen de un mes. | JSON con `serde` y `serde_json`, o SQLite con `rusqlite` (más dependencias). |
| T10 | El "**último mes**" son los **28 días** anteriores a la fecha y hora actual (4 semanas completas: cada día de la semana aparece exactamente 4 veces). Es configurable. | Al agrupar por día de la semana, todos los días tienen el mismo número de muestras. | 30 días, o el mes natural anterior. |
| T11 | Una **franja horaria** es una **hora del día** (24 franjas, de 0 a 23). | Es la granularidad más simple que capta los cambios "a lo largo del día". | Franjas de 30 o de 15 minutos. |
| T12 | **Demanda** de una planta = número de movimientos por **llamada** cuya planta de destino (la planta donde se llamó) es esa planta, hechos el **mismo día de la semana** y en la **misma franja horaria** que la fecha y hora actual, dentro de los 28 días de T10. Los movimientos por botonera y por reposicionamiento **no** cuentan. | Las llamadas indican dónde espera la gente al ascensor. | Contar también los destinos de la botonera. |
| T13 | **Plantas de espera preferentes** = las plantas con demanda mayor que 0, de más a menos demanda. A igual demanda, primero la más cercana a la planta principal; si también empatan, la de menor identificador. Se ignoran las plantas que no existen en el edificio. | Orden determinista; la planta principal suele ser la más transitada. | Otros desempates. |
| T14 | Solo se reposicionan los **ascensores en reposo**: los que están parados desde hace al menos el **tiempo de reposo** (**30 s** en la configuración estándar; configurable). Se cuenta desde el final del último desplazamiento ordenado por el control de tráfico, o desde que se le dio la última orden si no tuvo que moverse; al crear el control, desde el instante actual del simulador. | Evita llevarse un ascensor justo cuando acaba de llegar a una llamada, antes de que la persona pulse la botonera. | Reposicionar en cuanto quede libre, o solo cuando se pida de forma explícita. |
| T15 | **Reglas del reposicionamiento** (ver R7.4). Si no hay demanda para la franja actual, **ningún ascensor se mueve**. | Sin datos, no hay nada que optimizar y se evitan movimientos inútiles. | Sin datos, enviar los ascensores en reposo a la planta principal. |
| T16 | Mientras se reposiciona, el ascensor está ocupado (S6): **no atiende llamadas hasta llegar** a la planta de espera. | Consecuencia de S6; no se puede cambiar de destino en marcha. | Permitir cambiar de destino en marcha (otra funcionalidad). |
| T17 | Si el histórico falla al registrar un movimiento, la operación devuelve un **error de histórico**, pero **no deshace** la orden ya dada al ascensor. En `avanzar_tiempo`, se detiene en el primer error. | El ascensor ya ha recibido la orden; deshacerla complicaría el modelo. | Registrar antes de ordenar y, si falla, no ordenar. |
| T18 | El adaptador de archivo **lee el archivo entero una vez, al abrirlo**, y luego responde las consultas desde memoria (y escribe en el archivo y en memoria cada movimiento nuevo). Si el archivo tiene una **línea ilegible**, abrirlo da **error**, indicando el número de línea. | El control de tráfico consulta el histórico en cada avance de tiempo; no conviene leer el disco en cada fotograma. Mejor detectar un archivo dañado que ignorar datos sin avisar. | Ignorar las líneas ilegibles. |


## 2. Dependencias externas (pendientes de confirmar por el usuario)

### `chrono` (propuesta)

```toml
[dependencies]
chrono = { version = "0.4", default-features = false, features = ["clock", "std"] }
```

- **Para qué:** fechas y horas de calendario. Hace falta para el día de la semana, la hora del día, sumar duraciones que cruzan la medianoche, restar 28 días, escribir y leer fechas en formato ISO 8601 en el archivo del histórico, y obtener la hora local del sistema en `main.rs`.
- **Por qué no hacerlo a mano:** la biblioteca estándar no da ni calendario ni la zona horaria local. Hacerlo a mano exige el algoritmo de conversión entre días y fechas civiles, el formateo y el análisis de textos, y trabajar en UTC. Es más código propio que probar, y sin hora local.
- **Cómo se contiene:** solo la usan `src/dominio/fecha_y_hora.rs` (envuelta en el tipo propio `FechaYHora`) y el adaptador del reloj del sistema. El resto del código no ve tipos de `chrono`.
- `chrono` 0.4.45 y sus dependencias (`num-traits`, `iana-time-zone`, `autocfg`) ya están en la caché local de cargo.
- **Alternativa:** `jiff` (más moderno, misma función), o ninguna dependencia, con las limitaciones anteriores.

### No hacen falta

- `serde` y `serde_json`: el formato del archivo (T9) se escribe y se lee a mano.
- `tempfile`: los tests del adaptador de archivo usan rutas únicas dentro de `std::env::temp_dir()`.


## 3. Requisitos funcionales

### R1. Fecha y hora
- R1.1. `FechaYHora` es un valor del dominio: fecha y hora local, con fracciones de segundo. Se puede comparar y ordenar cronológicamente.
- R1.2. Se crea a partir de año, mes, día, hora, minuto y segundo. Si la combinación no existe (30 de febrero, hora 24...), no se obtiene ninguna fecha y hora.
- R1.3. `dia_de_la_semana()` devuelve un `DiaDeLaSemana`: `Lunes`, `Martes`, `Miercoles`, `Jueves`, `Viernes`, `Sabado` o `Domingo`.
- R1.4. `hora_del_dia()` devuelve la hora, de 0 a 23; es la franja horaria (T11).
- R1.5. Sumar una `Duration` avanza la fecha y hora esa duración, cambiando de día si hace falta.
- R1.6. Restar un número de días la retrocede esos días, conservando la hora.
- R1.7. Se escribe como texto ISO 8601 `AAAA-MM-DDTHH:MM:SS`, con la fracción de segundo solo si no es cero (por ejemplo `2026-10-05T08:00:10` o `2026-10-05T08:00:10.500`). Se puede leer de ese mismo texto; leer y escribir da la misma fecha y hora. Un texto mal formado da error.

### R2. Movimiento de ascensor
- R2.1. `MovimientoDeAscensor`: fecha y hora, identificador de ascensor, planta de origen, planta de destino y motivo.
- R2.2. `MotivoDeMovimiento`: `Llamada`, `Botonera` o `Reposicionamiento`.
- R2.3. En un movimiento por llamada, la planta de destino es la planta donde se pulsó el botón de llamada.

### R3. Puerto del histórico de movimientos
- R3.1. Trait `HistoricoDeMovimientos` (puerto de salida; repositorio en términos de DDD):
  - `registrar(movimiento) -> Result<(), ErrorDeHistoricoDeMovimientos>`
  - `movimientos_desde(fecha_y_hora) -> Result<Vec<MovimientoDeAscensor>, ErrorDeHistoricoDeMovimientos>`: los movimientos con fecha y hora igual o posterior a la indicada, en el orden en que se registraron.
- R3.2. `ErrorDeHistoricoDeMovimientos`: `AccesoAlAlmacenamiento { detalle: String }` (por ejemplo, un error de E/S) y `LineaIlegible { numero_de_linea: usize }` (las líneas se cuentan desde 1, incluida la cabecera).

### R4. Adaptadores del histórico
- R4.1. **En memoria** (`HistoricoDeMovimientosEnMemoria`): guarda los movimientos en un `Vec`. Se puede crear vacío o con unos movimientos iniciales (útil en los tests). Ofrece además `todos_los_movimientos()`, para que los tests comprueben lo registrado. No es permanente.
- R4.2. **En archivo** (`HistoricoDeMovimientosEnArchivo`), el histórico permanente (T9, T18):
  - `abrir(ruta)` lee el archivo, si existe. Si no existe, el histórico empieza vacío (no es un error, y todavía no se crea el archivo).
  - `registrar` añade una línea al final del archivo. Si el archivo no existe o está vacío, escribe antes la cabecera. Si la carpeta no existe, la crea.
  - Cabecera: `fecha_y_hora;ascensor;planta_de_origen;planta_de_destino;motivo`.
  - Cada línea: fecha y hora ISO 8601 (R1.7), número del ascensor, planta de origen, planta de destino y motivo en minúsculas (`llamada`, `botonera` o `reposicionamiento`). Por ejemplo: `2026-10-05T08:00:10;1;0;4;llamada`.
  - Lo registrado en una ejecución se recupera igual en la siguiente, y nunca se borra.

### R5. Selección del ascensor libre más cercano
- R5.1. Función pura del dominio: dadas las posiciones de todos los ascensores y la planta de la llamada, devuelve el ascensor libre más cercano (T3), o ninguno si no hay ascensores libres.

### R6. Control de tráfico: llamadas y botonera
- R6.1. `ControlDeTrafico::nuevo(simulador, historico, fecha_y_hora_de_inicio, configuracion)`. Recibe un simulador ya creado (normalmente recién creado) y se queda con él. Empieza sin llamadas pendientes.
- R6.2. `ConfiguracionDelControlDeTrafico`: `dias_de_historico_a_considerar` (28 en la estándar) y `tiempo_de_reposo_antes_de_reposicionar` (30 s en la estándar).
- R6.3. Consultas: `simulador()` (solo lectura), `historico()` (solo lectura), `fecha_y_hora_actual()` (T8) y `llamadas_pendientes()` (por orden de llegada).
- R6.4. `pulsar_boton_de_llamada(planta)`:
  1. Si la planta no existe: error `PlantaInexistente`, y no cambia nada.
  2. Si es una llamada repetida (T5): `LlamadaYaEnCurso`.
  3. Si hay algún ascensor libre: se ordena al más cercano (T3) ir a la planta, se registra el movimiento por llamada (T7) y se devuelve `AscensorAsignado(identificador)`. Si el ascensor ya estaba en la planta, no se mueve, pero el movimiento se registra igual.
  4. Si no: la llamada queda pendiente (T4) y se devuelve `PendienteDeAscensorLibre`.
- R6.5. `pulsar_boton_de_la_botonera(ascensor, planta_de_destino)`: ordena al ascensor ir a la planta. Da los mismos errores, y en el mismo orden, que `SimuladorDeEdificio::ordenar_mover_ascensor` (`AscensorInexistente`, `PlantaInexistente`, `AscensorOcupado`). Si el ascensor se desplaza, se registra el movimiento por botonera.
- R6.6. `avanzar_tiempo(duracion)`, en este orden:
  1. Avanza el tiempo del simulador.
  2. Atiende las llamadas pendientes, por orden de llegada, mientras quede algún ascensor libre: cada una con el más cercano, registrando el movimiento con la fecha y hora actual.
  3. Reposiciona los ascensores en reposo (R7).
  4. Si falla un registro, se detiene y devuelve el error (T17). La llamada cuyo registro falló se da por atendida, porque el ascensor ya ha recibido la orden.
- R6.7. Errores: `ErrorDeControlDeTrafico::Simulacion(ErrorDeSimulacion)` y `ErrorDeControlDeTrafico::Historico(ErrorDeHistoricoDeMovimientos)`, con `From` para poder usar `?`.
- R6.8. Para T14, el control guarda, por cada ascensor, el instante desde el que está libre: al dar una orden en el instante `t` de la planta `O` a la `D`, ese instante es `t + configuracion_del_edificio.duracion_de_un_desplazamiento(distancia(O, D))`, o `t` si la distancia es 0. Coincide con el instante en que el simulador deja el ascensor parado (R6.1 de la funcionalidad anterior).

### R7. Optimización de la posición de los ascensores libres
- R7.1. En cada `avanzar_tiempo`, después de atender las llamadas pendientes, si hay algún ascensor en reposo (T14), el control:
  1. pide al histórico los movimientos desde `fecha_y_hora_actual` menos `dias_de_historico_a_considerar` días;
  2. calcula las plantas de espera preferentes (R7.3);
  3. calcula el plan de reposicionamiento (R7.4);
  4. ordena cada movimiento del plan y lo registra con motivo `Reposicionamiento`.
- R7.2. Ascensores en reposo: parados y libres desde hace al menos `tiempo_de_reposo_antes_de_reposicionar`. Plantas cubiertas por otros ascensores: la planta de destino de cada ascensor que se desplaza, y la planta actual de cada ascensor parado que todavía no está en reposo.
- R7.3. Función pura `plantas_de_espera_preferentes(movimientos, fecha_y_hora_actual, configuracion_del_edificio) -> Vec<IdentificadorDePlanta>`: aplica T12 y T13 con el día de la semana y la franja horaria de `fecha_y_hora_actual`. Cada planta aparece una sola vez. No filtra por los 28 días: eso lo hace la consulta al histórico (R7.1).
- R7.4. Función pura `plan_de_reposicionamiento(plantas_de_espera_preferentes, ascensores_en_reposo, plantas_cubiertas_por_otros_ascensores)`. `ascensores_en_reposo` es una lista de pares (identificador, planta actual). Devuelve las órdenes de reposicionamiento (ascensor y planta de espera), ordenadas por identificador de ascensor:
  1. Se quitan de las plantas preferentes las que ya cubren otros ascensores.
  2. De las restantes, se toman tantas como ascensores en reposo haya, las de más prioridad: son las **plantas a cubrir**.
  3. Cada ascensor en reposo que ya está en una planta a cubrir se queda en ella. Si hay varios en la misma planta, se queda el de menor identificador y los demás siguen disponibles.
  4. Las plantas a cubrir que quedan, por orden de prioridad, se asignan una a una al ascensor en reposo disponible más cercano (a igual distancia, el de menor identificador).
  5. Los ascensores en reposo que no reciben planta se quedan donde están.
  6. Solo se devuelven órdenes que mueven el ascensor; nunca se mandan dos ascensores a la misma planta.

### R8. Raíz de composición (`src/main.rs`) y configuración
- R8.1. `main.rs` crea el simulador estándar, abre el histórico en archivo en `datos/historico_de_movimientos.csv`, obtiene la fecha y hora local del sistema (adaptador del reloj) y crea el control de tráfico con la configuración estándar.
- R8.2. Después ejecuta una breve demostración por consola: unas llamadas y alguna orden de botonera, varios avances de tiempo, y muestra las posiciones de los ascensores, las llamadas pendientes y la fecha y hora actual. Los errores se muestran con `{:?}`. No lleva tests.
- R8.3. Añadir `/datos/` a `.gitignore`: el histórico es un dato generado al ejecutar y no debe ir al control de versiones.
- R8.4. Adaptador del reloj del sistema (por ejemplo `src/adaptadores/reloj_del_sistema.rs`, con `fecha_y_hora_local_actual() -> FechaYHora`). Depende del reloj real, así que no lleva tests. Basta una función: un trait `Reloj` no aporta nada mientras solo se use una vez, al arrancar.


## 4. Casos de uso

- **CU1. Iniciar el control de tráfico.** Se crea con un simulador, un histórico y una fecha y hora de inicio. No hay llamadas pendientes; la fecha y hora actual es la de inicio.
- **CU2. Una persona llama al ascensor desde una planta.** Acude el ascensor libre más cercano y se registra el movimiento. Si no hay ninguno libre, la llamada queda pendiente. Si la planta ya estaba llamada, no pasa nada.
- **CU3. Una persona pulsa un botón de la botonera.** El ascensor va a esa planta y se registra el movimiento. Si el ascensor está ocupado, se rechaza la orden.
- **CU4. Pasa el tiempo.** Avanza la simulación, se atienden las llamadas pendientes que se pueda y se reposicionan los ascensores en reposo.
- **CU5. Se reposicionan los ascensores en reposo.** Según las llamadas de las últimas 4 semanas, el mismo día de la semana y en la misma franja horaria, los ascensores en reposo se reparten por las plantas con más demanda.
- **CU6. Se reanuda en otra ejecución.** Al abrir de nuevo el histórico en archivo, se recuperan los movimientos de las ejecuciones anteriores, que siguen sirviendo para la optimización.


## 5. Ejemplos de referencia para los tests

Edificio estándar (plantas de la -2 a la 7; 2 s por planta; 4 s de arranque y de parada). Duración de un desplazamiento de `n` plantas: `2·n + 4` s. Por ejemplo, de la 0 a la 1: 6 s; a la 2: 8 s; a la 3: 10 s; a la 4: 12 s; a la 5: 14 s; a la 7: 18 s.

Fechas útiles (2026):
- El lunes 5 de octubre de 2026 es la fecha de inicio sugerida para los tests del control (por ejemplo, a las 08:00:00).
- Lunes anteriores: 28 de septiembre (hace 7 días), 7 de septiembre (hace 28 días; con la hora 08:00:00, el periodo de T10 empieza justo ahí) y 31 de agosto (hace 35 días; queda fuera).
- Martes 29 de septiembre; miércoles 7 de octubre; sábado 10 de octubre; domingo 11 de octubre; lunes 12 de octubre.

Escenarios:
- **Llamada pendiente.** En el instante 0, botonera: ascensor 1 a la 1 (libre a los 6 s), ascensor 2 a la 5 (a los 14 s) y ascensor 3 a la 7 (a los 18 s). Llamada en la 2: queda pendiente. A los 5 s sigue pendiente. A los 6 s se asigna al ascensor 1, que va de la 1 a la 2, y se registra con fecha y hora 08:00:06.
- **Tiempo de reposo.** Edificio de un solo ascensor y un histórico con llamadas en la 5 el lunes anterior a las 08:30. Inicio el lunes a las 08:00:00. A los 29 s el ascensor sigue parado en la 0; a los 30 s se está desplazando hacia la 5.
- **Reposo contado desde el final del desplazamiento.** Igual que el anterior, pero en el instante 0 la botonera lo manda a la 2 (llega a los 8 s). A los 37 s sigue parado en la 2; a los 38 s se está desplazando hacia la 5.
- **Cambio de franja.** Edificio de un ascensor. Histórico: el lunes anterior, una llamada en la 5 a las 08:30 y otra en la -1 a las 09:30. Inicio el lunes a las 08:59:00. A los 30 s (08:59:30) va hacia la 5, adonde llega a los 44 s. A los 75 s (09:00:15) ya lleva 31 s en reposo y está en la franja de las 9: va hacia la -1.
- **Reparto entre varios ascensores.** Edificio estándar. Histórico del lunes anterior en la franja de las 8: dos llamadas en la 5 y una en la 2. Además, para comprobar que no cuentan: tres llamadas en la 3 a las 09:30, y tres en la 6 el martes a las 08:30. A los 30 s: el ascensor 1 va hacia la 5, el ascensor 2 hacia la 2, y el 3 sigue parado en la 0.


## 6. Propuesta de diseño (orientativa)

El programador puede ajustar los nombres. Si lo hace, mantendrá el significado.

### Organización del crate (arquitectura hexagonal)

```
src/
  lib.rs                        pub mod dominio; pub mod aplicacion; pub mod adaptadores;
  dominio.rs                    añadir los módulos nuevos
  dominio/fecha_y_hora.rs       FechaYHora, DiaDeLaSemana (único módulo del dominio que usa chrono)
  dominio/movimiento.rs         MovimientoDeAscensor, MotivoDeMovimiento
  dominio/historico_de_movimientos.rs   puerto: trait HistoricoDeMovimientos, ErrorDeHistoricoDeMovimientos
  dominio/seleccion_de_ascensor.rs      ascensor_libre_mas_cercano(...)
  dominio/reposicionamiento.rs          plantas_de_espera_preferentes(...), plan_de_reposicionamiento(...), OrdenDeReposicionamiento
  aplicacion.rs
  aplicacion/control_de_trafico.rs      ControlDeTrafico<H: HistoricoDeMovimientos>, ConfiguracionDelControlDeTrafico,
                                        ResultadoDeLaLlamada, ErrorDeControlDeTrafico
  adaptadores.rs
  adaptadores/historico_de_movimientos_en_memoria.rs
  adaptadores/historico_de_movimientos_en_archivo.rs
  adaptadores/reloj_del_sistema.rs
  main.rs                       raíz de composición
```

- **Dominio:** valores (`FechaYHora`, `MovimientoDeAscensor`), el puerto del histórico (repositorio) y los servicios de dominio puros (selección y reposicionamiento). No hace E/S.
- **Aplicación:** `ControlDeTrafico` orquesta el simulador, el histórico (a través del puerto) y los servicios de dominio. Es el puerto de entrada que usará la interfaz gráfica.
- **Adaptadores:** histórico en memoria, histórico en archivo y reloj del sistema.
- **SOLID:** el control depende de la abstracción `HistoricoDeMovimientos`, no del archivo (inversión de dependencias). Las reglas de selección y de reposicionamiento están en funciones puras, separadas de la orquestación (responsabilidad única).
- El código existente del simulador **no necesita cambios**: el control usa su API pública (`ordenar_mover_ascensor`, `posiciones_de_todos_los_ascensores`, `avanzar_tiempo`, `instante_actual`, `configuracion`).

### Tipos y firmas sugeridos

```rust
// dominio/fecha_y_hora.rs
pub struct FechaYHora(/* chrono::NaiveDateTime */);
impl FechaYHora {
    pub fn nueva(anio: i32, mes: u32, dia: u32, hora: u32, minuto: u32, segundo: u32) -> Option<FechaYHora>;
    pub fn dia_de_la_semana(&self) -> DiaDeLaSemana;
    pub fn hora_del_dia(&self) -> u32;
    pub fn sumar(&self, duracion: Duration) -> FechaYHora;
    pub fn restar_dias(&self, dias: u32) -> FechaYHora;
}
// Display y FromStr con el formato de R1.7.
pub enum DiaDeLaSemana { Lunes, Martes, Miercoles, Jueves, Viernes, Sabado, Domingo }

// dominio/movimiento.rs
pub struct MovimientoDeAscensor {
    pub fecha_y_hora: FechaYHora,
    pub identificador_de_ascensor: IdentificadorDeAscensor,
    pub planta_de_origen: IdentificadorDePlanta,
    pub planta_de_destino: IdentificadorDePlanta,
    pub motivo: MotivoDeMovimiento,
}
pub enum MotivoDeMovimiento { Llamada, Botonera, Reposicionamiento }

// dominio/seleccion_de_ascensor.rs
pub fn ascensor_libre_mas_cercano(
    posiciones: &[(IdentificadorDeAscensor, PosicionDeAscensor)],
    planta_de_la_llamada: IdentificadorDePlanta,
) -> Option<IdentificadorDeAscensor>;

// dominio/reposicionamiento.rs
pub fn plantas_de_espera_preferentes(
    movimientos: &[MovimientoDeAscensor],
    fecha_y_hora_actual: FechaYHora,
    configuracion_del_edificio: &ConfiguracionDelEdificio,
) -> Vec<IdentificadorDePlanta>;
pub struct OrdenDeReposicionamiento {
    pub identificador_de_ascensor: IdentificadorDeAscensor,
    pub planta_de_espera: IdentificadorDePlanta,
}
pub fn plan_de_reposicionamiento(
    plantas_de_espera_preferentes: &[IdentificadorDePlanta],
    ascensores_en_reposo: &[(IdentificadorDeAscensor, IdentificadorDePlanta)],
    plantas_cubiertas_por_otros_ascensores: &[IdentificadorDePlanta],
) -> Vec<OrdenDeReposicionamiento>;

// aplicacion/control_de_trafico.rs
pub enum ResultadoDeLaLlamada {
    AscensorAsignado(IdentificadorDeAscensor),
    PendienteDeAscensorLibre,
    LlamadaYaEnCurso,
}
impl<H: HistoricoDeMovimientos> ControlDeTrafico<H> {
    pub fn nuevo(simulador: SimuladorDeEdificio, historico: H, fecha_y_hora_de_inicio: FechaYHora,
                 configuracion: ConfiguracionDelControlDeTrafico) -> Self;
    pub fn simulador(&self) -> &SimuladorDeEdificio;
    pub fn historico(&self) -> &H;
    pub fn fecha_y_hora_actual(&self) -> FechaYHora;
    pub fn llamadas_pendientes(&self) -> Vec<IdentificadorDePlanta>;
    pub fn pulsar_boton_de_llamada(&mut self, planta: IdentificadorDePlanta)
        -> Result<ResultadoDeLaLlamada, ErrorDeControlDeTrafico>;
    pub fn pulsar_boton_de_la_botonera(&mut self, identificador_de_ascensor: IdentificadorDeAscensor,
                                       planta_de_destino: IdentificadorDePlanta)
        -> Result<(), ErrorDeControlDeTrafico>;
    pub fn avanzar_tiempo(&mut self, duracion: Duration) -> Result<(), ErrorDeControlDeTrafico>;
}
```

### Indicaciones para los tests
- Tests unitarios en un módulo `#[cfg(test)] mod tests` dentro de cada archivo, como en la funcionalidad anterior.
- Los tests del control usan `HistoricoDeMovimientosEnMemoria` y comprueban lo registrado con `control.historico().todos_los_movimientos()`.
- Para el error de histórico, un doble de test dentro del módulo de tests del control (por ejemplo `HistoricoQueFallaAlRegistrar`).
- Para aislar el reposicionamiento, conviene un edificio de 1 o 2 ascensores (`ConfiguracionDelEdificio { numero_de_ascensores: 1, ..ConfiguracionDelEdificio::estandar() }`) y, si hace falta, un tiempo de reposo de 0.
- Los tests del adaptador de archivo usan una ruta única por test dentro de `std::env::temp_dir()` (por ejemplo, con el nombre del test y `std::process::id()`); la borran al empezar y al acabar. Los tests se ejecutan en paralelo, así que dos tests no deben compartir ruta.
- No hay que modificar los tests existentes.

### Orden de implementación sugerido
1. `FechaYHora` (y añadir `chrono` a `Cargo.toml`).
2. Movimiento, puerto del histórico y adaptador en memoria.
3. Selección del ascensor libre más cercano.
4. Control de tráfico: llamadas, botonera, llamadas pendientes y registro en el histórico.
5. Adaptador de archivo.
6. Plantas de espera preferentes, plan de reposicionamiento e integración en `avanzar_tiempo`.
7. `main.rs`, reloj del sistema, `.gitignore` y glosario.

### Glosario
Cuando la implementación esté hecha, añadir a `src/glosario_de_dominio.md` (en una sección "Para el control de tráfico"):
- **control de tráfico**: decide qué ascensor atiende cada llamada, registra los movimientos y reposiciona los ascensores libres.
- **botón de llamada**: botón de una planta con el que una persona pide un ascensor; uno por planta.
- **llamada**: pulsación de un botón de llamada. **Llamada pendiente**: la que espera a que quede libre algún ascensor.
- **botonera**: botones de dentro de un ascensor, uno por planta, para elegir la planta de destino.
- **ascensor libre**: ascensor parado.
- **ascensor en reposo**: ascensor libre desde hace al menos el tiempo de reposo.
- **tiempo de reposo**: tiempo que un ascensor ha de estar libre antes de que se le pueda reposicionar.
- **movimiento**: registro de una orden dada a un ascensor: fecha y hora, ascensor, plantas de origen y de destino, y motivo.
- **motivo** (de un movimiento): llamada, botonera o reposicionamiento.
- **histórico de movimientos**: registro permanente de todos los movimientos.
- **fecha y hora**: fecha y hora local de calendario; en la simulación, la de inicio más el instante de simulación.
- **franja horaria**: cada una de las 24 horas del día.
- **demanda** (de una planta): número de llamadas hechas en ella el mismo día de la semana y en la misma franja horaria, en las últimas 4 semanas.
- **planta de espera**: planta a la que se manda un ascensor en reposo para esperar llamadas. **Plantas de espera preferentes**: las de más demanda.
- **reposicionamiento**: movimiento de un ascensor en reposo hacia una planta de espera.


## 7. Fuera del alcance de esta funcionalidad

- La interfaz gráfica (siguiente funcionalidad de la cola).
- Cola de destinos por ascensor y cambio de destino en marcha (idea en `0_funcionalidades_y_tareas_pendientes/`). Tampoco se recoge a nadie "de paso": solo se asignan llamadas a ascensores libres.
- Botones de llamada con sentido (subir y bajar).
- Personas, puertas, capacidad y tiempos de embarque.
- Mensajes legibles para los errores e implementación de `std::error::Error` (idea en `0_funcionalidades_y_tareas_pendientes/`).
- El tipo `InstanteDeSimulacion` (idea en `0_funcionalidades_y_tareas_pendientes/`): cambia la API pública y obligaría a modificar tests existentes. El control usa el `Duration` que devuelve el simulador; si más adelante se introduce, solo cambiará la conversión a fecha y hora.
- Purgar o rotar el histórico, y varios procesos escribiendo a la vez en el mismo archivo.
- Zonas horarias y cambios de horario de verano.
- Un caso límite: si se llama a una planta y, mientras la llamada está pendiente, la botonera manda otro ascensor a esa misma planta, la llamada pendiente se atiende igualmente con el siguiente ascensor libre.
