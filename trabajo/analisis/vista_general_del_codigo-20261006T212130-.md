# Vista general del código: simulador de ascensores

- Fecha del análisis: 2026-10-06 21:21:30
- Rama: `pruebas_con_el_bucle_automatico`. Último commit: `60344c8`.
- Agente: `analizar_codigo_y_determinar_estructura`. No ha modificado código.
- Fuera del análisis, tal como manda `CLAUDE.md`: `documentacion/` y `zz - trozos de codigo descartados - guardados por si acaso/`.

## 0. Estado del código en el momento del análisis

| Comprobación | Resultado |
|---|---|
| `cargo build` | Compila sin errores |
| `cargo test` | **171 tests, todos pasan** (0 en `main.rs` y 0 doc-tests) |
| `cargo clippy --all-targets` | **0 warnings** |
| `cargo fmt --check` | Sin diferencias |
| `cargo clippy -- -W clippy::pedantic` (solo informativo) | 42 avisos: 29 de `#[must_use]`, 11 por falta de sección `# Errors` en la documentación, 1 de `match` por `if let` y 1 de `map().unwrap_or()` |

Tamaño: unas 2.000 líneas de producción y unas 2.850 de tests, que van en los propios archivos (`#[cfg(test)] mod tests`).

Tests por módulo: simulador 38, reposicionamiento 20, fecha_y_hora 11, seleccion_de_ascensor 5, configuracion 4, planta 4, errores 2, historico_de_movimientos 1, control_de_trafico 35, presentador 41, histórico en archivo 7, histórico en memoria 3.

---

## 1. Estructura física

### 1.1 Raíz del repositorio

| Carpeta / archivo | Contenido y propósito |
|---|---|
| `Cargo.toml` / `Cargo.lock` | Crate único `pruebas_harness_01` (edición 2024). Dependencias: `chrono 0.4` (solo `clock` y `std`) y `eframe 0.36` (`glow`, `wayland`, `x11`, `default_fonts`). Biblioteca (`src/lib.rs`) más binario (`src/main.rs`). |
| `CLAUDE.md` | Guía del proyecto para los agentes: propósito, comandos, arquitectura (hexagonal, SOLID, DDD), flujo de trabajo, límites y estilo. **La sección "Mapa de la arquitectura actual" está vacía.** |
| `README.md` | Explica que el repositorio es un experimento de "arneses" (directrices) para agentes IA con Claude Code. El simulador de ascensores es el caso práctico. |
| `.gitignore` | Ignora `/target`, `.claude/settings.local.json`, `CLAUDE.local.md` y `/datos/`. |
| `src/` | Código fuente (ver 1.2) y `glosario_de_dominio.md`. |
| `datos/` | Datos que genera la ejecución: `historico_de_movimientos.csv`, el histórico permanente. Git lo ignora. |
| `trabajo/` | Flujo de trabajo de los agentes (ver 1.3). |
| `.claude/` | Configuración de Claude Code: `agents/` (6 agentes), `skills/programacion_en_bucle_automatico/SKILL.md`, `settings.json` (permisos: `git push` prohibido, `git commit` con confirmación), `agent-memory/` (memoria de los agentes) y `workflows/` (vacía, solo `.gitkeep`). |
| `documentacion/` | Apuntes sobre cómo configurar directrices para Claude. No se ha leído, por la restricción de `CLAUDE.md`. |
| `zz - trozos de codigo descartados - guardados por si acaso/` | Se ha ignorado, por la restricción de `CLAUDE.md`. |
| `target/` | Artefactos de compilación de cargo. |

### 1.2 `src/`: capas de la arquitectura hexagonal

```
src/
├── main.rs                        raíz de composición (41 líneas)
├── lib.rs                         declara las tres capas
├── glosario_de_dominio.md         lenguaje ubicuo (DDD)
├── dominio.rs + dominio/          núcleo: modelo, reglas puras y puerto del histórico
│   ├── planta.rs                  IdentificadorDePlanta (objeto valor) y distancia
│   ├── ascensor.rs                IdentificadorDeAscensor, EstadoDeAscensor, PosicionDeAscensor,
│   │                              Ascensor (entidad, pub(crate)), Desplazamiento (objeto valor privado)
│   ├── configuracion.rs           ConfiguracionDelEdificio (estándar, validación, duración de un desplazamiento)
│   ├── simulador.rs               SimuladorDeEdificio: raíz del agregado
│   ├── errores.rs                 ErrorDeConfiguracion y ErrorDeSimulacion (con Display en español)
│   ├── fecha_y_hora.rs            FechaYHora y DiaDeLaSemana: envuelven chrono
│   ├── movimiento.rs              MovimientoDeAscensor y MotivoDeMovimiento
│   ├── historico_de_movimientos.rs   PUERTO: trait HistoricoDeMovimientos y su error
│   ├── seleccion_de_ascensor.rs   regla pura: ascensor libre más cercano
│   └── reposicionamiento.rs       reglas puras: plantas de espera preferentes y plan de reposicionamiento
├── aplicacion.rs + aplicacion/
│   └── control_de_trafico.rs      ControlDeTrafico<H>: caso de uso que orquesta el dominio y el puerto
└── adaptadores.rs + adaptadores/
    ├── historico_de_movimientos_en_memoria.rs   adaptador secundario (tests y copia en memoria)
    ├── historico_de_movimientos_en_archivo.rs   adaptador secundario permanente (CSV con ';')
    ├── reloj_del_sistema.rs                      lee la hora local (chrono::Local)
    ├── interfaz_grafica.rs + interfaz_grafica/  adaptador primario: patrón MVP de vista pasiva
    │   ├── modelo_de_vista.rs     datos puros de lo que se dibuja y AccionDelUsuario
    │   ├── presentador.rs         PresentadorDelSimulador<H>: lógica de presentación, con tests
    │   └── vista_con_egui.rs      el único archivo que usa eframe/egui, sin tests (decisión U14)
```

