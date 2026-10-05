# Interfaz gráfica de usuario para manejar el simulador: requisitos y casos de uso

Funcionalidad de origen: `trabajo/2_en_curso/Interfaz de usuario para manejar el simulador.md`.

Se construye sobre las dos funcionalidades ya implementadas:
- "Estructura básica del edificio y ascensores" (commit `4b7a4dd`; requisitos en `trabajo/3_historico/20261005T083405/`). Supuestos S1–S7.
- "Un control de tráfico básico" (commit `d03e7dc`; requisitos en `trabajo/3_historico/20261005T091709/`). Supuestos T1–T18.

Siguen vigentes todos esos supuestos. Los más relevantes para la interfaz son estos:
- S2: tiempo simulado explícito.
- S6: un ascensor que se desplaza rechaza órdenes con `AscensorOcupado`.
- S7: ordenar ir a la planta en la que ya está parado no hace nada.
- T1: el control de tráfico es el único punto de entrada.
- T4: llamadas pendientes.
- T5: llamada repetida.
- T6: las llamadas pendientes y el reposicionamiento se procesan al final de cada avance.
- T14: tiempo de reposo de 30 s.

El enunciado pide una interfaz gráfica con **egui** que permita:
1. **Ver la posición de todos los ascensores** del edificio.
2. **Pulsar el botón de llamada** de una planta.
3. **Pulsar un botón de la botonera** de un ascensor para ir a la planta deseada.

La interfaz es un **adaptador de entrada** (lado conductor de la arquitectura hexagonal). Solo llama a la API pública de `ControlDeTrafico` y dibuja su estado.


## 1. Supuestos adoptados (pendientes de confirmar por el usuario)

Se numeran U1, U2... para no confundirlos con S1–S7 y T1–T18.

| # | Supuesto | Motivo | Alternativa |
|---|----------|--------|-------------|
| U1 | Se usa **eframe 0.36** (el marco oficial de egui) con el renderizador **glow** (OpenGL), sin las features por defecto (ver apartado 2). | egui no abre ventanas por sí solo; eframe sí. glow arrastra menos dependencias que wgpu, el renderizador por defecto en 0.36. | Las features por defecto (wgpu). |
| U2 | `cargo run` **abre la ventana** de la interfaz. La **demostración por consola** actual de `main.rs` **se elimina**. | La interfaz sustituye a la demostración como forma de manejar el simulador. | Conservar la demostración tras un argumento (por ejemplo `--demostracion`). |
| U3 | El tiempo simulado avanza **al ritmo del tiempo real** (factor 1). **No hay pausa ni control de velocidad.** | El enunciado no lo pide. Se propone como idea aparte en `0_funcionalidades_y_tareas_pendientes/`. | Añadir ya la pausa y la velocidad (x1, x2, x5...). |
| U4 | En cada fotograma, la simulación avanza el tiempo real transcurrido desde el fotograma anterior, con un **máximo de 1 s por fotograma**. Si la ventana deja de repintarse (minimizada u oculta), la simulación **se detiene** en lugar de dar un salto al volver. | Un salto grande procesaría las llamadas pendientes con mucho retraso (T6). Además, dejaría pasar el tiempo sin que el usuario lo vea. | Avanzar todo de una vez, o en pasos pequeños hasta alcanzar el tiempo real. |
| U5 | La ventana se **repinta cada 100 ms** (unos 10 fotogramas por segundo), aunque el usuario no haga nada. | egui solo repinta cuando hay eventos. La planta actual es entera y cambia cada 2 s, así que 10 fotogramas por segundo bastan. | Repintar sin parar (60 fotogramas por segundo, más CPU). |
| U6 | **Representación**: una rejilla con una **fila por planta** (la más alta arriba) y, en cada fila, el botón de llamada de la planta y una **celda por cada ascensor** (su **hueco**). En el hueco se marca la **planta actual** del ascensor y su **marcha** (parado, subiendo, bajando o llegando), y también su **planta de destino**. **No hay animación** entre plantas. | El simulador solo da la planta actual entera, y no hace falta más para "ver la posición". | Animar el movimiento con una posición fraccionaria. Exige ampliar el simulador; se propone como idea aparte. |
| U7 | Un **botón de llamada encendido** es aquel cuya pulsación no tendría efecto (T5). Lo está si la planta tiene una **llamada pendiente**, o si **algún ascensor se desplaza hacia ella**, por el motivo que sea. Se apaga cuando el ascensor queda parado en la planta. | Basta la API pública actual (`llamadas_pendientes()` y `posiciones_de_todos_los_ascensores()`). Además, el botón encendido significa justo lo que hace el control. | Encenderlo solo por las llamadas, y no por la botonera ni por el reposicionamiento. Exige que el control exponga las "llamadas en curso", lo que amplía su API. |
| U8 | Los **botones de llamada están siempre habilitados**. Pulsar uno encendido solo muestra el mensaje "ya estaba en curso". | Es lo que pasa en un ascensor real. | Deshabilitar los botones encendidos. |
| U9 | Los botones de la **botonera** de un ascensor están **habilitados solo si el ascensor está parado** (S6). Mientras se desplaza, están todos deshabilitados y el de su **planta de destino está encendido**. El botón de la planta en la que está parado sigue habilitado: pulsarlo no hace nada (S7) y muestra "ya está en la planta". | Se evita el error `AscensorOcupado` en el uso normal y se ve adónde va el ascensor. | Dejarlos habilitados y mostrar el error al pulsarlos. |
| U10 | Hay un **mensaje para el usuario**: el resultado de la última acción del usuario, o el último error. Se conserva hasta la siguiente acción o el siguiente error. Los errores se destacan con el color de error del tema. Los textos de la interfaz van en **español con tildes** (las fuentes por defecto de egui las incluyen). | Así el usuario sabe qué ha pasado con cada pulsación. | Un historial de mensajes, o una ventana emergente para los errores. |
| U11 | Los errores del dominio y del control de tráfico implementan **`std::fmt::Display`**, con mensajes en español, y **`std::error::Error`**. Es la idea `0_funcionalidades_y_tareas_pendientes/Mensajes legibles para los errores del dominio.md`. | La interfaz tiene que mostrar los errores. Además, `main.rs` puede informar de un fallo al arrancar. | Que el presentador traduzca los errores a texto sin tocar el dominio. |
| U12 | La **fecha y hora** se muestra como `Lunes 05/10/2026 08:00:10`: día de la semana, `DD/MM/AAAA` y `HH:MM:SS`, **sin fracción de segundo**. Para ello, `FechaYHora` ofrece consultas de sus componentes (año, mes, día, minuto y segundo). | El tiempo avanza por fracciones de segundo, y el texto ISO de `Display` mostraría la fracción (`08:00:10.123456789`). | Recortar el texto ISO. Funciona, pero depende del formato de otro módulo. |
| U13 | El presentador depende directamente de `ControlDeTrafico<H>`, genérico en el histórico. **No se define un trait** como puerto de entrada. Sus tests usan el control real con el histórico en memoria. | El control es determinista y fácil de crear, así que un trait solo serviría para tener dobles de test. Mientras haya una sola implementación, no aporta nada (el mismo criterio que con el reloj en R8.4 de la funcionalidad anterior). | Un trait `PuertoDeEntradaDelSimulador` implementado por `ControlDeTrafico`. |
| U14 | **No hay tests automáticos de la capa de dibujo.** Se reduce a un archivo fino, que se comprueba a mano con la lista del apartado 8. | `egui_kittest`, la biblioteca de tests de egui, **no está en la caché** de cargo: haría falta red y otra dependencia. Toda la lógica comprobable está en el presentador. | Añadir `egui_kittest` como dependencia de desarrollo. Hace falta descargarla. |
| U15 | Si no se puede crear el simulador, abrir el histórico o abrir la ventana, `main.rs` escribe `No se ha podido iniciar el simulador: <mensaje>` en la salida de errores y **termina con código de error**, sin abrir la ventana. | Hoy hace `expect`, que termina con un pánico poco legible. | Mantener el `expect`. |
| U16 | La ventana se titula **"Simulador de ascensores"** y su tamaño inicial es de unos **960 × 640** puntos. | Caben las 10 plantas, los 3 huecos y las 3 botoneras. | Otro tamaño. |
| U17 | **No** se incluyen aquí los tests de `0_funcionalidades_y_tareas_pendientes/Tests que faltan en el control de trafico.md`. | Esta funcionalidad solo toca el control de tráfico para añadir `Display` a su error. | Incluirlos ahora (son 2 o 3 tests más en `control_de_trafico.rs`). |


