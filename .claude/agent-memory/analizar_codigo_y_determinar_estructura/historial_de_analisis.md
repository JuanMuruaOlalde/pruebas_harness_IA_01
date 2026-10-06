---
name: historial-de-analisis
description: Informes de vista general ya escritos (fecha, commit, estado y hallazgos clave) para comparar la evolucion en analisis futuros
metadata:
  type: project
---

## 2026-10-06 (commit 60344c8, rama pruebas_con_el_bucle_automatico)

Informe: `trabajo/analisis/vista_general_del_codigo-20261006T212130-.md`

Estado: 171 tests en verde, 0 warnings de clippy, fmt limpio; 42 avisos con clippy::pedantic (sobre todo must_use y `# Errors`). ~2.000 lineas de produccion y ~2.850 de tests.

Hallazgos principales que vigilar en el proximo analisis (¿se han corregido?):
1. No hay puerto de entrada: el presentador depende del `ControlDeTrafico<H>` concreto y accede a `control().simulador()`; el genérico `H` llega hasta la vista egui.
2. La regla "llamada en curso" esta duplicada en el presentador (`boton_de_llamada_encendido`); la botonera no devuelve un resultado explicito.
3. El reposicionamiento consulta y copia el historico en cada fotograma mientras haya un ascensor en reposo: O(n) creciente.
4. El control depende del tamano del paso (las llamadas pendientes se atienden al final del paso).
5. La demanda incluye las llamadas de la sesion en curso.
6. `LineaIlegible` (detalle de archivo) en el error del puerto del dominio.
7. El doble `HistoricoQueFallaAlRegistrar` esta duplicado; los tests de aplicacion importan un adaptador.
8. La seccion "Mapa de la arquitectura actual" de CLAUDE.md esta vacia; la tarea pendiente "Mensajes legibles..." ya esta resuelta pero sigue en la carpeta de pendientes.

**Why:** permite que el proximo analisis informe de la evolucion en lugar de repetir lo mismo.
**How to apply:** al empezar, comprobar cada punto en el codigo actual antes de repetirlo. Relacionado: [[como-analizar-este-repositorio]].