### 1.3 `trabajo/`: flujo de trabajo de los agentes

| Carpeta | Contenido actual |
|---|---|
| `0_funcionalidades_y_tareas_pendientes/` | 9 ideas o tareas detectadas por los agentes (ver 5.0). Una ya está resuelta: "Mensajes legibles para los errores del dominio". |
| `1_listo_para_implementar/` | Vacía, sin `.gitkeep`. |
| `2_en_curso/` | Vacía, sin `.gitkeep`. |
| `3_historico/` | 3 vueltas del bucle (`20261005T083405`, `20261005T091709` y `20261005T095234`), con sus requisitos y listas de tests, y el informe `realizado_20261005T095750.md`. |
| `directrices/` | Reglas para los commits: no subir secretos, ni archivos de más de 10 MB, ni binarios ejecutables. |
| `analisis/` | Creada en este análisis para guardar este informe. |

---

## 2. Estructura lógica

### 2.1 Dominio (`src/dominio/`)

**Objetos valor**
- `IdentificadorDePlanta(pub i32)`, con la constante `PRINCIPAL` (0) y `distancia_a`.
- `IdentificadorDeAscensor(pub u32)`: se numeran desde 1, pero el tipo no lo impone.
- `ConfiguracionDelEdificio`: planta más baja y más alta, número de ascensores, tiempo de desplazamiento y tiempo de arranque y de parada. Incluye `estandar()`, `validar()`, `contiene_la_planta()` y `duracion_de_un_desplazamiento()`.
- `PosicionDeAscensor { planta_actual, estado: EstadoDeAscensor }`, donde `EstadoDeAscensor` es `Parado` o `Desplazandose { planta_de_destino }`.
- `FechaYHora` (envuelve `chrono::NaiveDateTime`), `DiaDeLaSemana` y `ErrorDeFormatoDeFechaYHora`.
- `MovimientoDeAscensor { fecha_y_hora, identificador_de_ascensor, planta_de_origen, planta_de_destino, motivo }`, con `MotivoDeMovimiento` = `Llamada`, `Botonera` o `Reposicionamiento`.
- `OrdenDeReposicionamiento { identificador_de_ascensor, planta_de_espera }`.
- `Desplazamiento` (privado): planta de origen, planta de destino e instante de inicio. Contiene el modelo temporal.

**Entidad y agregado**
- `Ascensor` (`pub(crate)`): identificador y `EstadoInterno` (`Parado { planta }` o `Desplazandose(Desplazamiento)`). Valida él mismo que no está ocupado.
- `SimuladorDeEdificio`: **raíz del agregado**. Contiene la configuración, los `Vec<Ascensor>` y el `instante_actual: Duration`. Es la única vía para dar órdenes a los ascensores: `ordenar_mover_ascensor`, `avanzar_tiempo`, `posicion_del_ascensor` y `posiciones_de_todos_los_ascensores`.

**Servicios de dominio (funciones puras)**
- `seleccion_de_ascensor::ascensor_libre_mas_cercano(posiciones, planta)`.
- `reposicionamiento::plantas_de_espera_preferentes(movimientos, ahora, configuracion)`: calcula la demanda por planta (mismo día de la semana y misma franja horaria) y desempata por cercanía a la planta principal y, después, por menor identificador.
- `reposicionamiento::plan_de_reposicionamiento(preferentes, en_reposo, cubiertas)`: los ascensores que ya están en una planta a cubrir se quedan en ella. Las plantas restantes, por orden de prioridad, van al ascensor disponible más cercano.

**Puerto (secundario / *driven*)**
- `trait HistoricoDeMovimientos { registrar(movimiento); movimientos_desde(fecha_y_hora) }`, con `ErrorDeHistoricoDeMovimientos` = `AccesoAlAlmacenamiento { detalle }` o `LineaIlegible { numero_de_linea }`.

**Errores**
- `ErrorDeConfiguracion` y `ErrorDeSimulacion` (en `errores.rs`), y `ErrorDeHistoricoDeMovimientos` (en el módulo del puerto). Todos implementan `Display` en español y `std::error::Error`.