## 2. Dependencias externas (pendientes de confirmar por el usuario)

### `eframe` (propuesta)

```toml
[dependencies]
chrono = { version = "0.4", default-features = false, features = ["clock", "std"] }
eframe = { version = "0.36", default-features = false, features = ["default_fonts", "glow", "wayland", "x11"] }
```

- **Para qué:** abrir la ventana nativa y dibujar la interfaz con egui. egui es la biblioteca de interfaz de modo inmediato, pero no abre ventanas. eframe es su marco oficial: la integra con `winit` (ventanas y eventos) y con un renderizador.
- **No se añade `egui` como dependencia aparte.** Se usa la reexportación `eframe::egui`, y así no pueden desalinearse sus versiones.
- **Features:**
  - `default_fonts`: las fuentes de egui, con tildes y eñes.
  - `glow`: renderizador OpenGL.
  - `wayland` y `x11`: los dos sistemas de ventanas de Linux. La sesión actual es Wayland, con XWayland disponible.
- **Features por defecto que se dejan fuera:**
  - `wgpu`: el renderizador por defecto en 0.36, mucho más pesado.
  - `accesskit`: lectores de pantalla.
  - `links`: abrir URLs.
  - `web_screen_reader`: solo para la web.
  - `persistence`: guardar el estado de la ventana.
- **Cómo se contiene:** solo la usa `src/adaptadores/interfaz_grafica/vista_con_egui.rs`. El modelo de vista y el presentador no ven ningún tipo de egui (igual que `chrono` está confinado en `FechaYHora`).

### Comprobaciones hechas al preparar este trabajo (en una copia del proyecto, en el scratchpad)

- **Caché local:** `eframe` 0.36.2, `egui` 0.36.2, `egui-winit` 0.36.2, `egui_glow` 0.36.2, `winit` 0.30.13, `glow` 0.17.0 y `glutin` 0.32.3, y todas sus dependencias para Linux, están en `~/.cargo/registry`. **No hace falta descargar nada.**
  - La resolución añade **269 paquetes** al `Cargo.lock`.
  - Con la feature por defecto `wgpu` también estaría todo en la caché, pero serían unos 115 paquetes más.
