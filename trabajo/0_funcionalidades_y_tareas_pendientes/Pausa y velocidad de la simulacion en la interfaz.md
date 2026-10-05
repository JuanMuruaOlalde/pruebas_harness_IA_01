# Pausa y velocidad de la simulación en la interfaz

Origen: al preparar la funcionalidad "Interfaz de usuario para manejar el simulador" (supuesto U3).

## Situación

En la primera versión de la interfaz gráfica, el tiempo simulado avanza al ritmo del tiempo real (factor 1), sin pausa. Algunos comportamientos del control de tráfico tardan en verse. Por ejemplo, el reposicionamiento de los ascensores en reposo necesita 30 s de reposo, y los cambios de franja horaria, una hora entera.

## Propuesta

Añadir a la interfaz:
- un botón de **pausa / continuar**: mientras está en pausa, `pasar_el_tiempo` no avanza el control;
- un selector de **velocidad de la simulación** (por ejemplo x1, x2, x5 y x10), que multiplique el tiempo real transcurrido antes de avanzar el control.

Hay que decidir cómo se combina con el tope de avance por fotograma (1 s, supuesto U4). Por ejemplo, aplicar el tope al tiempo real antes de multiplicar.

Las dos cosas son lógica del presentador (`PresentadorDelSimulador`), así que se pueden probar sin egui. La vista solo añade los controles.