### 2.2 Aplicación (`src/aplicacion/control_de_trafico.rs`)

- `ConfiguracionDelControlDeTrafico { dias_de_historico_a_considerar: 28, tiempo_de_reposo_antes_de_reposicionar: 30 s }`.
- `ControlDeTrafico<H: HistoricoDeMovimientos>`: es el dueño del simulador y del histórico. Guarda la fecha y hora de inicio, la cola `VecDeque<IdentificadorDePlanta>` de llamadas pendientes y un `HashMap` que anota, para cada ascensor, desde cuándo está libre.
- `ResultadoDeLaLlamada`: `AscensorAsignado(id)`, `PendienteDeAscensorLibre` o `LlamadaYaEnCurso`.
- `ErrorDeControlDeTrafico`: `Simulacion(..)` o `Historico(..)`, con `From` para usar `?`.

### 2.3 Adaptadores (`src/adaptadores/`)

- `HistoricoDeMovimientosEnMemoria`: un `Vec` con `nuevo`, `con_movimientos` y `todos_los_movimientos`. Sirve de doble en los tests y de copia en memoria dentro del adaptador de archivo.
- `HistoricoDeMovimientosEnArchivo`: compone el histórico en memoria. Al abrir, lee el archivo entero. Al registrar, añade una línea al final del archivo y después a la copia en memoria. El formato es la cabecera `fecha_y_hora;ascensor;planta_de_origen;planta_de_destino;motivo` seguida de una línea por movimiento.
- `reloj_del_sistema::fecha_y_hora_local_actual()`: una función libre, no un puerto.
- Interfaz gráfica, con el patrón MVP de vista pasiva:
  - `ModeloDeVista`, con `PlantaEnLaVista`, `CeldaDelHueco`, `MarchaDelAscensor` (`Parado`, `Subiendo`, `Bajando` o `Llegando`), `AscensorEnLaVista`, `BotonDeLaBotonera`, `MensajeParaElUsuario` y `AccionDelUsuario`.
  - `PresentadorDelSimulador<H>`: es el dueño del `ControlDeTrafico<H>`. Construye el modelo de vista, atiende las acciones del usuario y avanza el tiempo con un tope de 1 s por fotograma.
  - `InterfazGraficaConEgui<H>`: implementa `eframe::App`. Mide el tiempo real, llama al presentador y dibuja con `egui`.

### 2.4 Grafo de dependencias entre módulos

```
main.rs ──► adaptadores::{historico_en_archivo, reloj_del_sistema, interfaz_grafica}
        ──► aplicacion::control_de_trafico
        ──► dominio::{configuracion, simulador}

adaptadores::interfaz_grafica::vista_con_egui ──► presentador ──► modelo_de_vista
                         │                          │
                         │                          ├──► aplicacion::ControlDeTrafico (tipo concreto)
                         │                          └──► dominio::{ascensor, configuracion, fecha_y_hora,
                         │                                         historico (trait), planta}
                         └──► dominio::historico_de_movimientos (solo por la cota genérica H)

adaptadores::historico_en_archivo ──► historico_en_memoria ──► dominio::{historico (trait), movimiento, fecha_y_hora}
adaptadores::reloj_del_sistema ──► dominio::fecha_y_hora (constructor pub(crate) con chrono)

aplicacion::control_de_trafico ──► dominio::{simulador, ascensor, planta, movimiento, fecha_y_hora,
                                             historico (trait), seleccion_de_ascensor, reposicionamiento, errores}
                        (tests) ──► adaptadores::historico_en_memoria      ◄── dependencia hacia fuera, solo en tests

dominio::simulador ──► ascensor ──► configuracion ──► planta, errores
dominio::reposicionamiento ──► ascensor, configuracion, fecha_y_hora, movimiento, planta
dominio::seleccion_de_ascensor ──► ascensor, planta
dominio::historico_de_movimientos ──► fecha_y_hora, movimiento
```

- El dominio no depende de `aplicacion` ni de `adaptadores` (comprobado con `grep`).
- `chrono` solo aparece en `dominio/fecha_y_hora.rs` y en `adaptadores/reloj_del_sistema.rs`. `eframe` y `egui` solo aparecen en `adaptadores/interfaz_grafica/vista_con_egui.rs`.

---

## 3. Funciones y flujos de ejecución principales

### 3.1 Arranque (`main.rs`)
1. `SimuladorDeEdificio::nuevo(ConfiguracionDelEdificio::estandar())` valida la configuración y coloca los 3 ascensores parados en la planta 0.
2. `HistoricoDeMovimientosEnArchivo::abrir("datos/historico_de_movimientos.csv")` lee el archivo entero. Si el archivo no existe, el histórico empieza vacío. Si alguna línea es ilegible, da error y **el programa no arranca**.
3. `ControlDeTrafico::nuevo(simulador, historico, fecha_y_hora_local_actual(), ConfiguracionDelControlDeTrafico::estandar())`.
4. `ejecutar_interfaz_grafica(PresentadorDelSimulador::nuevo(control))` llama a `eframe::run_native` y no vuelve hasta que se cierra la ventana.
5. Si algo falla, se escribe `No se ha podido iniciar el simulador: {error}` y se sale con `ExitCode::FAILURE`.