- **Sin red:** la primera vez, después de añadir la dependencia a `Cargo.toml`, hay que ejecutar **`cargo build --offline`** (o `cargo add eframe@0.36 --no-default-features --features default_fonts,glow,wayland,x11 --offline`). Así se resuelve con el índice local. Con el `Cargo.lock` ya actualizado, los comandos normales (`cargo build`, `cargo test`, `cargo clippy`) no acceden a la red. Comprobado.
- **Versión de Rust:** eframe 0.36 exige Rust 1.95, y la instalada es la 1.98.
- **Con la dependencia añadida, los 123 tests actuales siguen pasando** y `cargo clippy --all-targets` no da ningún aviso.
- **Coste:**
  - La primera compilación tarda en torno a 1,5 minutos.
  - La carpeta `target/` crece en torno a 1,5 GB.
  - Un ejecutable de depuración mínimo ocupa unos 230 MB.
  - `target/` está en `.gitignore`, así que no afecta a las directrices de no subir ejecutables ni archivos de más de 10 MB.
- **Bibliotecas del sistema en Linux:**
  - **Compilar no necesita ningún paquete `-dev`:** el ejecutable solo enlaza con `libc`, `libm` y `libgcc_s`.
  - **Al ejecutarse**, `winit` y `glutin` cargan dinámicamente `libwayland-client`, `libwayland-egl`, `libwayland-cursor`, `libxkbcommon`, `libEGL`, `libGL`, `libX11`, `libX11-xcb`, `libxcb`, `libXcursor`, `libXrandr`, `libXi` y `libxkbcommon-x11`. **Todas están instaladas** en `/lib/x86_64-linux-gnu/`.
  - Si Wayland diera problemas, se puede forzar X11 con `WAYLAND_DISPLAY= cargo run`.
- **Alternativas:**
  - Las features por defecto (wgpu): están en la caché, pero son más pesadas.
  - `egui_kittest` para probar la capa de dibujo: **no está en la caché** (ver U14).

### API de egui/eframe 0.36 (comprobada compilando un esqueleto con clippy sin avisos)

La versión 0.36 ha renombrado cosas que aparecen de otra forma en muchos ejemplos de internet. Las funciones obsoletas dan avisos `deprecated`, y el bucle no admite ningún aviso.

| Lo que se usa | Lo que ya no se usa |
|---------------|---------------------|
| `impl eframe::App` con `fn ui(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame)` | `fn update(&mut self, ctx: &egui::Context, ...)`, que ya no existe |
| `egui::Panel::top(id)`, `::bottom(id)`, `::left(id)` y `::right(id)`, y después `.show(ui, \|ui\| ...)` | `SidePanel` y `TopBottomPanel`, que ya no existen; `show_inside`, que está obsoleto |
| `egui::CentralPanel::default().show(ui, \|ui\| ...)` | `CentralPanel::show_inside`, que está obsoleto |
| `eframe::run_native(nombre, opciones, Box::new(\|_contexto\| Ok(Box::new(aplicacion))))` | `run_simple_native`, que ya no existe |

Otras piezas comprobadas:
- `eframe::NativeOptions { viewport: egui::ViewportBuilder::default().with_title(...).with_inner_size([960.0, 640.0]), ..Default::default() }`
- `egui::Grid::new(id).striped(true).show(ui, ...)` y `ui.end_row()`
- `ui.add_enabled(habilitado, egui::Button::new(texto).selected(encendido)).clicked()`
- `ui.colored_label(ui.visuals().error_fg_color, texto)`
- `ui.ctx().request_repaint_after(Duration::from_millis(100))` y `ui.ctx().request_repaint()`
- `eframe::Error` implementa `std::error::Error`.

**Símbolos disponibles en las fuentes por defecto:**
- En el texto normal se ven bien `⬆ ⬇ ⏶ ⏷ ■ ○ ◎ ⏹` y las letras con tilde.
- `▲ ▼ ↑ ↓ ●` solo existen en la fuente monoespaciada (`egui::RichText::new(...).monospace()`). En el texto normal aparecerían como recuadros vacíos.


## 3. Cambios en la API existente

**No se cambia ninguna firma existente ni ninguno de los 123 tests.** Solo se añade API:

- `FechaYHora` (`src/dominio/fecha_y_hora.rs`): `anio() -> i32`, `mes() -> u32`, `dia() -> u32`, `minuto() -> u32` y `segundo() -> u32`. `hora_del_dia()` ya existe. El segundo no incluye la fracción.
- `impl std::fmt::Display` y `impl std::error::Error` para:
  - `ErrorDeConfiguracion` y `ErrorDeSimulacion` (`src/dominio/errores.rs`);
  - `ErrorDeHistoricoDeMovimientos` (`src/dominio/historico_de_movimientos.rs`);
  - `ErrorDeControlDeTrafico` (`src/aplicacion/control_de_trafico.rs`).

El simulador y el control de tráfico **no necesitan más cambios**. El presentador usa la API pública actual:
- `pulsar_boton_de_llamada`, `pulsar_boton_de_la_botonera` y `avanzar_tiempo`;
- `simulador()`, con `posiciones_de_todos_los_ascensores()`, `posicion_del_ascensor()`, `configuracion()` e `instante_actual()`;
- `llamadas_pendientes()` y `fecha_y_hora_actual()`.


## 4. Requisitos funcionales

### R1. Mensajes legibles de los errores (U11)

- R1.1. Cada error implementa `Display` con un mensaje en español, en minúscula inicial y sin punto final, como es costumbre en Rust. Así se puede componer detrás de un prefijo.
- R1.2. Textos exactos:

