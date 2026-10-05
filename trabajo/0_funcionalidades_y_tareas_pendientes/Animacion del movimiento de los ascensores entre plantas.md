# Animación del movimiento de los ascensores entre plantas

Origen: al preparar la funcionalidad "Interfaz de usuario para manejar el simulador" (supuesto U6).

## Situación

El simulador solo da la **planta actual** de cada ascensor: la última que ha alcanzado, un número entero. En la interfaz gráfica, el ascensor salta de una celda a la siguiente cada 2 s, en lugar de deslizarse por el hueco.

## Propuesta

Que el simulador ofrezca, además, una **posición fraccionaria** del ascensor. Por ejemplo, `altura_del_ascensor(identificador) -> f64` en plantas: 2,5 significaría a medio camino entre la 2 y la 3. Se calcularía con el mismo modelo temporal del desplazamiento (R6 de la primera funcionalidad), interpolando dentro del tramo entre dos plantas. Durante el arranque y la parada, el ascensor seguiría en la planta de origen o de destino.

Con ella, la vista podría dibujar el hueco como un rectángulo vertical y el ascensor como una caja que se desplaza con suavidad. Habría que repintar más a menudo (unos 30–60 fotogramas por segundo).

## A tener en cuenta

- Amplía la API pública del simulador, la raíz del agregado. Habría que añadir tests para el cálculo de la posición fraccionaria.
- Conviene decidirlo junto con las ideas "Cola de destinos pendientes por ascensor" y "El simulador podría decir cuándo queda libre cada ascensor", que también tocan el modelo de movimiento.