### 3.2 Bucle de cada fotograma (`InterfazGraficaConEgui::ui`)
1. Mide el tiempo real transcurrido desde el fotograma anterior.
2. `presentador.pasar_el_tiempo(Δ)` limita el avance a `min(Δ, 1 s)` y llama a `control.avanzar_tiempo`. Esta llamada:
   1. ejecuta `simulador.avanzar_tiempo(Δ)`, que solo suma el instante, porque las posiciones se calculan al consultarlas;
   2. ejecuta `atender_llamadas_pendientes()`: por orden de llegada, mientras quede algún ascensor libre, da una orden con motivo `Llamada`;
   3. ejecuta `reposicionar_ascensores_en_reposo()` (ver 3.5).
3. `presentador.modelo_de_vista()` construye las plantas de la más alta a la más baja, los ascensores con su marcha, la botonera, las celdas de los huecos, los botones de llamada encendidos, el texto de la fecha y hora y el de las llamadas pendientes.
4. `dibujar(ui, &modelo)` pinta los paneles superior, inferior y derecho (botoneras) y el central (rejilla), y devuelve las `AccionDelUsuario`.
5. Pasa cada acción a `presentador.atender(accion)` y pide un repintado cada 100 ms.

### 3.3 Pulsar el botón de llamada (`ControlDeTrafico::pulsar_boton_de_llamada`)
1. Si la planta no existe, devuelve `Err(Simulacion(PlantaInexistente))`.
2. Si la planta ya tiene una llamada pendiente, o algún ascensor se desplaza hacia ella por cualquier motivo, devuelve `LlamadaYaEnCurso`.
3. Si hay algún ascensor libre, escoge el más cercano y ejecuta `dar_orden(.., Llamada)`. Devuelve `AscensorAsignado`.
4. Si no hay ninguno libre, encola la llamada y devuelve `PendienteDeAscensorLibre`.

`dar_orden` hace lo siguiente:
1. lee la planta de origen;
2. ejecuta `simulador.ordenar_mover_ascensor`;
3. ejecuta `anotar_cuando_quedara_libre`;
4. registra el movimiento en el histórico, salvo que la orden sea a la misma planta y no sea una llamada.

Si el histórico falla, la orden ya dada al ascensor no se deshace.

### 3.4 Pulsar un botón de la botonera
`pulsar_boton_de_la_botonera(id, destino)` llama a `dar_orden(.., Botonera)`. El simulador valida, por este orden, que existe el ascensor, que existe la planta y que el ascensor no está ocupado.

### 3.5 Reposicionamiento de los ascensores en reposo
1. Clasifica los ascensores:
   - los que están **en reposo**, es decir, parados y con `instante_actual >= libre_desde + 30 s`;
   - las **plantas cubiertas** por los demás: el destino de los que se desplazan y la planta actual de los que están parados pero aún no en reposo.
2. Si hay algún ascensor en reposo, ejecuta `historico.movimientos_desde(ahora - 28 días)`.
3. `plantas_de_espera_preferentes(...)` calcula la demanda por planta: llamadas del mismo día de la semana y de la misma hora.
4. `plan_de_reposicionamiento(...)` devuelve las órdenes. Para cada una se ejecuta `dar_orden(.., Reposicionamiento)`.

### 3.6 Modelo temporal de un desplazamiento (`Desplazamiento::posicion_en`)
- Duración total = `distancia × tiempo_de_desplazamiento + tiempo_de_arranque_y_de_parada`.
- Durante la primera mitad del tiempo de arranque y de parada, el ascensor sigue en la planta de origen. Después, alcanza una planta más cada `tiempo_de_desplazamiento`. Durante la parada sigue en estado `Desplazandose`, ya en la planta de destino. Al cumplirse la duración total, queda `Parado`.
- Todo se calcula con aritmética entera de `Duration` y nanosegundos. No hay `f64`, así que avanzar en varios pasos da la misma posición que avanzar de una vez (hay un test que lo comprueba).

### 3.7 Persistencia (`HistoricoDeMovimientosEnArchivo`)
- `abrir`:
  1. `fs::read_to_string`;
  2. se salta la primera línea, sin comprobar que sea la cabecera;
  3. interpreta cada línea con `split(';')` en 5 campos;
  4. si alguna línea falla, da `LineaIlegible { n }`.
- `registrar`:
  1. crea la carpeta si no existe;
  2. escribe la cabecera si el archivo está vacío o no existe;
  3. abre el archivo en modo *append* y escribe la línea;
  4. añade el movimiento a la copia en memoria solo si la escritura ha ido bien.

---

## 4. Problemas respecto a lo que pide `CLAUDE.md`

