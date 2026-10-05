# Display para los identificadores de planta y de ascensor

Origen: revisión del código de la funcionalidad "Interfaz de usuario para manejar el simulador".

## Situación

`IdentificadorDePlanta` e `IdentificadorDeAscensor` solo implementan `Debug`. Para mostrarlos, la interfaz accede a su número interno (`.0`) en muchos sitios:
- en el presentador (`src/adaptadores/interfaz_grafica/presentador.rs`): los mensajes para el usuario, la descripción del estado y el texto de las llamadas pendientes;
- en la vista (`src/adaptadores/interfaz_grafica/vista_con_egui.rs`): el número de la planta de cada fila, los botones de la botonera, los títulos de los ascensores y el texto de las celdas.

## Propuesta

Implementar `std::fmt::Display` para los dos identificadores, de modo que muestren solo su número (`4`, `-2`, `1`...). Después, sustituir en la interfaz los `.0` usados para mostrarlos por el propio identificador (por ejemplo, `format!("Ascensor {identificador_de_ascensor}")`).

## Por qué conviene

- El dominio decide cómo se muestra un identificador, en lugar de repetirlo en cada adaptador.
- Los formatos de los mensajes se leen mejor, sin variables intermedias como `numero_de_planta`.

No se ha hecho al refactorizar porque amplía la API pública del dominio. Los textos actuales no cambiarían.