| Error | Mensaje |
|-------|---------|
| `ErrorDeConfiguracion::PlantaMasBajaPorEncimaDeLaMasAlta` | `la planta más baja está por encima de la más alta` |
| `ErrorDeConfiguracion::PlantaPrincipalFueraDelRangoDePlantas` | `la planta principal no está entre las plantas del edificio` |
| `ErrorDeConfiguracion::SinAscensores` | `el edificio no tiene ascensores` |
| `ErrorDeConfiguracion::TiempoDeDesplazamientoCero` | `el tiempo de desplazamiento no puede ser cero` |
| `ErrorDeSimulacion::AscensorInexistente` | `el ascensor no existe en el edificio` |
| `ErrorDeSimulacion::PlantaInexistente` | `la planta no existe en el edificio` |
| `ErrorDeSimulacion::AscensorOcupado` | `el ascensor se está desplazando y no acepta nuevas órdenes` |
| `ErrorDeHistoricoDeMovimientos::AccesoAlAlmacenamiento { detalle }` | `no se ha podido acceder al histórico de movimientos: {detalle}` |
| `ErrorDeHistoricoDeMovimientos::LineaIlegible { numero_de_linea }` | `la línea {numero_de_linea} del histórico de movimientos no se puede leer` |
| `ErrorDeControlDeTrafico::Simulacion(error)` | el mensaje de `error` |
| `ErrorDeControlDeTrafico::Historico(error)` | el mensaje de `error` |

- R1.3. Los cuatro tipos implementan `std::error::Error`. Como el `Display` de `ErrorDeControlDeTrafico` ya incluye el mensaje del error interno, `source()` se deja con su implementación por defecto (`None`): si no, el mensaje saldría repetido al recorrer la cadena de errores.

### R2. Componentes de la fecha y hora (U12)

- R2.1. `FechaYHora::anio()`, `mes()` (1–12), `dia()` (1–31), `minuto()` (0–59) y `segundo()` (0–59) devuelven el componente correspondiente.
- R2.2. `segundo()` no incluye la fracción de segundo. Por ejemplo, para las 08:00:10,5, devuelve 10.

### R3. Modelo de vista

Son datos puros que describen lo que se dibuja. No contienen tipos de egui, y se pueden comparar con `assert_eq!`: derivan `Debug`, `Clone`, `PartialEq` y `Eq`.

- R3.1. `ModeloDeVista`:
  - `texto_de_la_fecha_y_hora`, ver R4.7;
  - `plantas`, una por planta del edificio, **de la más alta a la más baja**;
  - `ascensores`, uno por ascensor, **ordenados por identificador**;
  - `texto_de_las_llamadas_pendientes`, ver R4.8;
  - `mensaje_para_el_usuario: Option<MensajeParaElUsuario>`.
- R3.2. `PlantaEnLaVista`:
  - `planta: IdentificadorDePlanta`;
  - `boton_de_llamada_encendido: bool`;
  - `celdas_de_los_huecos: Vec<CeldaDelHueco>`, una por ascensor y en el mismo orden que `ascensores`.
- R3.3. `CeldaDelHueco`: `Vacia`, `Ascensor(MarchaDelAscensor)` o `DestinoDelAscensor`.
- R3.4. `MarchaDelAscensor`:
  - `Parado`;
  - `Subiendo`;
  - `Bajando`;
  - `Llegando`: está en su planta de destino, en la fase de parada (R6.4 de la primera funcionalidad).
- R3.5. `AscensorEnLaVista`:
  - `identificador_de_ascensor`;
  - `planta_actual`;
  - `marcha`;
  - `planta_de_destino: Option<IdentificadorDePlanta>`;
  - `descripcion_del_estado: String`;
  - `botonera: Vec<BotonDeLaBotonera>`, un botón por planta, **de la más alta a la más baja**.
- R3.6. `BotonDeLaBotonera`: `planta`, `habilitado: bool` y `encendido: bool`.
- R3.7. `MensajeParaElUsuario`: `Informacion(String)` o `Error(String)`.
- R3.8. `AccionDelUsuario`, lo que la capa de dibujo devuelve cuando el usuario pulsa algo:
  - `PulsarBotonDeLlamada { planta }`;
  - `PulsarBotonDeLaBotonera { identificador_de_ascensor, planta_de_destino }`.

### R4. Presentador: del estado del control de tráfico al modelo de vista

`PresentadorDelSimulador<H: HistoricoDeMovimientos>` es el dueño del `ControlDeTrafico<H>` y del mensaje para el usuario.

- R4.1. `nuevo(control)`. Al crearlo, no hay mensaje para el usuario.
- R4.2. `control()` da acceso de solo lectura al control. Lo usan sobre todo los tests.
- R4.3. `modelo_de_vista()` calcula el modelo a partir del estado actual del control. No cambia nada.
- R4.4. **Marcha y destino** de cada ascensor, a partir de su `PosicionDeAscensor`:
  - estado `Parado`: marcha `Parado` y destino `None`;
  - estado `Desplazandose { planta_de_destino }`: destino `Some(planta_de_destino)`. La marcha es `Subiendo` si el destino está por encima de la planta actual, `Bajando` si está por debajo, y `Llegando` si es la misma planta.
- R4.5. **Celdas del hueco.** Para la planta `p` y el ascensor `a`:
  - `Ascensor(marcha de a)`, si la planta actual de `a` es `p`;
  - si no, `DestinoDelAscensor`, si el destino de `a` es `p`;
  - si no, `Vacia`.
- R4.6. **Botones** (U7 y U9):
  - El botón de llamada de `p` está encendido si `p` está en `llamadas_pendientes()`, o si algún ascensor está `Desplazandose { planta_de_destino: p }`.
  - El botón `p` de la botonera de `a` está habilitado si `a` está parado, y encendido si el destino de `a` es `p`.