Valoración general: **el código está muy por encima de la media**. Las capas están bien separadas, el agregado está bien encapsulado, las reglas de dominio son funciones puras con muchos tests, la nomenclatura es muy cuidada y las dependencias externas están aisladas. Los problemas que siguen son sobre todo de refinamiento.

### 4.1 Arquitectura hexagonal (Ports & Adapters)

1. **No hay un puerto de entrada (*driving port*).** `PresentadorDelSimulador` depende del tipo concreto `ControlDeTrafico<H>` y, a través de `control().simulador()`, accede al agregado `SimuladorDeEdificio` para leer posiciones y configuración. El adaptador primario ve el interior de la aplicación y del dominio, lo que va contra la Ley de Deméter. No hay un trait (por ejemplo `CasosDeUsoDelSimulador`) ni un modelo de lectura propio de la aplicación (por ejemplo `EstadoDelEdificio`).
2. **El parámetro genérico `H` llega a la interfaz.** `PresentadorDelSimulador<H>` e `InterfazGraficaConEgui<H>` tienen que conocer el tipo del histórico, aunque no lo usan. Se debe a que el adaptador de entrada es dueño del caso de uso por valor y genérico.
3. **Los tests de la capa de aplicación dependen de un adaptador.** `aplicacion/control_de_trafico.rs` (tests) importa `crate::adaptadores::historico_de_movimientos_en_memoria`. Solo ocurre en los tests, pero invierte la dirección de las dependencias. Además, el doble `HistoricoQueFallaAlRegistrar` está **duplicado** en `control_de_trafico.rs` y en `presentador.rs`.
4. **El error del puerto conoce un detalle del adaptador.** `ErrorDeHistoricoDeMovimientos::LineaIlegible { numero_de_linea }` está definido en el dominio, pero el concepto de "línea" pertenece al almacenamiento en archivo de texto. Otro adaptador, como una base de datos, no tendría líneas.
5. **Las capas solo las respeta la convención.** Todo está en un único crate, así que nada impide que `dominio` importe `adaptadores`. Ahora no ocurre, pero el compilador no lo vigila.
6. **El reloj no es un puerto.** Es una función libre que solo se usa en `main.rs`. Es aceptable, porque la simulación lleva su propio tiempo, pero no se puede sustituir si algún día hiciera falta, por ejemplo para empezar en una fecha y hora configurable.

### 4.2 Principios SOLID

1. **SRP: `ControlDeTrafico` acumula varias responsabilidades.** Asigna llamadas, gestiona la cola de pendientes, anota cuándo queda libre cada ascensor, decide la política de registro (qué se registra y qué no), orquesta el reposicionamiento y avanza el tiempo. Son 354 líneas de producción, que todavía se leen bien, pero cada funcionalidad nueva (cola de destinos, estrategias) lo hará crecer.
2. **SRP / DRY: lógica duplicada.**
   - `anotar_cuando_quedara_libre` repite el cálculo de duración que ya hace `Desplazamiento` en el simulador (ya está anotado en pendientes).
   - `presentador::boton_de_llamada_encendido` repite la regla de dominio `ControlDeTrafico::hay_una_llamada_en_curso_para`: "pendiente, o un ascensor se dirige a ella". Si cambia la regla en el control, el botón mostraría algo distinto de lo que hace el control.
   - `presentador::pulsar_boton_de_la_botonera` **deduce** que el ascensor "ya está en la planta" mirando el estado después de dar la orden. La aplicación no devuelve un resultado explícito, como sí hace en las llamadas con `ResultadoDeLaLlamada`.
3. **OCP: las estrategias están fijas en el código.** La selección (`ascensor_libre_mas_cercano`) y el reposicionamiento son funciones libres a las que se llama directamente. Para probar otra estrategia, como el algoritmo de ascensor clásico o una ponderación por antigüedad, habría que modificar `ControlDeTrafico`. Es un punto natural de extensión en un simulador de experimentación, aunque hoy aplicar YAGNI también es defendible.
4. **DIP:** en el lado de salida está bien resuelto, porque el control depende del trait del histórico. En el lado de entrada no, porque el presentador depende de tipos concretos (ver 4.1.1).
5. **ISP y LSP:** sin problemas. El puerto del histórico es pequeño y cohesionado.

### 4.3 DDD y glosario (`src/glosario_de_dominio.md`)

1. **El "control de tráfico" es un concepto de dominio que vive en la capa de aplicación.** El glosario lo define en la sección de dominio, y contiene reglas de negocio con estado: la cola FIFO de llamadas pendientes, qué es "llamada en curso", cuándo está un ascensor "en reposo" y qué movimientos se registran. Las reglas puras sí están en el dominio (selección y reposicionamiento), pero la política con estado se mezcla con la orquestación de E/S, que es el histórico. Así, el servicio de aplicación tiende a ser "gordo" y el modelo de dominio a ser algo anémico en esta parte.
2. **Hay conceptos del glosario sin tipo propio** (*primitive obsession*):
   - **instante de simulación**: es `Duration`, y se confunde con las duraciones. Ya está anotado en pendientes;
   - **llamada / llamada pendiente**: es un `IdentificadorDePlanta` dentro de un `VecDeque`;
   - **franja horaria**: es `u32` (`hora_del_dia`);
   - **demanda**: es `usize` dentro de un `HashMap`;
   - **tiempo de reposo**: es un `Duration` suelto.
