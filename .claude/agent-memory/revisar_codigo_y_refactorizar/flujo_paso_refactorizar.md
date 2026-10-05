---
name: flujo-paso-refactorizar
description: Restricciones de este agente como paso 4 del bucle automatico: lista_de_tests.md intocable, sin commit, cambios de API publica y huecos de tests a pendientes, comprobar tests con mutaciones (y limpiar target/ despues)
metadata:
  type: feedback
---

Como paso 4 del bucle (`/programacion_en_bucle_automatico`), los nombres de test de `trabajo/2_en_curso/lista_de_tests.md` deben coincidir exactamente con los tests reales: se pueden mover de modulo y reescribir su cuerpo, pero no renombrar, quitar ni anadir tests.
**Why:** el orquestador lo exige y `validar_codigo_y_commitearlo` contrasta la lista; anadir tests la desincroniza.
**How to apply:** al terminar, comparar la lista con `cargo test --lib -- --list`; no hacer commit (lo hace el paso 6); pasar `cargo fmt` antes de clippy. Cargo funciona con `--offline`.

Cambios que alteran la API publica (p. ej. tipos de retorno o metodos nuevos del simulador) no se hacen en este paso: se describen en un archivo nuevo de `trabajo/0_funcionalidades_y_tareas_pendientes/`. Restringir visibilidad de tipos internos si se considera refactorizacion valida.

El orquestador pide revisar que los tests comprueben lo que promete su nombre, porque el programador a veces escribe tests y codigo juntos (sin RED).
**How to apply:** copiar `Cargo.toml`, `Cargo.lock` y `src/` al scratchpad, aplicar mutaciones (implementacion incorrecta plausible) con un script y ver que tests las detectan. Si un test pasa con la mutacion, reforzarlo cambiando el escenario o las aserciones sin cambiar nombre ni significado. Los requisitos sin ningun test (todas las mutaciones pasan) van a pendientes como "Tests que faltan", porque aqui no se pueden anadir tests. Funciono bien en el control de trafico (2026-10-05): 4 tests reforzados y 2 huecos detectados.
Con eframe, compilar la copia en un `target/` propio cuesta ~1,5 min y ~1,5 GB. Se puede compartir el `target/` del proyecto (`CARGO_TARGET_DIR`), pero entonces cargo deja en el proyecto el binario de tests de la ultima mutacion y da un fallo falso. **How to apply:** tras las mutaciones, `cargo clean --offline -p pruebas_harness_01` antes de la verificacion final (recompila en segundos). En la interfaz (2026-10-05) se reforzaron 2 tests y hubo 1 mutante equivalente.

Relacionado: [[decisiones-arquitectura-dominio]], [[ubicacion-de-la-memoria-del-agente]]
