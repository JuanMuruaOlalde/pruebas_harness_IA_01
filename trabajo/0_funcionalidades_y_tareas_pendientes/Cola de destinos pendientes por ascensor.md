# Cola de destinos pendientes por ascensor

Origen: al preparar la funcionalidad "Estructura básica del edificio y ascensores".

## Situación

En la primera versión del modelo de simulación, un ascensor solo atiende un destino cada vez. Si recibe una orden mientras se está desplazando, la rechaza con el error "ascensor ocupado" (supuesto S6 de aquella funcionalidad).

## Propuesta

Que cada ascensor guarde una cola de plantas de destino pendientes:
- Si el ascensor está ocupado, las órdenes nuevas se encolan en lugar de rechazarse.
- Al acabar un desplazamiento, el ascensor empieza automáticamente el siguiente de la cola.
- No se encola una planta que ya está en la cola, ni la planta de destino del desplazamiento en curso.
- Más adelante, se podría reordenar la cola para atender primero las plantas que quedan en el sentido de la marcha (algoritmo de ascensor clásico).

## Por qué conviene

- La botonera de la interfaz gráfica permite pulsar varios botones seguidos.
- El control de tráfico necesitará asignar llamadas a ascensores que todavía no están libres.