3. **Invariantes que no imponen los tipos:**
   - `IdentificadorDeAscensor(pub u32)` admite el 0, pero el glosario dice "se numeran desde 1";
   - `IdentificadorDePlanta(pub i32)` y `IdentificadorDeAscensor` tienen el campo público, y por eso unas 16 líneas de producción fuera de los propios tipos acceden a `.0`: 8 en el presentador, 5 en la vista y 3 en el adaptador de archivo;
   - `ConfiguracionDelEdificio` tiene los campos públicos y `validar()` aparte, así que se pueden crear configuraciones inválidas. Solo se validan al pasarlas a `SimuladorDeEdificio::nuevo`.
4. **Discrepancias entre el glosario y el código:**
   - el código usa términos que no están en el glosario: **posición** (`PosicionDeAscensor`), **estado**, **orden** (`ordenar_mover_ascensor`, `dar_orden` y `OrdenDeReposicionamiento`), **resultado de la llamada**, **configuración del control de tráfico** y **días de histórico a considerar**;
   - el glosario dice "en las últimas 4 semanas", pero el código lo tiene configurable (`dias_de_historico_a_considerar = 28`). La funcionalidad original decía "el último mes";
   - la regla del modelo temporal "el arranque es **la mitad** del tiempo de arranque y de parada" solo está en un comentario de `ascensor.rs`. El glosario define el tiempo de arranque y de parada como un único bloque de segundos extra;
   - `ordenar_mover_ascensor` no usa el término del glosario. El glosario llama **desplazamiento** a la "orden a un ascensor de ir de una planta de origen a una de destino", así que `ordenar_desplazamiento` encajaría mejor;
   - **marcha** (`Llegando`) está definida en la sección de interfaz, pero describe una fase del dominio: la parada en la planta de destino. El dominio no la expone y el presentador la deduce.
5. **Los errores están repartidos sin un criterio claro.** `ErrorDeConfiguracion` y `ErrorDeSimulacion` están juntos en `errores.rs`, mientras que `ErrorDeHistoricoDeMovimientos` está en el módulo de su puerto y `ErrorDeControlDeTrafico` en su caso de uso.

### 4.4 Estilo y nomenclatura

La nomenclatura es muy buena: nombres largos y descriptivos en español, sin tildes ni eñes en los identificadores (comprobado con `grep`). Los nombres de los tests son frases completas. Excepciones menores:

1. Hay closures de una letra en los tests del presentador: `|p|` (línea 454), `|a|` (473) y `|b|` (482).
2. Los parámetros genéricos tienen nombre de una sola letra (`H`). Es idiomático en Rust, pero `CLAUDE.md` pide nombres descriptivos, por ejemplo `Historico`.
3. Los comentarios evitan también tildes y eñes ("dueno", "Anade", "anio"), aunque la regla solo afecta a los identificadores. Leen peor ("dueno") y no son coherentes con los mensajes para el usuario, que sí las llevan.
4. Algunos nombres de tests son tan largos que `rustfmt` pone la llave `{` sola en la línea siguiente (`fn ...()\n     {`). Es solo estético.

### 4.5 Comportamiento: riesgos y posibles defectos

1. **Rendimiento del reposicionamiento a largo plazo.** `reposicionar_ascensores_en_reposo` se ejecuta en **cada fotograma** (cada ≤100 ms) mientras haya algún ascensor en reposo, que es la situación habitual cuando no se hace nada. Cada vez, `movimientos_desde` recorre **todo** el histórico en memoria, **copia** en un `Vec` nuevo los movimientos de los últimos 28 días y vuelve a calcular la demanda. El histórico es permanente y solo crece, así que el coste por fotograma es O(n) y aumenta con el tiempo de uso. Además, `abrir` carga el archivo entero en memoria.
2. **El control depende del tamaño del paso de tiempo.** El simulador da posiciones iguales se avance en un paso o en varios, pero el control no. Las llamadas pendientes y los reposicionamientos se atienden **al final** del paso, y el movimiento se registra con la fecha y hora del final del paso, no con la del instante en que quedó libre el ascensor. El tope de 1 s por fotograma lo limita en la interfaz, pero no hay ningún test que documente esa propiedad.
3. **La demanda incluye las llamadas de la sesión en curso.** El histórico de los últimos 28 días incluye el día de hoy, así que una llamada de hace unos segundos ya cuenta como demanda y atrae a los ascensores en reposo. En `datos/historico_de_movimientos.csv` se ve: el ascensor 3 se reposiciona a la planta 3 justo después de una llamada en la 3. Es coherente con el glosario, pero puede no ser el comportamiento deseado ("según el día de la semana").
4. **Fecha y hora simulada en un histórico "real".** La fecha y hora que se registra es la de inicio más el instante de simulación. Con el tope de 1 s por fotograma, o con la ventana sin repintar, el tiempo simulado se retrasa respecto del real y el histórico permanente guarda horas que no son reales. Además, guarda nanosegundos (`2026-10-05T16:08:40.361996248`).
5. **Robustez del archivo** (ya anotado en pendientes): la cabecera no se comprueba, una línea que ha quedado cortada estropea la siguiente, y **una sola línea ilegible impide arrancar la aplicación**.
6. **Ruta del histórico relativa al directorio de trabajo.** `"datos/historico_de_movimientos.csv"` está fijada en `main.rs`. Si se ejecuta desde otro directorio, se crea otro histórico distinto. Las configuraciones del edificio y del control también están fijadas, sin archivo de configuración ni argumentos.
7. **Si el histórico falla en una llamada pendiente, la llamada desaparece de la cola**, aunque el ascensor acuda (requisito R6.6.4). Está documentado, pero ningún test lo cubre (ya anotado en pendientes).
8. Si el histórico falla de forma persistente, `pasar_el_tiempo` sobrescribe el mensaje de error en cada fotograma, y el mensaje de la última acción del usuario se pierde.

