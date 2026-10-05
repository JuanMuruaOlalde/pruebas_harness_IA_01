# Estructura básica del edificio y ascensores: requisitos y casos de uso

Funcionalidad de origen: `trabajo/2_en_curso/Estructura basica del edificio y ascensores.md`.

Es la primera funcionalidad del simulador. Las otras dos de la cola (interfaz de usuario con egui y control de tráfico básico) se construyen sobre el modelo de simulación que se implementa aquí.

Se pide implementar el **modelo de simulación** de un edificio con 10 plantas y 3 ascensores, que permita:
- consultar la posición de cada ascensor en un momento dado;
- mover cada ascensor, de forma independiente, a la planta que se le indique.


## 1. Supuestos adoptados (pendientes de confirmar por el usuario)

| # | Supuesto | Motivo | Alternativa |
|---|----------|--------|-------------|
| S1 | Las plantas del edificio estándar van de la **-2 a la 7**, ambas incluidas (10 plantas). La planta 0 (principal) cuenta como una de las 8 "por encima del suelo". | El enunciado dice "Total 10 plantas" y el glosario fija la 0 como planta principal. Solo así cuadran las dos cosas. | Plantas de la -2 a la 8 (serían 11). |
| S2 | La simulación lleva un **tiempo simulado** propio, que solo avanza cuando se pide de forma explícita. No usa el reloj real. | "Posición en un momento dado" y "modelo de simulación" implican que los desplazamientos llevan tiempo. Además, el glosario ya define el tiempo de desplazamiento y el de arranque y de parada. El tiempo explícito hace que los tests sean deterministas. | Movimientos instantáneos, sin tiempo. |
| S3 | En el edificio estándar, el **tiempo de desplazamiento** es de **2 s** y el **tiempo de arranque y de parada** es de **4 s**. Ambos se pueden configurar. | El enunciado no da valores. | Otros valores. |
| S4 | **La mitad** del tiempo de arranque y de parada se consume al principio del desplazamiento (arranque) y **la otra mitad** al final (parada). | Respeta el sentido del término del glosario. | Consumirlo entero al principio o entero al final. |
| S5 | Al empezar la simulación, todos los ascensores están **parados en la planta 0**. | Es la planta principal, la de acceso al edificio. | Otra planta inicial. |
| S6 | Si se ordena moverse a un ascensor que está en pleno desplazamiento, la orden **se rechaza** con el error "ascensor ocupado". | Es el modelo más simple. Una cola de destinos o el cambio de rumbo quedan para más adelante (ver `trabajo/0_funcionalidades_y_tareas_pendientes/`). | Encolar el nuevo destino, o cambiar el destino sobre la marcha. |
| S7 | Si se ordena a un ascensor parado ir a la planta en la que ya está, **no pasa nada**: el ascensor sigue parado y no se consume tiempo. La orden se acepta (`Ok`). | Es el comportamiento más simple. | Que cueste el tiempo de arranque y de parada (por ejemplo, para abrir puertas). |


## 2. Requisitos funcionales

### R1. Plantas
- R1.1. Cada planta se identifica con un número entero (identificador de planta), según el glosario: la 0 es la principal, los positivos están por encima y los negativos por debajo.
- R1.2. La distancia entre dos plantas es el valor absoluto de la diferencia entre sus identificadores. Es simétrica, y entre una planta y ella misma vale 0.
- R1.3. Las plantas de un edificio forman un rango continuo `[planta_mas_baja, planta_mas_alta]`. En el edificio estándar ese rango es `[-2, 7]` (10 plantas).

### R2. Ascensores
- R2.1. Cada ascensor se identifica con un número entero positivo (identificador de ascensor). Se numeran desde 1, y el 0 no es válido.
- R2.2. El edificio estándar tiene 3 ascensores (1, 2 y 3), y todos pueden ir a todas las plantas.

### R3. Configuración del edificio
- R3.1. La configuración del edificio reúne: planta más baja, planta más alta, número de ascensores, tiempo de desplazamiento y tiempo de arranque y de parada.
- R3.2. Hay una configuración estándar: plantas de la -2 a la 7, 3 ascensores, desplazamiento de 2 s, y arranque y parada de 4 s.
- R3.3. Crear un simulador con una configuración inválida da un error de configuración. Una configuración es inválida si:
  - la planta más baja está por encima de la más alta;
  - la planta 0 (principal) no está dentro del rango de plantas, porque es donde empiezan los ascensores;
  - el número de ascensores es 0;
  - el tiempo de desplazamiento es 0, porque provocaría divisiones por cero al calcular la posición.
- R3.4. El tiempo de arranque y de parada sí puede ser 0.

### R4. Tiempo simulado
- R4.1. El simulador tiene un instante de simulación actual, que empieza en 0.
- R4.2. `avanzar_tiempo(duracion)` suma la duración al instante actual y actualiza el estado de todos los ascensores en consecuencia.
- R4.3. Se pueden avanzar fracciones de segundo, porque la interfaz gráfica avanzará el tiempo fotograma a fotograma. Avanzar 0 no cambia nada.
- R4.4. Avanzar el tiempo de una vez o en varios pasos que sumen lo mismo deja a los ascensores en la misma posición. Para garantizarlo, la posición se calcula a partir del instante en que empezó el desplazamiento, y no acumulando incrementos.

