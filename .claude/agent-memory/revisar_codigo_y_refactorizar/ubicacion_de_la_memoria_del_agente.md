---
name: ubicacion-de-la-memoria-del-agente
description: La ruta de memoria de este agente depende del cwd de lanzamiento; hay copias en .claude/agent-memory de la raiz y en trabajo/2_en_curso/.claude
metadata:
  type: reference
---

La memoria de este agente esta en `.claude/agent-memory/revisar_codigo_y_refactorizar/`, relativa al cwd con el que se lance.
- Lanzado desde la raiz del proyecto: `<raiz>/.claude/agent-memory/revisar_codigo_y_refactorizar/`, donde esta la memoria original.
- El 2026-10-05 el bucle lo lanzo con cwd `trabajo/2_en_curso`, y la ruta indicada fue `trabajo/2_en_curso/.claude/agent-memory/...`, que estaba vacia.

**How to apply:** al empezar, si la memoria indicada esta vacia, leer tambien la de la raiz. Al terminar, escribir en la indicada y copiar los cambios a la de la raiz. Es probable que el contenido de `2_en_curso` se archive en `3_historico`.