### 4.6 Forma de trabajar, flujo y documentación

1. **La sección "Mapa de la arquitectura actual" de `CLAUDE.md` está vacía.** Los agentes no tienen esa referencia y cada uno la reconstruye, o la guarda en su memoria, como el revisor en `decisiones_arquitectura_dominio.md`.
2. **Hay una tarea pendiente que ya está resuelta.** "Mensajes legibles para los errores del dominio" ya está implementada (`Display` y `std::error::Error`), pero sigue en `0_funcionalidades_y_tareas_pendientes/`. Los agentes solo pueden añadir archivos en esa carpeta, así que no pueden retirarla. Le corresponde al usuario.
3. **Las carpetas `1_listo_para_implementar/` y `2_en_curso/` no tienen `.gitkeep`**, así que no aparecen en un clon. Los agentes las crean si no existen, así que no es grave.
4. **El histórico del bucle señala que TDD no se siguió de forma estricta** (vuelta 2: tests y código a la vez) y que la validación no cotejó los tests con los requisitos. Los revisores detectaron con mutaciones 6 tests débiles, y quedan 2 huecos anotados en pendientes.
5. `settings.json` prohíbe leer la carpeta `zz - ...`, pero **no impide leer `documentacion/`** ni modificar los archivos ya existentes de `0_funcionalidades_y_tareas_pendientes/`. Esas dos restricciones de `CLAUDE.md` solo dependen de que el agente las cumpla.
6. Ni la vista con egui ni `main.rs` tienen tests automáticos. Es una decisión explícita (U14), y el apartado 8 de los requisitos de la vuelta 3 incluye una comprobación manual pendiente.

---

## 5. Sugerencias de mejora

### 5.0 Ideas ya registradas en `trabajo/0_funcionalidades_y_tareas_pendientes/`
Este análisis las confirma y no las repite con detalle: `InstanteDeSimulacion`, el simulador dice cuándo queda libre cada ascensor, cola de destinos por ascensor, `Display` para los identificadores, robustez del archivo del histórico, tests que faltan en el control de tráfico, pausa y velocidad de la simulación, y animación del movimiento. "Mensajes legibles para los errores del dominio" ya está hecha y se puede retirar.

### 5.1 Arquitectura (prioridad alta o media)
1. **Crear un puerto de entrada y un modelo de lectura en la aplicación.**
   - Una consulta `ControlDeTrafico::estado_actual() -> EstadoDelEdificio` (posiciones, plantas, llamadas pendientes, **plantas con llamada en curso**, fecha y hora), y que el presentador solo use eso, sin `control().simulador()`.
   - Opcionalmente, un trait `CasosDeUsoDelSimulador` (pulsar llamada, pulsar botonera, avanzar tiempo y estado actual) del que dependa el presentador. Así desaparece el genérico `H` de la interfaz y el presentador se puede probar con un doble del caso de uso.
2. **Devolver un resultado explícito de la botonera:** `ResultadoDeLaOrdenDeBotonera { AscensorEnviado, YaEstabaEnLaPlanta }`, igual que `ResultadoDeLaLlamada`. Así el presentador no tiene que deducirlo.
3. **Mover al dominio la política con estado del control de tráfico.** Por ejemplo, un `ControlDeTraficoDelEdificio` o varios objetos de dominio pequeños: `ColaDeLlamadasPendientes`, `RegistroDeReposo` (libre desde) y `PoliticaDeRegistroDeMovimientos`. La capa de aplicación quedaría como un orquestador fino: consulta el histórico, invoca el dominio y registra los movimientos resultantes, que podrían ser **eventos de dominio**.
4. **Generalizar el error del puerto:** `ErrorDeHistoricoDeMovimientos::DatosIlegibles { detalle }` en lugar de `LineaIlegible`. El adaptador de archivo pondría el número de línea en el detalle.
5. **Que el compilador haga respetar las capas:** un *workspace* con crates `dominio`, `aplicacion`, `adaptadores` y `simulador` (binario), o al menos módulos con visibilidad `pub(crate)` más restrictiva. Con crates separados, una dependencia de `dominio` hacia `adaptadores` no compilaría.

