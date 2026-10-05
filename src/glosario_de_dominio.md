# Glosario de dominio

Nomenclatura a emplear. Se irá ampliando a medida que surjan nuevos conceptos/términos.

## De uso general

- **edificio**.
- **ascensor**.
- **planta**. 
- **identificador de ascensor**: número que distingue a cada ascensor del edificio. Se numeran desde 1.
- **identificador de planta**: número que distingue a cada planta del edificio; 0 es la planta principal por donde habitualmente se accede al edificio; números positivos por encima de ella y números negativos por debajo de ella.
- **persona**: persona que utiliza un ascensor para desplazarse.

## Para el control de ascensores

- **distancia** (entre dos plantas): número de plantas que las separan, el valor absoluto de la diferencia entre sus identificadores.
- **tiempo de desplazamiento**: es el número de segundos que tarda un ascensor en pasar de una planta a la siguiente; se asume que es el mismo en ambos sentidos (subiendo o bajando) y entre cualesquiera plantas.
- **tiempo de arranque y de parada**: es el número de segundos extra a sumar cualquier desplazamiento de cualquier ascensor.

## Para el simulador

- **simulador**: modelo que reúne el edificio, sus ascensores y un tiempo simulado propio, que solo avanza cuando se pide.
- **configuración del edificio**: datos que definen el edificio: planta más baja, planta más alta, número de ascensores, tiempo de desplazamiento y tiempo de arranque y de parada.
- **planta más baja** y **planta más alta**: extremos del rango continuo de plantas del edificio. En el edificio estándar son la -2 y la 7.
- **instante de simulación**: tiempo transcurrido desde el inicio de la simulación, que empieza en 0.
- **desplazamiento**: orden a un ascensor de ir de una planta de origen a una de destino; incluye el arranque y la parada.
- **planta de origen** y **planta de destino**: plantas de salida y de llegada de un desplazamiento.
- **planta actual**: última planta que ha alcanzado un ascensor en un instante dado.
- **ascensor parado**: ascensor que no está realizando ningún desplazamiento y acepta órdenes.
- **ascensor desplazándose** (u **ocupado**): ascensor en pleno desplazamiento, desde el arranque hasta el final de la parada; rechaza nuevas órdenes.

## Para el control de tráfico

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

## Para la interfaz de usuario

- **hueco** (de un ascensor): en la interfaz, la columna que representa el recorrido vertical de un ascensor, con una celda por planta.
- **marcha** (de un ascensor): parado, subiendo, bajando o llegando. Está **llegando** cuando ya está en su planta de destino, pero aún no ha terminado la parada.
- **botón encendido**:
  - un botón de llamada cuya pulsación no tendría efecto, porque ya hay una llamada en curso para la planta (pendiente, o con un ascensor desplazándose hacia ella);
  - en la botonera, el botón de la planta de destino del ascensor mientras se desplaza.
- **mensaje para el usuario**: el resultado de la última acción del usuario, o el último error.