### R5. Orden de mover un ascensor
- R5.1. `ordenar_mover_ascensor(identificador_de_ascensor, planta_de_destino)` inicia, en el instante actual, el desplazamiento del ascensor hacia la planta de destino.
- R5.2. Errores, por orden de comprobación:
  1. El identificador no corresponde a ningún ascensor del edificio: `AscensorInexistente`.
  2. La planta de destino está fuera del rango del edificio: `PlantaInexistente`.
  3. El ascensor se está desplazando: `AscensorOcupado` (S6).
- R5.3. Una orden rechazada no cambia nada: ni la posición, ni el estado, ni el instante de ningún ascensor.
- R5.4. Si la planta de destino es la misma en la que está parado el ascensor, no pasa nada (S7).
- R5.5. Cada ascensor se mueve con independencia de los demás. Cada desplazamiento guarda su propio instante de inicio.

### R6. Modelo temporal de un desplazamiento
Sean:
- `td` el tiempo de desplazamiento y `tap` el tiempo de arranque y de parada;
- `t0` el instante en que se da la orden;
- `O` la planta de origen y `D` la de destino;
- `n = distancia(O, D) >= 1`.

Entonces:
- R6.1. Duración total del desplazamiento: `n * td + tap`.
- R6.2. Arranque, de `t0` a `t0 + tap/2` (sin incluir el final): el ascensor sigue en la planta `O`, pero ya se está desplazando.
- R6.3. Recorrido: alcanza la planta k-ésima del trayecto (k = 1..n) en el instante `t0 + tap/2 + k * td`. Entre dos plantas, la planta actual es la última que ha alcanzado. Funciona igual subiendo que bajando.
- R6.4. Parada, de `t0 + tap/2 + n * td` a `t0 + n * td + tap` (sin incluir el final): el ascensor está en la planta `D`, pero todavía se está desplazando (ocupado).
- R6.5. Desde `t0 + n * td + tap` en adelante, el ascensor queda parado en `D` y acepta nuevas órdenes.

Ejemplo de referencia (`td` = 2 s, `tap` = 4 s, orden en `t0` = 0 s de la planta 0 a la 3; duración total 3·2 + 4 = 10 s):

| Instante | Planta actual | Estado |
|----------|---------------|--------|
| 0 s | 0 | Desplazándose hacia 3 |
| 1 s | 0 | Desplazándose hacia 3 (arranque) |
| 3,9 s | 0 | Desplazándose hacia 3 |
| 4 s | 1 | Desplazándose hacia 3 |
| 6 s | 2 | Desplazándose hacia 3 |
| 8 s | 3 | Desplazándose hacia 3 (parada) |
| 9 s | 3 | Desplazándose hacia 3 (parada) |
| 10 s | 3 | Parado |

Bajando de la 0 a la -2 (duración total 2·2 + 4 = 8 s): planta 0 hasta los 4 s; planta -1 a los 4 s; planta -2 a los 6 s; parado en -2 a los 8 s.

### R7. Consulta de posición
- R7.1. `posicion_del_ascensor(identificador)` devuelve la posición en el instante actual: la planta actual y el estado (`Parado` o `Desplazandose { planta_de_destino }`).
- R7.2. Consultar un ascensor inexistente da el error `AscensorInexistente`.
- R7.3. `posiciones_de_todos_los_ascensores()` devuelve la posición de cada ascensor junto con su identificador, ordenadas por identificador. Hay una entrada por ascensor.
- R7.4. Consultar no cambia el estado del simulador.


## 3. Casos de uso

- **CU1. Iniciar la simulación.** Se crea un simulador con la configuración estándar o con otra personalizada. Resultado: instante 0 y todos los ascensores parados en la planta 0. Con una configuración inválida, se obtiene un error de configuración.
- **CU2. Consultar la posición de un ascensor.** Dado su identificador, se obtiene su planta actual y su estado. Si el ascensor no existe, se obtiene un error.
- **CU3. Consultar la posición de todos los ascensores.** Se obtiene la lista completa, ordenada por identificador. Es la que usará la interfaz gráfica para dibujar.
- **CU4. Ordenar a un ascensor que vaya a una planta.** El ascensor empieza a desplazarse. Si el ascensor no existe, la planta no existe o el ascensor está ocupado, se obtiene un error y no cambia nada.
- **CU5. Hacer avanzar el tiempo.** El instante actual avanza, y los ascensores en desplazamiento progresan o llegan a su destino según R6.
- **CU6. Desplazamientos simultáneos.** Varios ascensores se desplazan a la vez, cada uno con su propio origen, destino e instante de inicio, sin interferir entre sí.


## 4. Propuesta de diseño (orientativa)

El programador puede ajustar los nombres. Si lo hace, mantendrá el significado.

