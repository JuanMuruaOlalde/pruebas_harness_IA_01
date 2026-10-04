# Arneses (harness) para guiar el trabajo de IAs agénticas

## Introducción general

En los últimos tiempos, los modelos de IA están llegando a niveles que permiten trabajar con ellos de forma colaborativa. Personas humanas y agentes IA están comenzando a trabajar "codo con codo", formando un "equipo" de trabajo.

Al igual que las personas, los agentes IA también necesitan tener unas **directrices claras** cuando trabajan en un equipo. Para evitar incertidumbres y malentendidos.

En este repositorio se pretende explorar cómo se pueden explicitar esas directrices en este momento (*septiembre de 2026*). Las ideas recogidas son más o menos válidas para trabajar con cualquier proveedor de IA; pero, concretamente, nos hemos centrado en los entornos de trabajo de [Anthropic](https://www.anthropic.com/):
- tanto en su vertiente de **sesiones interactivas de charla directa con el modelo IA** ([Claude](https://claude.com/docs)),
- como en su vertiente de **agentes IA para desarrollo de software** ([Claude Code](https://code.claude.com/docs/es/overview)).

El uso de **agentes IA de escritorio** está todavía en sus primeros pasos. Hay iniciativas tales como, por ejemplo: Claude Desktop, [Claude Cowork](https://claude.com/docs/cowork/overview), [Claude in Slack](https://claude.com/docs/claude-tag/overview), [Claude in Chrome](https://claude.com/claude-in-chrome) o [Claude for M365](https://claude.com/docs/office-agents/overview). Pero aún se están explorando formas de explicitar directrices y salvaguardas cuando se trabaja en esos entornos agénticos. (En este repositorio, excepto en este párrafo, no se trata nada sobre ellos; nos centramos solo en Claude Code.)

> En el campo de los agentes de escritorio, por ahora, prácticamente solo tenemos los *mecanismos base*:
> - Las directrices generales (`System Prompts`) que se expliciten para el modelo:
>   - Tanto las que pone de serie el propio proveedor del modelo.
>   - Como las que configuremos nosotros para nuestra cuenta. 
> - Las directrices marcadas a cada tipo de agente que se defina.
> - El cuidado que pongamos las personas humanas en los `prompts` que escribamos en cada sesión.

Por otro lado, merece la pena también citar otro tipo de uso que está surgiendo. La eficacia de los agentes IA hace que resulte prácticamente inviable hacer que todos sus resultados sean verificados detenidamente por parte de personas humanas. Y eso está llevando a la necesidad de disponer de otros **agentes IA que nos ayuden a verificar** el trabajo de los agentes IA que hacen el trabajo. 


## Entrando en harina

En este repositorio hay:

- Unos apuntes generales detallando cómo explicitar directrices y definir agentes:  [`./documentacion/configuracion_de_directrices_para_Claude.md`](./documentacion/configuracion_de_directrices_para_Claude.md)

- Una muestra de definición de agentes: [`.claude/agents`](./.claude/agents). 

- Una muestra de utilización de agentes dentro de un bucle de trabajo automatizado: [`.claude/skills/programacion_en_bucle_automatico/SKILL.md`](.claude/skills/programacion_en_bucle_automatico/SKILL.md).

- Una muestra de salvaguardas y reglas fijas de obligado cumplimiento: [`.claude/settings.json`](./.claude/settings.json)


## Comentario final

Todo esto de la IA está avanzando a velocidad vertiginosa. Y también su documentación evoluciona con rapidez. Enlaces concretos pueden variar, por ello solo cito aquí las entradas raíz a la documentación de Anthropic:
- Claude: [https://claude.com/docs](https://claude.com/docs)
- Claude Code: [https://code.claude.com/docs/es/overview](https://code.claude.com/docs/es/overview)
