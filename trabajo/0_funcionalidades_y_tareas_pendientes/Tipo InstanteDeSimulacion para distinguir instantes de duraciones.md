# Tipo `InstanteDeSimulacion` para distinguir instantes de duraciones

Origen: revisión del código de la funcionalidad "Estructura básica del edificio y ascensores".

## Situación

El simulador usa `std::time::Duration` para dos conceptos distintos del glosario:
- el **instante de simulación** (`SimuladorDeEdificio::instante_actual()`, el `instante_de_inicio` de un desplazamiento);
- las **duraciones** (`avanzar_tiempo(duracion)`, el tiempo de desplazamiento y el de arranque y de parada).

El compilador no impide mezclarlos; por ejemplo, pasar un instante donde se espera una duración.

## Propuesta

Crear un tipo nuevo `InstanteDeSimulacion(Duration)` (tiempo transcurrido desde el inicio de la simulación) con operaciones como:
- `InstanteDeSimulacion::INICIO`;
- `avanzar(duracion) -> InstanteDeSimulacion`;
- `tiempo_transcurrido_desde(otro_instante) -> Duration`.

Y usarlo en `SimuladorDeEdificio::instante_actual()` y en el estado interno del ascensor.

## Por qué no se ha hecho en la refactorización

Cambia la API pública del simulador, y habría que modificar los tests que comparan `instante_actual()` con un `Duration`. Conviene decidirlo antes de que la interfaz gráfica y el control de tráfico (que necesitará fecha y hora) empiecen a usar el instante.
