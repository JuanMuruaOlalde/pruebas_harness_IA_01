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