### Organización del crate
- Crear `src/lib.rs` con `pub mod dominio;`, y hacer que `src/main.rs` use el crate de biblioteca (`pruebas_harness_01`).
  - **Motivo:** en un crate solo binario, los elementos públicos que `main` no usa provocan avisos `dead_code`, y el bucle exige que `cargo clippy --all-targets` no dé ningún aviso.
- Módulos de dominio sugeridos, en `src/dominio/`:
  - `planta.rs`: `IdentificadorDePlanta`, tipo nuevo sobre `i32`, con `distancia_a(&self, otra) -> u32`.
  - `ascensor.rs`: `IdentificadorDeAscensor`, tipo nuevo sobre `u32`; la entidad `Ascensor`; `EstadoDeAscensor`; `PosicionDeAscensor`.
  - `configuracion.rs`: `ConfiguracionDelEdificio`, con `estandar()`, `numero_de_plantas() -> u32` y `duracion_de_un_desplazamiento(distancia) -> Duration`.
  - `simulador.rs`: `SimuladorDeEdificio`, la raíz del agregado. Contiene la configuración, los ascensores y el instante actual.
  - `errores.rs`: `ErrorDeConfiguracion` y `ErrorDeSimulacion`.
- Los tests unitarios van en un módulo `#[cfg(test)] mod tests` dentro de cada archivo.

### Tipos sugeridos
- Para duraciones e instantes, `std::time::Duration`. El instante actual es el tiempo transcurrido desde el inicio de la simulación; se puede envolver en un tipo nuevo `InstanteDeSimulacion` si aporta claridad.
- No usar `f64` para calcular posiciones. Usar aritmética entera, por ejemplo sobre `Duration::as_nanos()`, o restas y divisiones de `Duration`. Así los resultados son exactos y los tests deterministas.
- `EstadoDeAscensor` (público): `Parado`, o `Desplazandose { planta_de_destino: IdentificadorDePlanta }`.
- `PosicionDeAscensor` (público): `{ planta_actual: IdentificadorDePlanta, estado: EstadoDeAscensor }`.
- Estado interno de `Ascensor`: `Parado { planta }`, o `Desplazandose { planta_de_origen, planta_de_destino, instante_de_inicio }`. A partir de él se calcula la posición en cualquier instante.

### Operaciones públicas sugeridas de `SimuladorDeEdificio`
```rust
pub fn nuevo(configuracion: ConfiguracionDelEdificio) -> Result<SimuladorDeEdificio, ErrorDeConfiguracion>
pub fn instante_actual(&self) -> Duration
pub fn avanzar_tiempo(&mut self, duracion: Duration)
pub fn ordenar_mover_ascensor(&mut self, identificador_de_ascensor: IdentificadorDeAscensor, planta_de_destino: IdentificadorDePlanta) -> Result<(), ErrorDeSimulacion>
pub fn posicion_del_ascensor(&self, identificador_de_ascensor: IdentificadorDeAscensor) -> Result<PosicionDeAscensor, ErrorDeSimulacion>
pub fn posiciones_de_todos_los_ascensores(&self) -> Vec<(IdentificadorDeAscensor, PosicionDeAscensor)>
pub fn configuracion(&self) -> &ConfiguracionDelEdificio
```

### Arquitectura
- Todo lo de esta funcionalidad pertenece al **dominio**. Aún no hay dependencias externas: ni persistencia, ni reloj real, ni interfaz. Por eso **no hace falta definir puertos (traits) todavía**, y definirlos ahora sería anticiparse.
- Más adelante harán falta puertos:
  - la interfaz egui necesitará un puerto de entrada (la fachada del simulador);
  - el control de tráfico necesitará un puerto de persistencia para el histórico, y un reloj con fecha y hora.
- `src/main.rs` hace de raíz de composición. Para esta funcionalidad basta con que cree el simulador estándar y muestre por consola la posición inicial de cada ascensor. Esto no lleva tests.

### Glosario
Cuando la implementación esté hecha, conviene añadir a `src/glosario_de_dominio.md` estos términos:
- **simulador**;
- **instante de simulación**;
- **desplazamiento** (orden de ir de una planta de origen a una de destino, que incluye el arranque y la parada);
- **planta de origen** y **planta de destino**;
- **planta actual**;
- **ascensor parado** y **ascensor desplazándose u ocupado**;
- **planta más baja** y **planta más alta**;
- **configuración del edificio**.


## 5. Fuera del alcance de esta funcionalidad

- La interfaz gráfica, las llamadas desde las plantas y las botoneras: corresponden a otra funcionalidad.
- El control de tráfico, el histórico de movimientos y la optimización: corresponden a otra funcionalidad.
- Varios destinos encolados por ascensor, o cambiar de destino durante un desplazamiento.
- Puertas, capacidad, personas dentro del ascensor y peso.
- Fecha y hora de calendario; por ahora solo hay tiempo transcurrido desde el inicio de la simulación.
- Posición fraccionaria entre plantas, por ejemplo para animar la interfaz. Si la interfaz la necesita, se añadirá entonces.