- R4.7. **Fecha y hora**: `"{dia de la semana} {DD}/{MM}/{AAAA} {HH}:{MM}:{SS}"`, con ceros a la izquierda y sin fracción de segundo. Por ejemplo, `Lunes 05/10/2026 08:00:10`. Los días son `Lunes`, `Martes`, `Miércoles`, `Jueves`, `Viernes`, `Sábado` y `Domingo`.
- R4.8. **Llamadas pendientes**: `Llamadas pendientes: ninguna`, o las plantas por orden de llegada y separadas por coma y espacio. Por ejemplo, `Llamadas pendientes: 2, -1`.
- R4.9. **Descripción del estado** de un ascensor:

| Marcha | Texto |
|--------|-------|
| Parado | `Parado en la planta {actual}` |
| Subiendo | `En la planta {actual}, subiendo hacia la {destino}` |
| Bajando | `En la planta {actual}, bajando hacia la {destino}` |
| Llegando | `Llegando a la planta {destino}` |

### R5. Presentador: acciones del usuario y paso del tiempo

- R5.1. `atender(accion)` ejecuta la acción en el control y **sustituye** el mensaje para el usuario.
  - Mensajes de `PulsarBotonDeLlamada { planta: p }`:

| Resultado | Mensaje |
|-----------|---------|
| `AscensorAsignado(a)` | `Informacion("Llamada en la planta {p}: atendida por el ascensor {a}")` |
| `PendienteDeAscensorLibre` | `Informacion("Llamada en la planta {p}: pendiente hasta que quede libre algún ascensor")` |
| `LlamadaYaEnCurso` | `Informacion("Llamada en la planta {p}: ya estaba en curso")` |
| `Err(error)` | `Error("Llamada en la planta {p}: {error}")` |

  - Mensajes de `PulsarBotonDeLaBotonera { identificador_de_ascensor: a, planta_de_destino: p }`:

| Resultado | Mensaje |
|-----------|---------|
| `Ok`, y el ascensor sigue parado en `p` (S7) | `Informacion("Ascensor {a}: ya está en la planta {p}")` |
| `Ok`, y el ascensor se desplaza | `Informacion("Ascensor {a}: va a la planta {p}")` |
| `Err(error)` | `Error("Ascensor {a}, botón de la planta {p}: {error}")` |

  - `{p}` y `{a}` son los números: `-2`, `4`... Si falla el histórico, el control ya ha dado la orden (T17): el ascensor se mueve igualmente, y el mensaje es el error.
- R5.2. `pasar_el_tiempo(tiempo_real_transcurrido: Duration)` avanza el control `min(tiempo_real_transcurrido, AVANCE_MAXIMO_DE_LA_SIMULACION_POR_FOTOGRAMA)`, que es 1 s (U4).
  - Si el control devuelve un error, el mensaje pasa a ser `Error("Al avanzar el tiempo: {error}")`.
  - Si no hay error, el mensaje no cambia.
- R5.3. El presentador no mide el tiempo real: lo recibe ya medido. Así los tests son deterministas.

### R6. Capa de dibujo con egui (sin tests automáticos; U14)

Va en `src/adaptadores/interfaz_grafica/vista_con_egui.rs`. Ha de ser lo más fina posible: **ninguna regla de negocio ni de presentación**, solo traducir el `ModeloDeVista` a widgets y los clics a `AccionDelUsuario`.

- R6.1. `InterfazGraficaConEgui<H>` guarda el presentador y el `Instant` del fotograma anterior, e implementa `eframe::App`. En cada llamada a `ui`, por este orden:
  1. Mide con `std::time::Instant` el tiempo real transcurrido desde el fotograma anterior y llama a `presentador.pasar_el_tiempo(...)`.
  2. Obtiene `presentador.modelo_de_vista()`.
  3. Dibuja el modelo y reúne las `AccionDelUsuario` de los botones pulsados.
  4. Llama a `presentador.atender(accion)` con cada acción. Si ha habido alguna, pide `request_repaint()` para que se vea el resultado enseguida.
  5. Pide `request_repaint_after(INTERVALO_DE_REPINTADO)`, con un intervalo de 100 ms (U5).
- R6.2. Disposición orientativa; el programador la puede ajustar:
  - **Panel superior:** título y `texto_de_la_fecha_y_hora`.
  - **Panel central:** una rejilla (`egui::Grid`). Columnas: "Planta", "Llamada" y "Ascensor 1", "Ascensor 2"... Una fila por cada `PlantaEnLaVista`:
    - el número de la planta;
    - el botón de llamada, que se muestra seleccionado si está encendido;
    - una celda por hueco. Por ejemplo, `■ 1` para un ascensor parado, `⬆ 1` subiendo, `⬇ 1` bajando, `■ 1` o `⏹ 1` llegando, `○` para el destino, y nada si está vacía.
  - **Panel derecho o inferior:** una botonera por ascensor, con su `descripcion_del_estado` y un botón por planta. Cada botón está habilitado o deshabilitado según `habilitado`, y se muestra seleccionado si está encendido.
  - **Panel inferior:** `texto_de_las_llamadas_pendientes` y el mensaje para el usuario, en color de error si es `Error`.
- R6.3. `ejecutar_interfaz_grafica(presentador) -> Result<(), eframe::Error>` abre la ventana (U16) y no vuelve hasta que se cierra.
- R6.4. Los símbolos han de ser de los disponibles en las fuentes por defecto (apartado 2).

### R7. Raíz de composición (`src/main.rs`)

