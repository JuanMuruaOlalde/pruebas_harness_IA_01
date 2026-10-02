# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Propósito del proyecto

Es una prueba para experimentar y practicar programación en un entorno agéntico.

Vamos a implementar un pequeño simulador de ascensores.


## Comandos habituales

```bash

cargo fmt
cargo clippy
cargo build
cargo run
cargo test

```

- `cargo clippy --all-targets`: revisa también los tests. El bucle automático exige que no haya ningún warning.
- `cargo test <parte_del_nombre>`: ejecuta solo los tests cuyo nombre la contiene. Por ejemplo, `cargo test dominio::edificio::` ejecuta solo los tests de ese módulo.


## Arquitectura

Allá donde sea conveniente:
- Implementar una arquitectura Hexagonal (Ports&Adapters).
- Seguir los principios SOLID.
- Seguir la metodología DDD (Domain Driven Design). El glosario de dominio está en el archivo `src/glosario_de_dominio.md`.

### Mapa de la arquitectura actual





## Flujo de trabajo

Las funcionalidades avanzan por las carpetas de `trabajo/`:

- `0_funcionalidades_y_tareas_pendientes/`: ideas y tareas pendientes. Los agentes añaden aquí lo que detectan fuera de su encargo.
- `1_listo_para_implementar/`: la cola de funcionalidades para el bucle automático.
- `2_en_curso/`: la funcionalidad en curso de implementación. Si aparece un `PROBLEMA_*.md`, el bucle se detiene.
- `3_historico/`: las funcionalidades terminadas y los informes `realizado_AAAAMMDDTHHMMSS.md` de cada ejecución del bucle.
- `directrices/`: las reglas que `validar_codigo_y_commitearlo` comprueba antes de cada commit.

La skill `/programacion_en_bucle_automatico` solo se lanza a mano. La coordina la conversación principal, porque los subagentes no pueden lanzar otros subagentes (`CLAUDE_CODE_MAX_SUBAGENT_SPAWN_DEPTH=1`). Pasos de cada vuelta:

1. `evaluar_y_preparar_trabajo`
2. El usuario confirma la funcionalidad elegida y su lista de tests.
3. `programar_codigo`
4. `revisar_codigo_y_refactorizar`
5. `cargo fmt`
6. `validar_codigo_y_commitearlo`

En `.claude/settings.json`, `git commit` pide confirmación y `git push` está prohibido.

Fuera del bucle hay dos agentes de análisis, `analizar_codigo_y_determinar_estructura` y `buscar_costuras_y_extraer_interfaces`, que escriben sus informes en la carpeta `trabajo/analisis/`.


## Límites a respetar

No modificar la carpeta `trabajo/directrices/`. Sí leer su contenido.

No modificar la carpeta `documentacion/`. No leer su contenido; excepto cuando sea necesario para hacer un commit.

No modificar la carpeta `trabajo/0_funcionalidades_y_tareas_pendientes`; excepto para añadir nuevos archivos.

Ignorar totalmente la carpeta `zz - trozos de codigo descartados - guardados por si acaso`. No modificar ni leer su contenido.


## Estilo al escribir código

Dedicar atención a la nomenclatura. Poner nombres descriptivos, aunque resulten largos. El código se ha de poder leer con facilidad y quedando claro qué representa cada variable, función, estructura,...

Todo va en español: identificadores, nombres de tests, comentarios y mensajes. En los identificadores se escribe sin tildes ni eñes (`anio`, `Miercoles`). Los términos del dominio son los del glosario; para usar uno que no esté, hay que acordarlo antes.



