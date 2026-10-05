# El simulador podría decir cuándo queda libre cada ascensor

Origen: revisión del código de la funcionalidad "Un control de tráfico básico".

## Situación

Para el tiempo de reposo (T14, R6.8), `ControlDeTrafico` guarda, por cada ascensor, el instante desde el que está libre. Lo calcula el propio control en `anotar_cuando_quedara_libre`: instante actual + `duracion_de_un_desplazamiento(distancia)`, o el instante actual si la distancia es 0.

Ese cálculo repite lo que ya sabe el simulador (`Desplazamiento::posicion_en` en `src/dominio/ascensor.rs`): cuánto dura un desplazamiento y cuándo se queda parado el ascensor. Hoy coinciden, y R6.8 lo indica de forma explícita. Pero si cambia el modelo de movimiento del simulador (aceleración, paradas intermedias, cola de destinos...), el control calcularía un instante distinto sin que nada lo avise.

## Propuesta

Que el simulador sea la única fuente de esa información. Por ejemplo:
- `SimuladorDeEdificio::ordenar_mover_ascensor` podría devolver el instante en que el ascensor quedará parado en el destino (el instante actual si ya estaba en la planta);
- o `SimuladorDeEdificio::instante_en_que_queda_libre(identificador_de_ascensor)`.

Así, el control solo guardaría ese instante, sin calcularlo.

## Por qué no se ha hecho en la refactorización

Cambia la API pública del simulador, que es la raíz del agregado y tiene sus propios tests. Conviene decidirlo junto con la idea "Cola de destinos pendientes por ascensor", que también cambia cuándo queda libre un ascensor.
