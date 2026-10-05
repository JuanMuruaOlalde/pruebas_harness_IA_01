# Mensajes legibles para los errores del dominio

Origen: revisión del código de la funcionalidad "Estructura básica del edificio y ascensores".

## Situación

`ErrorDeConfiguracion` y `ErrorDeSimulacion` (en `src/dominio/errores.rs`) solo implementan `Debug`. No tienen un texto pensado para el usuario, ni implementan `std::error::Error`.

## Propuesta

Implementar `std::fmt::Display` (con mensajes en español) y `std::error::Error` para ambos tipos de error. Por ejemplo:
- `AscensorOcupado`: "El ascensor se está desplazando y no acepta nuevas órdenes".
- `PlantaInexistente`: "La planta indicada no existe en el edificio".

## Por qué conviene

- La interfaz gráfica (funcionalidad en `1_listo_para_implementar/`) tendrá que informar al usuario cuando se rechace una orden de la botonera.
- Implementar `std::error::Error` permite usar `?` con `Box<dyn Error>` en la raíz de composición (`main.rs`).
