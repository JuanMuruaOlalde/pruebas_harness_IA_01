# Robustez del archivo del histórico de movimientos

Origen: revisión del código de la funcionalidad "Un control de tráfico básico".

## Situación

`HistoricoDeMovimientosEnArchivo` (`src/adaptadores/historico_de_movimientos_en_archivo.rs`) tiene dos casos en los que pierde o estropea datos sin avisar. Eso va contra el espíritu del supuesto T18: "mejor detectar un archivo dañado que ignorar datos sin avisar".

1. **La primera línea se salta sin comprobar que sea la cabecera.** Si el archivo no tiene cabecera (por ejemplo, porque alguien lo ha creado o editado a mano), el primer movimiento se pierde sin ningún error.
2. **Si el archivo no termina en salto de línea** (por ejemplo, porque se cortó una escritura anterior), `registrar` pega la línea nueva al final de la última. La línea resultante es ilegible y, en la siguiente ejecución, `abrir` da error `LineaIlegible`. El fallo aparece lejos de donde se produjo.

## Propuesta

1. Al abrir, comprobar que la primera línea es exactamente la cabecera. Si no lo es, dar `LineaIlegible { numero_de_linea: 1 }`, o un error nuevo `CabeceraIncorrecta`.
2. Al registrar, si el archivo no está vacío y no termina en `\n`, escribir antes un salto de línea. Otra opción es dar un error de almacenamiento.

Las dos cambian el comportamiento observable, así que necesitan tests nuevos.
