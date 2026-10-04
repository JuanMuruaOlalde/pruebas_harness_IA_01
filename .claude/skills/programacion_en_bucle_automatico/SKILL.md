---
description: Lanza un bucle agéntico para implementar las funcionalidades o realizar las tareas indicadas en la carpeta `trabajo/1_listo_para_implementar`
disable-model-invocation: true
---

## Estado de `trabajo/` al invocar la skill

Contenido de la carpeta `trabajo/1_listo_para_implementar/`:
!`ls "${CLAUDE_PROJECT_DIR}/trabajo/1_listo_para_implementar/" | grep . || echo "(vacía)" 2>/dev/null`

Contenido de la carpeta `trabajo/2_en_curso/`:
!`ls "${CLAUDE_PROJECT_DIR}/trabajo/2_en_curso/" | grep . || echo "(vacía)" 2>/dev/null`

## Instrucciones

Coordinas el bucle de trabajo. Los subagentes no se lanzan entre sí: cada uno hace su parte y te devuelve un informe, y tú lanzas el siguiente. Ninguno ve esta conversación, así que en cada encargo incluye el contexto que necesita: qué funcionalidad es y qué hizo el paso anterior.

No hagas tú el trabajo de los subagentes ni corrijas sus problemas.

### Antes de empezar

- Si `trabajo/1_listo_para_implementar/` está vacía, avisa y termina.

- Si `trabajo/2_en_curso/` no está vacía, no lances ningún subagente: resume lo que contiene (sobre todo si hay un `PROBLEMA_*.md`), avisa y termina.

### Una vuelta del bucle

1. Usa el subagente `evaluar_y_preparar_trabajo`.
2. Muestra al usuario la funcionalidad escogida y el contenido de `trabajo/2_en_curso/lista_de_tests.md`, y pregúntale si quiere continuar. Si no lo confirma, detente y recuérdale que la funcionalidad sigue en `trabajo/2_en_curso/` con su lista de tests, para que la ajuste o la devuelva a `trabajo/1_listo_para_implementar/`.
3. Usa el subagente `programar_codigo`.
4. Usa el subagente `revisar_codigo_y_refactorizar`. Pásale el resumen de lo que implementó `programar_codigo`.
5. Ejecuta `cargo fmt`.
6. Usa el subagente `validar_codigo_y_commitearlo`. Pásale el resumen de los pasos 3 y 4.

Después de cada paso, muestra al usuario lo ocurrido;  si en la carpeta `trabajo/2_en_curso/` aparece algún archivo `PROBLEMA_*.md` o el informe del subagente ha indicado un problema, detente.

Si la validación del paso 6 termina bien y quedan archivos en `trabajo/1_listo_para_implementar/`, empieza otra vuelta desde el paso 1.

### Al terminar

Tanto si vacías la cola como si te detienes por algún problema, escribe un archivo `trabajo/3_historico/realizado_AAAAMMDDTHHMMSS.md` (donde AAAAMMDDTHHMMSS es un timestamp en formato ISO con el año, mes, dia, hora, minuto y segundo); en ese archivo detalla, por cada vuelta completada: 
- qué se implementó, 
- las signaturas de los tests añadidos, 
- el hash y el mensaje del commit.
En caso de haber tenido algún problema, incluye descripción del mismo al final del archivo.