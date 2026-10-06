---
name: como-analizar-este-repositorio
description: Procedimiento y trampas al analizar el simulador de ascensores (carpetas vedadas, permisos que bloquean find, carpeta trabajo/analisis, no ejecutar cargo run, pendientes ya registrados)
metadata:
  type: project
---

Al analizar este repositorio (simulador de ascensores en Rust, crate unico `pruebas_harness_01`):

- No leer `documentacion/` ni `zz - trozos de codigo descartados - guardados por si acaso/` (CLAUDE.md).
  **Why:** restriccion explicita del usuario; settings.json deniega `Read` sobre la carpeta `zz`.
  **How to apply:** un `find .` con `-prune` que nombre esa carpeta fue DENEGADO por permisos (2026-10-06). Listar con `ls` y `find src ...` en su lugar.
- `trabajo/analisis/` no existia la primera vez; se crea con `mkdir -p` (permitido en settings). El informe va en `trabajo/analisis/vista_general_del_codigo-AAAAMMDDTHHMMSS-.md` (con el guion final antes de `.md`).
- Nunca `cargo run`: abre la ventana egui y bloquea. `cargo build`, `cargo test`, `cargo clippy --all-targets` y `cargo fmt --check` si.
- Antes de proponer mejoras, leer `trabajo/0_funcionalidades_y_tareas_pendientes/` para no duplicar ideas ya registradas (solo se pueden anadir archivos, nunca modificar). Leer tambien `trabajo/3_historico/realizado_*.md` (incidencias del bucle).
- Las decisiones de arquitectura ya tomadas (y su porque) estan en la memoria del revisor: ver [[decisiones-deliberadas-que-no-son-defectos]].

Relacionado: [[historial-de-analisis]].