### 5.2 Dominio y lenguaje ubicuo
1. Ampliar el glosario con **posición**, **estado**, **orden**, **resultado de la llamada**, **fase de arranque / recorrido / parada** (y la regla de "la mitad"), **días de histórico a considerar** y **llamada en curso**. Unificar "últimas 4 semanas", "último mes" y "28 días".
2. Valorar renombrar `ordenar_mover_ascensor` como `ordenar_desplazamiento` para alinearlo con el glosario.
3. Tipos propios para `Llamada` (o `LlamadaPendiente`), `FranjaHoraria { dia_de_la_semana, hora }` y `Demanda`. `IdentificadorDeAscensor` con `NonZeroU32` o constructor validado, y campos privados más `Display` (ya está en pendientes).
4. `ConfiguracionDelEdificio::nueva(...) -> Result<Self, ErrorDeConfiguracion>` con campos privados, para que no se puedan crear configuraciones inválidas.
5. Valorar que el dominio exponga la fase de parada ("llegando"), en lugar de que el presentador la deduzca comparando la planta actual con la de destino.
6. Agrupar los errores con un criterio único: cada uno en el módulo de su concepto, o todos en `errores.rs`.

### 5.3 Rendimiento y robustez
1. **Calcular la demanda en caché** por franja horaria (día de la semana y hora). Solo cambia al cambiar de franja o al registrar una llamada nueva. Otra opción: que el histórico ofrezca una consulta más específica, como `llamadas_en_la_franja(dia, hora, desde)`, con un índice en el adaptador. Así se evita recorrer el histórico y copiar parte de él en cada fotograma.
2. Atender las llamadas pendientes en el **instante exacto** en que queda libre cada ascensor, dividiendo el avance del control en subpasos hasta el siguiente evento. Así el control tampoco depende del tamaño del paso.
3. Decidir si la demanda debe excluir las llamadas de la sesión en curso o del día de hoy, y reflejarlo en el glosario.
4. Truncar a segundos o milisegundos la fecha y hora de inicio. Revisar si el histórico permanente debe guardar el tiempo simulado o el real.
5. Hacer configurables la ruta del histórico y las configuraciones del edificio y del control (argumentos o archivo). Si el histórico está dañado, valorar arrancar avisando, por ejemplo en modo de solo lectura o con un histórico vacío, en lugar de no arrancar.

### 5.4 Tests y calidad
1. Añadir los tests que faltan (ya en pendientes): R7.2 y R6.6.4. Añadir también uno que documente la dependencia del control respecto del tamaño del paso, o su independencia si se aplica 5.3.2.
2. Sacar los dobles de test compartidos, como `HistoricoQueFallaAlRegistrar`, a un módulo `#[cfg(test)]` común (por ejemplo `crate::dobles_de_test`), para que los tests de la aplicación no dependan de `adaptadores` y no se repitan.
3. Pasar los tests largos a archivos aparte (`control_de_trafico/tests.rs` y `presentador/tests.rs`). Los dos archivos tienen más de 1.000 líneas, de las que ~75% son tests, y el código de producción se pierde entre ellos.
4. Valorar activar en `Cargo.toml` (`[lints.clippy]`) algunos lints de `pedantic`, por ejemplo `must_use_candidate` y `missing_errors_doc`, ahora que el código ya está casi limpio.
5. Cambiar las closures de una letra del presentador (`|p|`, `|a|`, `|b|`) por nombres descriptivos.
6. Probar a mano la interfaz gráfica (apartado 8 de los requisitos de la vuelta 3), que sigue pendiente.

### 5.5 Flujo de trabajo y documentación
1. Rellenar **"Mapa de la arquitectura actual"** en `CLAUDE.md`. Se puede partir de los apartados 1.2, 2 y 2.4 de este informe. Es un cambio en un archivo de configuración: lo decide y lo aplica el usuario.
2. Retirar de pendientes "Mensajes legibles para los errores del dominio", que ya está hecha.
3. Añadir `.gitkeep` a `trabajo/1_listo_para_implementar/` y `trabajo/2_en_curso/`.
4. Valorar añadir a `settings.json` `deny: Read(/documentacion/**)` y una regla que impida editar los archivos existentes de `trabajo/0_funcionalidades_y_tareas_pendientes/`, para que esas restricciones de `CLAUDE.md` no dependan solo del agente.
5. Valorar que `validar_codigo_y_commitearlo` coteje los tests con los requisitos y no solo las casillas `[x]`, y que `programar_codigo` deje constancia de la fase RED de cada test.