- R7.1. Crea el simulador estándar y abre el histórico en archivo en `datos/historico_de_movimientos.csv`. Crea el control de tráfico estándar, con la fecha y hora local del sistema como inicio, y lo pasa al presentador. Después llama a `ejecutar_interfaz_grafica`. Hasta el control, igual que ahora.
- R7.2. Se elimina la demostración por consola (U2).
- R7.3. Si falla algún paso, escribe `No se ha podido iniciar el simulador: {error}` en la salida de errores (`eprintln!`) y devuelve `std::process::ExitCode::FAILURE` (U15). Si no, devuelve `ExitCode::SUCCESS`. No lleva tests.

### R8. Bucle automático

- R8.1. **No ejecutar `cargo run` en los pasos automáticos** (programar, revisar, validar): abre una ventana y bloquea hasta que alguien la cierra. Basta con `cargo build`, `cargo clippy --all-targets` y `cargo test`. La comprobación visual es manual (apartado 8).


## 5. Casos de uso

- **CU1. Arrancar la interfaz.** Al ejecutar el programa, se abre la ventana. Se ven las 10 plantas, los 3 ascensores parados en la planta 0, la fecha y hora de inicio avanzando, y "Llamadas pendientes: ninguna".
- **CU2. Ver la posición de los ascensores.** En cada momento, se ve en el hueco de cada ascensor su planta actual y su marcha, y su planta de destino si se está desplazando. También se ve su descripción del estado.
- **CU3. Pulsar el botón de llamada de una planta.** Acude el ascensor libre más cercano, o la llamada queda pendiente. El botón se enciende hasta que un ascensor queda parado en la planta. Un mensaje informa del resultado.
- **CU4. Pulsar un botón de la botonera.** El ascensor va a la planta indicada. Mientras se desplaza, su botonera está deshabilitada y el botón del destino, encendido. Un mensaje informa del resultado.
- **CU5. Ver cómo pasa el tiempo.** La simulación avanza sola: los ascensores progresan, las llamadas pendientes se atienden y, si el histórico lo indica, los ascensores en reposo se reposicionan sin que el usuario haga nada.
- **CU6. Ver un error.** Si el control rechaza una orden o falla el histórico, el mensaje lo indica, destacado como error.
- **CU7. Cerrar la interfaz.** Se cierra la ventana y termina el programa. Los movimientos de la sesión quedan en el histórico en archivo.


## 6. Ejemplos de referencia para los tests

Valores por defecto:
- edificio estándar: plantas de la -2 a la 7, 3 ascensores, 2 s por planta, y 4 s de arranque y de parada (2 s de arranque y 2 s de parada);
- configuración estándar del control;
- histórico en memoria;
- inicio el **lunes 5 de octubre de 2026 a las 08:00:00**.

Un desplazamiento de `n` plantas dura `2·n + 4` s. El ascensor alcanza la k-ésima planta del trayecto a los `2 + 2·k` s, y queda parado a los `2·n + 4` s.

Como `pasar_el_tiempo` avanza como máximo 1 s, para avanzar `n` segundos conviene una función auxiliar de test que llame `n` veces a `pasar_el_tiempo(1 s)`.

- **A. Llamada que sube.** En el instante 0 se pulsa la llamada de la planta 4. Los tres ascensores están a distancia 4, así que acude el ascensor 1 (T3). El mensaje es `Llamada en la planta 4: atendida por el ascensor 1`.

| Instante | Planta actual del ascensor 1 | Marcha | Descripción | Celda de la planta 4 en su hueco | Botón de llamada de la planta 4 | Botonera del ascensor 1 |
|----------|------------------------------|--------|-------------|----------------------------------|---------------------------------|-------------------------|
| 0 s | 0 | Subiendo | `En la planta 0, subiendo hacia la 4` | `DestinoDelAscensor` | encendido | deshabilitada, 4 encendido |
| 4 s | 1 | Subiendo | `En la planta 1, subiendo hacia la 4` | `DestinoDelAscensor` | encendido | deshabilitada |
| 10 s | 4 | Llegando | `Llegando a la planta 4` | `Ascensor(Llegando)` | encendido | deshabilitada |
| 12 s | 4 | Parado | `Parado en la planta 4` | `Ascensor(Parado)` | apagado | habilitada, ninguno encendido |

- **B. Botonera que baja.** En el instante 0, se pulsa en la botonera del ascensor 2 el botón de la planta -2. El mensaje es `Ascensor 2: va a la planta -2`.
  - 0 s: `En la planta 0, bajando hacia la -2`.
  - 4 s: planta -1.
  - 6 s: `Llegando a la planta -2`.
  - 8 s: `Parado en la planta -2`.
  - Mientras se desplaza, el botón de llamada de la planta -2 está encendido (U7: por cualquier motivo).
- **C. Llamadas pendientes.** En el instante 0, por la botonera: el ascensor 1 a la 1 (libre a los 6 s), el 2 a la 5 (a los 14 s) y el 3 a la 7 (a los 18 s).
  - Llamada en la 2: `Llamada en la planta 2: pendiente hasta que quede libre algún ascensor`.
  - Llamada en la -1: queda pendiente.
  - Texto: `Llamadas pendientes: 2, -1`. Los botones de llamada de la 2 y de la -1 están encendidos.
  - Nueva llamada en la 2: `Llamada en la planta 2: ya estaba en curso`.
  - A los 6 s, el ascensor 1 atiende la llamada de la 2: `Llamadas pendientes: -1`. El botón de la 2 sigue encendido, porque el ascensor 1 va hacia ella.
- **D. Ascensor ocupado.** Por la botonera, el ascensor 1 a la 3, y enseguida el ascensor 1 a la 5. El mensaje es `Error("Ascensor 1, botón de la planta 5: el ascensor se está desplazando y no acepta nuevas órdenes")`.
- **E. Ya está en la planta.** Al empezar, por la botonera, el ascensor 1 a la 0. El mensaje es `Informacion("Ascensor 1: ya está en la planta 0")`.
- **F. Planta inexistente.** Llamada en la planta 9. El mensaje es `Error("Llamada en la planta 9: la planta no existe en el edificio")`.
- **G. Fallo del histórico al pasar el tiempo.** Un edificio de 1 ascensor, con un doble de histórico que siempre falla al registrar con `AccesoAlAlmacenamiento { detalle: "fallo simulado" }`.
  - Por la botonera, el ascensor 1 a la 1. Da error, pero se mueve (T17) y queda libre a los 6 s.
  - Llamada en la 3: queda pendiente.
  - Pasar el tiempo 6 veces 1 s. A los 6 s se atiende la llamada y falla el registro. El mensaje es `Error("Al avanzar el tiempo: no se ha podido acceder al histórico de movimientos: fallo simulado")`.
- **H. Tope de avance.** `pasar_el_tiempo(5 s)` deja el instante del simulador en 1 s. `pasar_el_tiempo(250 ms)` dos veces lo deja en 500 ms.
- **I. Fecha y hora.** Después de pasar 10 veces 1 s y una vez 500 ms, el texto es `Lunes 05/10/2026 08:00:10`.


## 7. Propuesta de diseño (orientativa)

El programador puede ajustar los nombres. Si lo hace, mantendrá el significado.

### Organización (arquitectura hexagonal)

```
src/
  adaptadores.rs                         añadir: pub mod interfaz_grafica;
  adaptadores/interfaz_grafica.rs        pub mod modelo_de_vista; pub mod presentador; pub mod vista_con_egui;
  adaptadores/interfaz_grafica/modelo_de_vista.rs   tipos de R3 (datos puros, sin egui)
  adaptadores/interfaz_grafica/presentador.rs       PresentadorDelSimulador<H> (R4, R5), sin egui
  adaptadores/interfaz_grafica/vista_con_egui.rs    InterfazGraficaConEgui<H>, ejecutar_interfaz_grafica (R6); único archivo que usa eframe
  dominio/errores.rs                     Display + Error (R1)
  dominio/historico_de_movimientos.rs    Display + Error (R1)
  dominio/fecha_y_hora.rs                anio(), mes(), dia(), minuto(), segundo() (R2)
  aplicacion/control_de_trafico.rs       Display + Error para ErrorDeControlDeTrafico (R1)
  main.rs                                raíz de composición (R7)
```

- **Patrón:** modelo-vista-presentador (MVP) de vista pasiva.
  - El **presentador** traduce el estado del control a un **modelo de vista**, y las **acciones del usuario** a órdenes al control. Es lógica pura y se prueba con tests unitarios.
  - La **vista** (egui) solo dibuja y recoge clics.
- **Hexagonal:**
  - La interfaz entera (presentador y vista) es un **adaptador de entrada** que llama al caso de uso `ControlDeTrafico`.
  - El dominio y la aplicación no saben nada de la interfaz.
  - `eframe` queda confinado en un solo archivo.
- **SOLID:**
  - Responsabilidad única: el presentador decide qué se muestra y la vista decide cómo.
  - La vista depende de abstracciones de datos (`ModeloDeVista` y `AccionDelUsuario`), no del control.
  - Sobre la inversión de dependencias hacia el control, ver U13.

### Firmas sugeridas

```rust
// adaptadores/interfaz_grafica/presentador.rs
pub const AVANCE_MAXIMO_DE_LA_SIMULACION_POR_FOTOGRAMA: Duration = Duration::from_secs(1);

pub struct PresentadorDelSimulador<H: HistoricoDeMovimientos> {
    control: ControlDeTrafico<H>,
    mensaje_para_el_usuario: Option<MensajeParaElUsuario>,
}
impl<H: HistoricoDeMovimientos> PresentadorDelSimulador<H> {
    pub fn nuevo(control: ControlDeTrafico<H>) -> Self;
    pub fn control(&self) -> &ControlDeTrafico<H>;
    pub fn modelo_de_vista(&self) -> ModeloDeVista;
    pub fn atender(&mut self, accion: AccionDelUsuario);
    pub fn pasar_el_tiempo(&mut self, tiempo_real_transcurrido: Duration);
}
// Funciones auxiliares puras (privadas; se prueban desde el módulo de tests):
fn nombre_del_dia_de_la_semana(dia: DiaDeLaSemana) -> &'static str;
fn texto_de_la_fecha_y_hora(fecha_y_hora: FechaYHora) -> String;

// adaptadores/interfaz_grafica/vista_con_egui.rs
const INTERVALO_DE_REPINTADO: Duration = Duration::from_millis(100);
pub struct InterfazGraficaConEgui<H: HistoricoDeMovimientos> { /* presentador, instante_real_del_fotograma_anterior: Instant */ }
impl<H: HistoricoDeMovimientos> eframe::App for InterfazGraficaConEgui<H> { fn ui(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame) { ... } }
pub fn ejecutar_interfaz_grafica<H: HistoricoDeMovimientos + 'static>(presentador: PresentadorDelSimulador<H>) -> Result<(), eframe::Error>;
```

Conviene que la parte de dibujo sea una función `fn dibujar(ui: &mut egui::Ui, modelo: &ModeloDeVista) -> Vec<AccionDelUsuario>`, sin acceso al presentador. Así se ve que no contiene lógica.

### Indicaciones para los tests

- Los tests van en módulos `#[cfg(test)] mod tests` dentro de cada archivo, como hasta ahora. **No se modifica ningún test existente**: solo se añaden tests nuevos, también en los módulos de tests que ya existen.
- Los tests del presentador usan el control real con `HistoricoDeMovimientosEnMemoria`. Lo registrado se comprueba con `presentador.control().historico().todos_los_movimientos()`, y las posiciones con `presentador.control().simulador()` o con el modelo de vista.
- Para los fallos del histórico, conviene un doble de test en el módulo de tests del presentador (por ejemplo `HistoricoQueFallaAlRegistrar`), que devuelva `AccesoAlAlmacenamiento { detalle: "fallo simulado" }`. El doble del control está en su propio módulo de tests y no se puede reutilizar.
- Para un edificio de 1 ascensor: `ConfiguracionDelEdificio { numero_de_ascensores: 1, ..ConfiguracionDelEdificio::estandar() }`.
- **Cuidado con el reposicionamiento (T14).** Las llamadas que hace el propio test cuentan como demanda del mismo día y de la misma franja horaria. Por eso, un test que deje pasar 30 s o más con ascensores parados puede ver reposicionamientos. Los ejemplos del apartado 6 no llegan a 30 s. Si alguno lo necesitara, se puede usar un tiempo de reposo grande en `ConfiguracionDelControlDeTrafico`.
- Los tests de `nombre_del_dia_de_la_semana` y `texto_de_la_fecha_y_hora` pueden llamar directamente a esas funciones privadas desde el módulo de tests.

### Orden de implementación sugerido

1. `Display` y `Error` para los errores (R1).
2. Componentes de `FechaYHora` (R2).
3. Tipos del modelo de vista y `PresentadorDelSimulador::modelo_de_vista` (R3, R4).
4. `atender` y los mensajes (R5.1).
5. `pasar_el_tiempo` (R5.2).
6. Añadir `eframe` a `Cargo.toml` y ejecutar **`cargo build --offline`** la primera vez (apartado 2). Después, `vista_con_egui.rs` y `main.rs` (R6, R7). Los pasos 1 a 5 no necesitan eframe.
7. El glosario.

### Glosario

Cuando la implementación esté hecha, añadir a `src/glosario_de_dominio.md`, en una sección "Para la interfaz de usuario":
- **hueco** (de un ascensor): en la interfaz, la columna que representa el recorrido vertical de un ascensor, con una celda por planta.
- **marcha** (de un ascensor): parado, subiendo, bajando o llegando. Está **llegando** cuando ya está en su planta de destino, pero aún no ha terminado la parada.
- **botón encendido**:
  - un botón de llamada cuya pulsación no tendría efecto, porque ya hay una llamada en curso para la planta (pendiente, o con un ascensor desplazándose hacia ella);
  - en la botonera, el botón de la planta de destino del ascensor mientras se desplaza.
- **mensaje para el usuario**: el resultado de la última acción del usuario, o el último error.


## 8. Verificación manual de la capa de dibujo (la hace el usuario, después de implementar)

Se ejecuta `cargo run` y se comprueba lo siguiente:

1. Se abre una ventana "Simulador de ascensores", y la fecha y hora avanza segundo a segundo.
2. Hay una fila por planta, con la 7 arriba y la -2 abajo, y un hueco por ascensor. Los tres ascensores están en la planta 0 y aparecen como "Parado en la planta 0".
3. Al pulsar la llamada de la 4:
   - el botón se enciende y el ascensor 1 sube planta a planta (unos 2 s por planta);
   - aparece "Llegando a la planta 4" y, a los 12 s, queda parado;
   - el botón se apaga, y el mensaje indica qué ascensor atendió la llamada.
4. Al pulsar la -2 en la botonera del ascensor 2, baja. Mientras se desplaza, su botonera está deshabilitada y la -2 encendida.
5. Con los tres ascensores ocupados, una llamada aparece en "Llamadas pendientes" y se atiende en cuanto queda libre uno.
6. Al pulsar de nuevo una llamada encendida, el mensaje es "ya estaba en curso".
7. Si se minimiza la ventana unos segundos y se vuelve, la simulación no ha dado un salto (U4).
8. Las tildes y los símbolos se ven bien: no aparecen recuadros vacíos.
9. Si el histórico tiene llamadas de otras ejecuciones del mismo día de la semana y de la misma franja horaria, los ascensores en reposo se reposicionan solos a los 30 s.
10. Al cerrar la ventana, termina el programa. `datos/historico_de_movimientos.csv` contiene los movimientos de la sesión.


## 9. Fuera del alcance de esta funcionalidad

- Pausa y control de la velocidad de la simulación (U3). Se propone como idea en `0_funcionalidades_y_tareas_pendientes/`.
- Animación del movimiento entre plantas (U6). Necesita una posición fraccionaria en el simulador; se propone como idea en `0_funcionalidades_y_tareas_pendientes/`.
- Mostrar el motivo de cada desplazamiento (llamada, botonera o reposicionamiento). El control no lo expone.
- Consultar el histórico o la demanda desde la interfaz, y configurar el edificio o el archivo del histórico desde la interfaz.
- Botones de subir y bajar en cada planta (T2), y cola de destinos por ascensor (S6). La botonera no admite varias plantas seguidas.
- Tests automáticos de la capa de dibujo (U14).
- Guardar el tamaño y la posición de la ventana (feature `persistence`), accesibilidad (`accesskit`) e internacionalización.
- Los tests que faltan en el control de tráfico (U17).
